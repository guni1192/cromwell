use std::fs;
use std::env::{set_var, args};
use std::process::*;
use std::ffi::CString;
use nix::unistd::*;
use nix::unistd::{execv, fork, ForkResult};
use nix::sched::*; // 調べる
use nix::unistd::*;
use nix::sys::wait::*;
use nix::mount::{mount, MsFlags};

fn print_help() {}

// TODO Bootstrap func

fn main() {
    // debug
    set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("invalid argments");
        print_help();
        exit(1);
    }

    let container_path = args[1].as_str();

    match unshare(CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS) {
        Ok(_) => {},
        Err(e) => eprintln!("{}", e)
    }

    fs::create_dir_all(container_path).unwrap();

    mount(
        None::<&str>,
        "/",
        None::<&str>,
        MsFlags::MS_PRIVATE,
        None::<&str>,
    ).expect("Can not mount specify dir.");


    mount(
        Some(container_path),
        container_path,
        None::<&str>,
        MsFlags::MS_BIND | MsFlags::MS_REC,
        None::<&str>,
    ).expect("mount root dir faild.");

    chroot(container_path).expect("chroot failed.");

    chdir("/").expect("cd / faild.");

    match fork() {
        Ok(ForkResult::Parent{ child, .. }) => {
            // 親プロセスは待つだけ
            match waitpid(child, None).expect("wait_pid faild") {
                WaitStatus::Exited(pid, status) => {
                    println!("Exit: pid: {:?}, status: {:?}", pid, status)
                }
                WaitStatus::Signaled(pid, status, _) => {
                    println!("Signal: pid={:?}, status={:?}", pid, status)
                }
                _ => eprintln!("Unexpected exit."),
            }
        }
        Ok(ForkResult::Child) => {
            // Setting Host
            sethostname("archlinux-test-container").expect("sethostname faild.");
            // TODO: locale

            fs::create_dir_all("proc").unwrap_or_else(|why| {
                eprintln!("{:?}", why.kind());
            });

            mount(
                Some("proc"),
                "/proc",
                Some("proc"),
                MsFlags::MS_MGC_VAL,
                None::<&str>,
            ).expect("mount procfs faild.");

            let dir = CString::new("/bin/bash".to_string()).unwrap();
            let arg = CString::new("-l".to_string()).unwrap();

            execv(&dir, &[dir.clone(), arg]).expect("execution faild.");

        }
        Err(_) => eprintln!("Fork failed"),
    }


}
