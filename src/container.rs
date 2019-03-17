use std::ffi::CString;
use std::fs::{self, File};
use std::io::prelude::*;
use std::iter;
use std::path::Path;

use nix::sched::{unshare, CloneFlags};
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{chdir, chroot, daemon, fork, getgid, getpid, getuid, ForkResult, Gid, Uid};
use nix::unistd::{execve, sethostname};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use log::info;

use super::config::Config;
use super::image::Image;
use super::mounts;
use super::pids::Pidfile;

pub struct Container {
    id: String,
    command: String,
    image: Option<Image>,
    host_uid: Uid,
    host_gid: Gid,
    become_daemon: bool,
    config: Config,
}

impl Container {
    pub fn new(
        image_name: Option<&str>,
        command: &str,
        path: Option<&str>,
        become_daemon: bool,
    ) -> Container {
        let mut rng = thread_rng();

        if let Some(id) = path {
            return Container {
                id: id.to_string(),
                command: command.to_string(),
                image: None,
                host_uid: getuid(),
                host_gid: getgid(),
                become_daemon,
                config: Config::new(None),
            };
        }

        let id: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(16)
            .collect();

        Container {
            id,
            command: command.to_string(),
            image: Some(Image::new(image_name.unwrap())),
            host_uid: getuid(),
            host_gid: getgid(),
            become_daemon,
            config: Config::new(None),
        }
    }

    fn uid_map(&self) -> std::io::Result<()> {
        let mut uid_map_file = File::create("/proc/self/uid_map")?;
        let uid_map = format!("0 {} 1", self.host_uid);

        uid_map_file.write_all(uid_map.as_bytes())?;
        info!("[Host] wrote {} /proc/self/uid_map", uid_map);
        Ok(())
    }

    fn gid_map(&self) -> std::io::Result<()> {
        let mut setgroups_file = File::create("/proc/self/setgroups")?;
        setgroups_file.write_all(b"deny")?;

        let mut gid_map_file = File::create("/proc/self/gid_map")?;
        info!("[Host] open(2) /proc/self/gid_map done.");
        let gid_map = format!("0 {} 1", self.host_gid);

        gid_map_file.write_all(gid_map.as_bytes())?;
        info!("[Host] wrote {} /proc/self/gid_map", gid_map);
        Ok(())
    }

    fn guid_map(&self) -> std::io::Result<()> {
        self.uid_map().expect("Failed to write uid_map");
        self.gid_map().expect("Failed to write gid_map");
        Ok(())
    }

    fn get_full_path(&self) -> String {
        format!("{}/{}", self.config.container_path, self.id)
    }

    pub fn prepare(&mut self) {
        // specify Image name
        if let Some(image) = &mut self.image {
            image.pull(&self.id).expect("Failed to cromwell pull");

            let c_hosts = format!("{}/etc/hosts", image.get_full_path(&self.id));
            let c_resolv = format!("{}/etc/resolv.conf", image.get_full_path(&self.id));

            fs::copy("/etc/hosts", &c_hosts).expect("Failed copy /etc/hosts");
            info!("[Host] Copied /etc/hosts to {}", c_hosts);

            fs::copy("/etc/resolv.conf", &c_resolv).expect("Failed copy /etc/resolv.conf");
            info!("[Host] Copied /etc/resolv.conf {}", c_resolv);
        }

        // nochdir, close tty
        if self.become_daemon {
            daemon(true, false).expect("cannot become daemon");
        }

        unshare(
            CloneFlags::CLONE_NEWPID
                | CloneFlags::CLONE_NEWUTS
                | CloneFlags::CLONE_NEWNS
                | CloneFlags::CLONE_NEWUSER,
        )
        .expect("Can not unshare(2).");

        self.guid_map()
            .expect("Failed to write /proc/self/gid_map|uid_map");
    }

    pub fn run(&self) {
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                info!("[Host] PID: {}", getpid());
                info!("[Container] PID: {}", child);

                let pids = "/home/vagrant/.cromwell/pids";
                fs::create_dir_all(pids).expect("failed mkdir pids");

                let pidfile_path = format!("{}/{}", pids, self.id);
                let pidfile_path = Path::new(&pidfile_path);
                println!("{:?}", pidfile_path);

                Pidfile::create(&pidfile_path, child).expect("Failed to create pidfile");

                match waitpid(child, None).expect("waitpid faild") {
                    WaitStatus::Exited(_, _) => {
                        Pidfile::delete(&pidfile_path).expect("Failed to remove pidfile");
                    }
                    WaitStatus::Signaled(_, _, _) => {}
                    _ => eprintln!("Unexpected exit."),
                }
            }
            Ok(ForkResult::Child) => {
                chroot(self.get_full_path().as_str()).expect("chroot failed.");
                chdir("/").expect("cd / failed.");

                sethostname(&self.id).expect("Could not set hostname");
                fs::create_dir_all("proc").unwrap_or_else(|why| {
                    eprintln!("{:?}", why.kind());
                });

                info!("[Container] Mount procfs ... ");
                mounts::mount_proc().expect("mount procfs failed");

                let cmd = CString::new(self.command.clone()).unwrap();
                let default_shell = CString::new("/bin/sh").unwrap();
                let shell_opt = CString::new("-c").unwrap();
                let lang = CString::new("LC_ALL=C").unwrap();
                let path =
                    CString::new("PATH=/bin/:/usr/bin/:/usr/local/bin:/sbin:/usr/sbin").unwrap();

                execve(
                    &default_shell,
                    &[default_shell.clone(), shell_opt, cmd],
                    &[lang, path],
                )
                .expect("execution failed.");
            }
            Err(e) => panic!("Fork failed: {}", e),
        }
    }

    pub fn delete(&self) -> std::io::Result<()> {
        fs::remove_dir_all(&self.get_full_path())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_container() {
        let image_name = "library/alpine:3.8";
        let command = "/bin/bash";
        let container = Container::new(Some(image_name), &command, None, false);
        assert_eq!(container.command, command);
    }
}
