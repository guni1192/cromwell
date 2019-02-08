use std::ffi::CString;
use std::fs;

use nix::sched::{unshare, CloneFlags};
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{chdir, chroot, fork, ForkResult};
use nix::unistd::{execve, sethostname};

use super::image::Image;
use super::mounts;

pub struct Container {
    pub name: String,
    pub command: String,
    pub image: Image,
}

impl Container {
    pub fn new(name: String, command: String) -> Container {
        Container {
            name: name.clone(),
            command,
            image: Image::new(name.clone()),
        }
    }

    pub fn prepare(&mut self) {
        self.image.pull().expect("Failed to cromwell pull");

        println!("Started initialize Container!");
        let c_hosts = format!("{}/etc/hosts", self.image.path);
        let c_resolv = format!("{}/etc/resolv.conf", self.image.path);

        println!("[INFO] Copying /etc/hosts to {}", c_hosts);
        println!("[INFO] Copying /etc/resolv.conf {}", c_resolv);

        fs::copy("/etc/hosts", c_hosts).expect("Failed copy /etc/hosts: ");
        fs::copy("/etc/resolv.conf", c_resolv).expect("Failed copy /etc/resolv.conf: ");

        unshare(
            CloneFlags::CLONE_NEWPID
                | CloneFlags::CLONE_NEWUTS
                | CloneFlags::CLONE_NEWNS
                | CloneFlags::CLONE_NEWUSER,
        )
        .expect("Can not unshare(2).");

        chroot(self.image.path.as_str()).expect("chroot failed.");
        chdir("/").expect("cd / failed.");
    }

    pub fn run(&self) {
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                println!("[INFO] container pid: {}", child);

                match waitpid(child, None).expect("waitpid faild") {
                    WaitStatus::Exited(_, _) => {}
                    WaitStatus::Signaled(_, _, _) => {}
                    _ => eprintln!("Unexpected exit."),
                }
            }
            Ok(ForkResult::Child) => {
                sethostname(&self.name).expect("Could not set hostname");

                fs::create_dir_all("proc").unwrap_or_else(|why| {
                    eprintln!("{:?}", why.kind());
                });

                println!("[INFO] Mount procfs ... ");
                mounts::mount_proc().expect("mount procfs failed");

                let cmd = CString::new(self.command.clone()).unwrap();
                let default_shell = CString::new("/bin/sh").unwrap();
                let shell_opt = CString::new("-c").unwrap();
                let lang = CString::new("LC_ALL=C").unwrap();
                let path = CString::new("PATH=/bin/:/usr/bin/:/usr/local/bin:/sbin").unwrap();

                execve(
                    &default_shell,
                    &[default_shell.clone(), shell_opt, cmd],
                    &[lang, path],
                )
                .expect("execution faild.");
            }
            Err(_) => eprintln!("[ERROR] Fork failed"),
        }
    }

    pub fn delete(&self) -> std::io::Result<()> {
        fs::remove_dir_all(&self.image.path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_container() {
        let image_name = String::from("library/alpine:3.8");
        let command = "/bin/bash".to_string();
        let container = Container::new(image_name, command.clone());
        assert_eq!(container.command, command);
    }
}
