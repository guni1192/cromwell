use std::process::exit;

use clap::{App, Arg, SubCommand};

mod bootstrap;
mod commands;
mod container;
mod mounts;
mod network;
mod options;
mod runner;

fn main() {
    let mut app = App::new("Cromwell")
        .version("v0.1.0")
        .author("Takashi IIGUNI <ad2314ce71926@gmail.com>")
        .about("Ownership Managed Container Runntime");
    let app_matches = &app
        .clone()
        .subcommand(
            SubCommand::with_name("run")
                .version("v1.0.0")
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
                ),
        )
        .get_matches();

    match &app_matches.subcommand() {
        ("run", Some(sub_m)) => runner::run(&sub_m),
        _ => {
            eprintln!("Unexpected arguments");
            app.print_help().unwrap();
            println!();
            exit(1);
        }
    }
}
