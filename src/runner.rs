use clap::ArgMatches;

use super::container;
use super::image::Image;
use super::process::Process;

pub fn run(sub_m: &ArgMatches) {
    let command = match sub_m.value_of("exec_command") {
        Some(c) => c,
        None => "/bin/sh",
    };

    let image_name = sub_m.value_of("container_name");
    let image = match image_name {
        Some(name) => Some(Image::new(name)),
        None => None,
    };

    let container_path = sub_m.value_of("container_path");

    let become_daemon = sub_m.is_present("daemonize_flag");

    let mut container = container::Container::new(image, container_path);

    if sub_m.is_present("del") {
        container.delete().expect("Failed to remove container: ");
    }

    let process = Process::new(
        command,
        format!("/home/vagrant/.cromwell/containers/{}", container.id),
        become_daemon,
        Vec::<String>::new(),
    );

    container.prepare(&process);
    container.run(&process);
}

pub fn pull(sub_m: &ArgMatches) {
    let image_name = sub_m
        .value_of("image_name")
        .expect("invalied arguments about image name");

    let mut image = Image::new(image_name);

    image.pull().expect("Failed to image pull");
}
