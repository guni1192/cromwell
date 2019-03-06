use clap::ArgMatches;

use nix::unistd::daemon;

use super::container;
use super::image::Image;

// TODO: deamonize option
pub fn run(sub_m: &ArgMatches) {
    let command = match sub_m.value_of("exec_command") {
        Some(c) => c.to_string(),
        None => "/bin/sh".to_string(),
    };

    let container_name = sub_m
        .value_of("container_name")
        .expect("invalied arguments about container name");

    let mut container = container::Container::new(container_name, command);

    if sub_m.is_present("del") {
        container.delete().expect("Failed to remove container: ");
    }

    // daemonize
    if sub_m.is_present("daemonize_flag") {
        // nochdir, close tty
        println!("become daemon");
        daemon(true, false).expect("cannot become daemon");
    }

    container.prepare();

    container.run();
}

pub fn pull(sub_m: &ArgMatches) {
    let image_name = sub_m
        .value_of("image_name")
        .expect("invalied arguments about image name");

    let mut image = Image::new(image_name);
    image.pull().expect("Failed to image pull");
}
