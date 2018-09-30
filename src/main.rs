mod bootstrap;
mod help;
mod options;

use self::bootstrap::pacstrap;
use self::help::print_help;
use self::options::get_options;
use nix::mount::{mount, MsFlags};
use nix::sched::*;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{chdir, chroot, execv, fork, ForkResult};
use std::env::args;
use std::ffi::CString;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }

    let matches = get_options(args).expect("Invalid arguments");

    if matches.opt_present("h") {
        print_help();
        return;
    }

    let command = match matches.opt_str("exec") {
        Some(c) => c,
        None => "/bin/bash".to_string(),
    };

    let container_path = matches
        .opt_str("path")
        .expect("invalied arguments about path");
    let container_path = container_path.as_str();

    fs::create_dir_all(container_path).expect("Could not create directory to your path");

    if matches.opt_present("init") {
        pacstrap(container_path);
        return;
    }

    if !Path::new(&format!("{}/etc", container_path)).exists() {
        pacstrap(container_path);
    }

    unshare(
        CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWNET
            | CloneFlags::CLONE_FS,
    )
    .expect("Can not unshare(2).");

    mount(
        None::<&str>,
        "/",
        None::<&str>,
        MsFlags::MS_PRIVATE,
        None::<&str>,
    )
    .expect("Can not mount specify dir.");

    mount(
        Some(container_path),
        container_path,
        None::<&str>,
        MsFlags::MS_BIND | MsFlags::MS_REC,
        None::<&str>,
    )
    .expect("Can not mount root dir.");

    chroot(container_path).expect("chroot failed.");

    chdir("/").expect("cd / faild.");

    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            // 親プロセスは待つだけ
            match waitpid(child, None).expect("waitpid faild") {
                WaitStatus::Exited(_, _) => {}
                WaitStatus::Signaled(_, _, _) => {}
                _ => eprintln!("Unexpected exit."),
            }
        }
        Ok(ForkResult::Child) => {
            fs::create_dir_all("proc").unwrap_or_else(|why| {
                eprintln!("{:?}", why.kind());
            });

            mount(
                Some("proc"),
                "/proc",
                Some("proc"),
                MsFlags::MS_MGC_VAL,
                None::<&str>,
            )
            .expect("mount procfs faild.");

            let dir = CString::new(command).unwrap();

            execv(&dir, &[dir.clone()]).expect("execution faild.");
        }
        Err(_) => eprintln!("Fork failed"),
    }
}
