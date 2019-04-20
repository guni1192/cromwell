use std::ffi::CString;
use std::fs::{self, File};
use std::io::prelude::*;
use std::iter;
use std::path::Path;

use nix::sched::{unshare, CloneFlags};
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{chdir, chroot, daemon, fork, getpid, ForkResult, Gid, Uid};
use nix::unistd::{execve, sethostname};

use dirs::home_dir;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use log::info;

use super::image::Image;
use super::mounts;
use super::pids::Pidfile;
use super::process::Process;

pub struct Container {
    pub id: String,
    image: Option<Image>,
}

impl Container {
    pub fn new(image: Option<Image>, path: Option<&str>) -> Container {
        let id: String = match path {
            Some(id) => id.to_string(),
            None => {
                let mut rng = thread_rng();
                iter::repeat(())
                    .map(|()| rng.sample(Alphanumeric))
                    .take(8)
                    .collect::<String>()
            }
        };

        Container { id, image }
    }

    fn uid_map(&self, uid: Uid) -> std::io::Result<()> {
        let mut uid_map_file = File::create("/proc/self/uid_map")?;
        let uid_map = format!("0 {} 1", uid);

        uid_map_file.write_all(uid_map.as_bytes())?;
        info!("[Host] wrote {} /proc/self/uid_map", uid_map);
        Ok(())
    }

    fn gid_map(&self, gid: Gid) -> std::io::Result<()> {
        let mut setgroups_file = File::create("/proc/self/setgroups")?;
        setgroups_file.write_all(b"deny")?;

        let mut gid_map_file = File::create("/proc/self/gid_map")?;
        info!("[Host] open(2) /proc/self/gid_map done.");
        let gid_map = format!("0 {} 1", gid);

        gid_map_file.write_all(gid_map.as_bytes())?;
        info!("[Host] wrote {} /proc/self/gid_map", gid_map);
        Ok(())
    }

    fn guid_map(&self, process: &Process) -> std::io::Result<()> {
        self.uid_map(process.host_uid)
            .expect("Failed to write uid_map");
        self.gid_map(process.host_gid)
            .expect("Failed to write gid_map");
        Ok(())
    }

    pub fn prepare(&mut self, process: &Process) {
        // specify Image name
        if let Some(image) = &mut self.image {
            image.pull().expect("Failed to cromwell pull");
            image
                .build_from_tar(&process.cwd)
                .expect("Failed build image from fsLayer");

            let c_hosts = format!("{}/etc/hosts", process.cwd);
            let c_resolv = format!("{}/etc/resolv.conf", process.cwd);

            fs::copy("/etc/hosts", &c_hosts).expect("Failed copy /etc/hosts");
            info!("[Host] Copied /etc/hosts to {}", c_hosts);

            fs::copy("/etc/resolv.conf", &c_resolv).expect("Failed copy /etc/resolv.conf");
            info!("[Host] Copied /etc/resolv.conf {}", c_resolv);
        }

        // nochdir, close tty
        if process.become_daemon {
            daemon(true, false).expect("cannot become daemon");
        }

        unshare(
            CloneFlags::CLONE_NEWPID
                | CloneFlags::CLONE_NEWUTS
                | CloneFlags::CLONE_NEWNS
                | CloneFlags::CLONE_NEWUSER,
        )
        .expect("Can not unshare(2).");

        self.guid_map(&process)
            .expect("Failed to write /proc/self/gid_map|uid_map");
    }

    pub fn run(&self, process: &Process) {
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                info!("[Host] PID: {}", getpid());
                info!("[Container] PID: {}", child);

                let home = home_dir().expect("Could not get your home_dir");
                let home = home.to_str().expect("Could not PathBuf to str");
                let pids_path = format!("{}/.cromwell/pids", home);
                fs::create_dir_all(&pids_path).expect("failed mkdir pids");

                let pidfile_path = format!("{}/{}.pid", pids_path, self.id);
                let pidfile_path = Path::new(&pidfile_path);

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
                chroot(Path::new(&process.cwd)).expect("chroot failed.");
                chdir("/").expect("cd / failed.");

                sethostname(&self.id).expect("Could not set hostname");
                fs::create_dir_all("proc").unwrap_or_else(|why| {
                    eprintln!("{:?}", why.kind());
                });

                info!("[Container] Mount procfs ... ");
                mounts::mount_proc().expect("mount procfs failed");

                let cmd = CString::new(process.cmd.clone()).unwrap();
                let default_shell = CString::new("/bin/sh").unwrap();
                let shell_opt = CString::new("-c").unwrap();

                execve(
                    &default_shell,
                    &[default_shell.clone(), shell_opt, cmd],
                    &process.env,
                )
                .expect("execution failed.");
            }
            Err(e) => panic!("Fork failed: {}", e),
        }
    }

    pub fn delete(&self, process: &Process) -> std::io::Result<()> {
        fs::remove_dir_all(&process.cwd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_container() {
        let image_name = Some("library/alpine:3.8");
        let image = match image_name {
            Some(name) => Some(Image::new(name)),
            None => None,
        };
        let container = Container::new(image, None);
        assert_eq!(container.id.len(), 8);
    }
}
