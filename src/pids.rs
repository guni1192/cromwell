use std::path::Path;

use clap::ArgMatches;
use prettytable::{Cell, Row, Table};

use nix::unistd::Pid;

pub fn show(sub_m: &ArgMatches) {
    // Create the table
    let mut table = Table::new();

    table.add_row(row!["Container ID", "PID"]);
    for i in 0..3 {
        let pidfile = Pidfile::read(Path::new("/hoge"));
        table.add_row(row![pidfile.name, pidfile.pid]);
    }

    table.printstd();
}

struct Pidfile {
    pid: Pid,
    name: String,
}

impl Pidfile {
    fn read(path: &Path) -> Self {
        Pidfile {
            pid: Pid::from_raw(1000),
            name: "library/alpine:latest".to_string(),
        }
    }
}
