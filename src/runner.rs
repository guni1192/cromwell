use std::iter;

use clap::ArgMatches;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use super::container;
use super::image::Image;

pub fn run(sub_m: &ArgMatches) {
    let command = match sub_m.value_of("exec_command") {
        Some(c) => c,
        None => "/bin/sh",
    };

    let container_name = sub_m.value_of("container_name");

    let container_path = sub_m.value_of("container_path");

    let become_daemon = sub_m.is_present("daemonize_flag");

    let mut container =
        container::Container::new(container_name, &command, container_path, become_daemon);

    if sub_m.is_present("del") {
        container.delete().expect("Failed to remove container: ");
    }

    container.prepare();

    container.run();
}

pub fn pull(sub_m: &ArgMatches) {
    let image_name = sub_m
        .value_of("image_name")
        .expect("invalied arguments about image name");

    let mut rng = thread_rng();
    let id: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(16)
        .collect();
    let mut image = Image::new(image_name);

    image.pull(&id).expect("Failed to image pull");
}
