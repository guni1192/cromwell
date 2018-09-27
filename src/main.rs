use getopts::Options;
use nix::mount::{mount, MsFlags};
use nix::sched::*;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{chdir, chroot, execv, fork, sethostname, ForkResult};
use std::env::{args, set_var};
use std::ffi::CString;
use std::fs;
use std::process::Command;

fn print_help() {
    println!("help message");
}

// TODO Bootstrap func
fn bootstrap_container(container_path: &str) -> Result<&str, &str> {
    match Command::new("pacstrap")
        .arg("-i")
        .arg(format!("{}", container_path))
        .arg("base")
        .arg("--noconfirm")
        .spawn()
    {
        Ok(_) => Ok("Bootstrap Done"),
        Err(_) => Err("Faild Bootstrap"),
    }
}

fn main() {
    // debug
    set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = args().collect();

    let mut opts = Options::new();
    opts.optopt("", "path", "set container path", "CONTAINER PATH");
    opts.optflag("h", "help", "print help message");
    opts.optflag("", "init", "exec pacstrap");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_help();
        return;
    }

    let container_path = matches.opt_str("path").unwrap();
    let container_path = container_path.as_str();

    fs::create_dir_all(container_path).unwrap();

    if matches.opt_present("init") {
        match bootstrap_container(container_path) {
            Ok(m) => println!("{:?}", m),
            Err(e) => eprintln!("{:?}", e),
        };
        return;
    }

    unshare(CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWNET)
        .expect("Can not unshare(2).");

    match bootstrap_container(container_path) {
        Ok(m) => println!("{:?}", m),
        Err(e) => eprintln!("{:?}", e),
    };

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
            )
            .expect("mount procfs faild.");

            let dir = CString::new("/bin/bash".to_string()).unwrap();
            let arg = CString::new("-l".to_string()).unwrap();

            execv(&dir, &[dir.clone(), arg]).expect("execution faild.");
        }
        Err(_) => eprintln!("Fork failed"),
    }
}
