#[macro_use]
extern crate serde_derive;
extern crate env_logger;

use std::process::exit;

use clap::{crate_name, crate_version, App, Arg, SubCommand};

mod bootstrap;
mod cgroups;
mod commands;
mod config;
mod container;
mod image;
mod mounts;
mod network;
mod runner;

fn main() {
    env_logger::init();

    let mut app = App::new(crate_name!())
        .version(crate_version!())
        .author("Takashi IIGUNI <ad2314ce71926@gmail.com>")
        .about("Rust Rootless Container Runntime");
    let app_matches = &app
        .clone()
        .subcommand(
            SubCommand::with_name("run")
                .version(crate_version!())
                .about("run cromwell container")
                .arg(
                    Arg::with_name("container_name")
                        .long("name")
                        .short("n")
                        .help("Specify container name")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("exec_command")
                        .long("exec")
                        .help("Specify exec your command")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("delete container")
                        .long("del")
                        .help("delete container dir")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("container_path")
                        .long("path")
                        .help("specify container path")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("daemonize_flag")
                        .short("d")
                        .help("daemonize flag"),
                ),
        )
        .subcommand(
            SubCommand::with_name("pull")
                .version(crate_version!())
                .about("pull oci image")
                .arg(
                    Arg::with_name("image_name")
                        .long("name")
                        .short("n")
                        .help("Specify image name")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    match &app_matches.subcommand() {
        ("run", Some(sub_m)) => runner::run(&sub_m),
        ("pull", Some(sub_m)) => runner::pull(&sub_m),
        _ => {
            eprintln!("Unexpected arguments");
            app.print_help().unwrap();
            println!();
            exit(1);
        }
    }
}
