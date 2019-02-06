use std::env;
use std::process;

use clap::ArgMatches;
use nix::unistd::getuid;

// use super::bootstrap::pacstrap;
use super::container;
use super::image::Image;

// TODO: deamonize option
pub fn run(sub_m: &ArgMatches) {
    let ace_container_path_env = "ACE_CONTAINER_PATH";
    let ace_path = "/var/lib/cromwell/containers";
    env::set_var(ace_container_path_env, ace_path);

    let command = match sub_m.value_of("exec_command") {
        Some(c) => c.to_string(),
        None => "/bin/sh".to_string(),
    };

    let container_name = sub_m
        .value_of("container_name")
        .expect("invalied arguments about container name");

    let pid = process::id();
    println!("pid: {}", pid);

    let uid = getuid();

    let mut container = container::Container::new(container_name.to_string(), command, uid);

    if sub_m.is_present("del") {
        container.delete().expect("Faild to remove container: ");
    }

    // TODO: pull rootfs docker image by DockerHub
    // bootstraping
    // if !Path::new(&container.path).exists() {
    //     println!(
    //         "Creating container bootstrap to {} ...",
    //         container.path.as_str()
    //     );
    //     fs::create_dir_all(container.path.as_str())
    //         .expect("Could not create directory to your path");
    //     pacstrap(container.path_str());
    // }

    container.prepare();

    container.run();
}

pub fn pull(sub_m: &ArgMatches) {
    let image_name = sub_m
        .value_of("image_name")
        .expect("invalied arguments about image name");

    let mut image = Image::new(image_name.to_string());
    image.pull().expect("Failed to image pull");
}
