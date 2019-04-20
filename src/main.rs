#[macro_use]
extern crate serde_derive;
extern crate env_logger;
#[macro_use]
extern crate prettytable;
use std::process::exit;

use clap::{crate_name, crate_version, App, Arg, SubCommand};

mod commands;
mod config;
mod container;
mod image;
mod mounts;
mod network;
mod pids;
mod process;
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
        .subcommand(
            SubCommand::with_name("ps")
                .version(crate_version!())
                .about("show containers"),
        )
        .get_matches();

    let config = config::Config::new(None);

    match &app_matches.subcommand() {
        ("run", Some(sub_m)) => runner::run(&sub_m),
        ("pull", Some(sub_m)) => runner::pull(&sub_m),
        ("ps", Some(sub_m)) => pids::show(&sub_m, config).expect("cannot get container processes"),
        _ => {
            eprintln!("Unexpected arguments");
            app.print_help().unwrap();
            println!();
            exit(1);
        }
    }
}
