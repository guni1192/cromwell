use std::process::exit;

use clap::{App, Arg, SubCommand};

mod bootstrap;
mod cli;
mod commands;
mod container;
mod mounts;
mod network;
mod options;

fn main() {
    let mut app = App::new("Cromwell")
        .version("v1.0.0")
        .author("Takashi IIGUNI <ad2314ce71926@gmail.com>")
        .about("Ownership Managed Container Runntime");
    let app_matches = app
        .clone()
        .subcommand(
            SubCommand::with_name("run")
                .version("v1.0.0")
                .about("run cromwell container")
                .arg(
                    Arg::with_name("container name")
                        .long("name")
                        .short("n")
                        .help("Specify container name")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("exec command")
                        .long("exec")
                        .help("Specify exec your command")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("delete container")
                        .long("del")
                        .help("delete container dir")
                        .takes_value(true),
                ),
        )
        .get_matches();

    match &app_matches.subcommand() {
        ("run", Some(sub_m)) => cli::run((*sub_m).clone()),
        // Some("network", sub_m) => cli::network(sub_m),
        _ => {
            eprintln!("Unexpected arguments");
            app.print_help().unwrap();
            println!();
            exit(1);
        }
    }
}
