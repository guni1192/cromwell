use std::env::args;
use std::process::exit;

mod bootstrap;
mod cli;
mod commands;
mod container;
mod help;
mod mount;
mod network;
mod options;

use self::help::print_help;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        print_help();
        exit(1);
    }

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
}
