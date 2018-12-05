use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::process::exit;

use nix::sched::*;
use nix::unistd::{chdir, chroot};

use super::bootstrap::pacstrap;
use super::container;
use super::help::print_help;
use super::mounts;
use super::network::{Bridge, Network};
use super::options;

// TODO: deamonize option
pub fn run(args: &[String]) {
    let args = args.to_vec();

    let ace_container_path = "ACE_CONTAINER_PATH";
    // TODO: settting.rsからの読み込みに変更
    env::set_var(ace_container_path, "/var/lib/ace-containers");

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

    let container = container::Container::new(container_name.clone(), command);

    if matches.opt_present("del") {
        container.delete().expect("Faild to remove container: ");
    }

    // bootstraping
    if !Path::new(&container.path).exists() {
        println!(
            "Creating container bootstrap to {} ...",
            container.path_str()
        );
        fs::create_dir_all(container.path_str()).expect("Could not create directory to your path");
        pacstrap(container.path_str());
    }

    let pid = process::id();
    println!("pid: {}", pid);

    container.prepare();
    // println!("Creating network...");
    // container.struct_network();

    // mounts
    println!("Mount rootfs ... ");
    mounts::mount_rootfs().expect("Can not mount root dir.");
    println!("Mount container path ... ");
    mounts::mount_container_path(container.path_str()).expect("Can not mount specify dir.");

    unshare(
        CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWIPC
            | CloneFlags::CLONE_NEWUTS
            | CloneFlags::CLONE_NEWNS
            // | CloneFlags::CLONE_NEWNET,
    )
    .expect("Can not unshare(2).");

    chroot(container.path_str()).expect("chroot failed.");

    chdir("/").expect("cd / failed.");

    container.run();
}

pub fn network(args: &[String]) {
    let args = args.to_vec();
    let matches = options::get_network_options(args).expect("Invalid arguments");

    let container_name = matches
        .opt_str("name")
        .expect("invalied arguments about container name");

    let network = Network::new(
        format!("{}-ns", &container_name),
        Bridge::new(),
        format!("{}_host", &container_name),
        format!("{}_guest", &container_name),
        "172.0.0.2".parse().unwrap(),
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

    if matches.opt_present("clean") {
        network.clean().expect("Failed clean up network");
        exit(0);
    }
}
