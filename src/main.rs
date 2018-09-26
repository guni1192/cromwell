use nix::mount::{mount, MsFlags};
use nix::sched::*; // 調べる
use nix::sys::wait::*;
use nix::unistd::*;
// use nix::unistd::{execv, fork, ForkResult};
use getopts::Options;
use std::env::{args, set_var};
use std::ffi::CString;
use std::fs;
use std::process::*;

fn print_help() {
    println!("help message");
}

// TODO Bootstrap func

fn main() {
    // debug
    set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = args().collect();

    let mut opts = Options::new();
    opts.optopt("", "path", "set container path", "CONTAINER PATH");
    opts.optflag("h", "help", "print help message");

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

    println!("{:?}", container_path);
    // let container_path = args[1].as_str();

    match unshare(CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
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
            ).expect("mount procfs faild.");

            let dir = CString::new("/bin/bash".to_string()).unwrap();
            let arg = CString::new("-l".to_string()).unwrap();

            execv(&dir, &[dir.clone(), arg]).expect("execution faild.");
        }
        Err(_) => eprintln!("Fork failed"),
    }
}
