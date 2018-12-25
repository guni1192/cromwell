use std::env::args;
use std::process::exit;

use clap::{App, Arg, SubCommand};

mod bootstrap;
mod cli;
mod commands;
mod container;
mod help;
mod mounts;
mod network;
mod options;

use self::help::print_help;

fn main() {
    let app_matches = App::new("Cromwell")
        .version("v1.0.0")
        .author("Takashi IIGUNI <ad2314ce71926@gmail.com>")
        .about("Ownership Managed Container Runntime")
        .subcommand(
            SubCommand::with_name("run")
                .version("v1.0.0")
                .about("run cromwell container")
                .arg(Arg::with_name("debug").help("print debug information verbosely")),
        )
        .get_matches();

    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        print_help();
        exit(1);
    }

    match &app_matches.subcommand() {
        ("run", Some(sub_m)) => cli::run(sub_m.clone().clone()),
        // Some("network", sub_m) => cli::network(sub_m),
        _ => {
            eprintln!("Unexpected Arguments");
            print_help();
            exit(1);
        }
    }

    /*
    match &args[1][..] {
        "run" => cli::run(&args[1..]),
        "network" => cli::network(&args[1..]),
        "help" => {
            print_help();
            exit(0);
        }
        _ => {
            eprintln!("Unexpected Arguments");
            print_help();
            exit(1);
        }
    }
    */
}
