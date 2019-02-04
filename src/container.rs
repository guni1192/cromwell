use std::env;
use std::ffi::CString;
use std::fs;

use nix::sched::{unshare, CloneFlags};
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{chdir, chroot, fork, ForkResult};
use nix::unistd::{execve, sethostname, Uid};

use super::mounts;

pub struct Container {
    pub name: String,
    pub path: String,
    pub command: String,
    pub uid: Uid,
}

impl Container {
    pub fn new(name: String, command: String, uid: Uid) -> Container {
        let path = format!("{}/{}", get_containers_path().unwrap(), name.clone());

        Container {
            name: name.clone(),
            path,
            command,
            uid,
        }
    }

    pub fn prepare(&self) {
        println!("Started initialize Container!");
        let c_hosts = format!("{}/etc/hosts", self.path);
        let c_resolv = format!("{}/etc/resolv.conf", self.path);

        println!("Copying /etc/hosts to {}", c_hosts);
        println!("Copying /etc/resolv.conf {}", c_resolv);

        fs::copy("/etc/hosts", c_hosts).expect("Failed copy file: ");
        fs::copy("/etc/resolv.conf", c_resolv).expect("Failed copy file: ");

        unshare(
            CloneFlags::CLONE_NEWPID
                | CloneFlags::CLONE_NEWUTS
                | CloneFlags::CLONE_NEWNS
                | CloneFlags::CLONE_NEWUSER,
        )
        .expect("Can not unshare(2).");

        chroot(self.path.as_str()).expect("chroot failed.");
        chdir("/").expect("cd / failed.");
    }

    pub fn run(&self) {
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                println!("container pid: {}", child);

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

                println!("Mount procfs ... ");
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
            Err(_) => eprintln!("Fork failed"),
        }
    }

    pub fn delete(&self) -> std::io::Result<()> {
        fs::remove_dir_all(&self.path)
    }
}

pub fn get_containers_path() -> Result<String, env::VarError> {
    let ace_container_env = "ACE_CONTAINER_PATH";
    env::var(ace_container_env)
}

#[test]
fn test_get_container_path() {
    let ace_container_env = "ACE_CONTAINER_PATH";
    let ace_container_path = "/var/lib/ace-containers";
    env::set_var(ace_container_env, ace_container_path);

    assert_eq!(ace_container_path, get_containers_path().unwrap())
}
