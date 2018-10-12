use std::env;
use std::ffi::CString;
use std::fs;
use std::path::Path;
use std::process::exit;

use nix::mount::{mount, MsFlags};
use nix::sched::*;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{chdir, chroot, execv, fork, sethostname, ForkResult};

use super::bootstrap::pacstrap;
use super::container;
use super::help::print_help;
use super::mount;
use super::network::{Bridge, Network};
use super::options;

// TODO: deamonize option
pub fn run(args: &[String]) {
    let args = args.to_vec();
    let ace_container_path = "ACE_CONTAINER_PATH";
    // TODO: settting.rsからの読み込みに変更
    env::set_var(ace_container_path, "/var/tmp/ace-containers");

    let default_container_path = match env::var(ace_container_path) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Could' not get {}: {}", ace_container_path, e);
            exit(1);
        }
    };

    let matches = options::get_runner_options(args).expect("Invalid arguments");

    if matches.opt_present("help") {
        print_help();
        exit(0);
    }

    let command = match matches.opt_str("exec") {
        Some(c) => c,
        None => "/bin/bash".to_string(),
    };

    let container_name = matches
        .opt_str("name")
        .expect("invalied arguments about container name");
    let container_path = format!("{}/{}", default_container_path, container_name);
    let container_path = container_path.as_str();

    if matches.opt_present("del") {
        match container::delete(container_name.as_str()) {
            Ok(_) => {
                println!("delete container succeed.");
                exit(0);
            }
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        };
    }

    fs::create_dir_all(container_path).expect("Could not create directory to your path");

    if !Path::new(&format!("{}/etc", container_path)).exists() {
        pacstrap(container_path);
    }

    unshare(
        CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWIPC
            | CloneFlags::CLONE_NEWUTS
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
    .expect("Can not mount root dir.");

    mount(
        Some(container_path),
        container_path,
        None::<&str>,
        MsFlags::MS_BIND | MsFlags::MS_REC,
        None::<&str>,
    )
    .expect("Can not mount specify dir.");

    chroot(container_path).expect("chroot failed.");

    chdir("/").expect("cd / faild.");

    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            match waitpid(child, None).expect("waitpid faild") {
                WaitStatus::Exited(_, _) => {}
                WaitStatus::Signaled(_, _, _) => {}
                _ => eprintln!("Unexpected exit."),
            }
        }
        Ok(ForkResult::Child) => {
            sethostname(container_name).expect("Could not set hostname");

            fs::create_dir_all("proc").unwrap_or_else(|why| {
                eprintln!("{:?}", why.kind());
            });

            mount::mount_proc().expect("mount procfs faild.");

            let cmd = CString::new(command.clone()).unwrap();
            let default_shell = CString::new("/bin/bash").unwrap();
            let shell_opt = CString::new("-c").unwrap();

            execv(&default_shell, &[default_shell.clone(), shell_opt, cmd])
                .expect("execution faild.");
        }
        Err(_) => eprintln!("Fork failed"),
    }
}

pub fn network(args: &[String]) {
    let args = args.to_vec();
    let matches = options::get_network_options(args).expect("Invalid arguments");

    let network = Network::new(
        "test-ns".to_string(),
        Bridge::new(),
        "test_veth_host".to_string(),
        "test_veth_guest".to_string(),
    );

    if matches.opt_present("create-bridge") {
        network
            .bridge
            .add_bridge_ace0()
            .expect("Could not create bridge");
        exit(0);
    }

    if matches.opt_present("delete-bridge") {
        network
            .bridge
            .del_bridge_ace0()
            .expect("Could not delete bridge");
        exit(0);
    }
}
