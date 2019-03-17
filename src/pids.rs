use std::fs;
use std::io;
use std::path::Path;

use clap::ArgMatches;
use prettytable::Table;

use nix::unistd::Pid;

pub fn show(_sub_m: &ArgMatches) {
    // Create the table
    let mut table = Table::new();

    table.add_row(row!["Container ID", "PID"]);
    for _ in 0..3 {
        let pidfile = Pidfile::read(Path::new("/hoge"));
        table.add_row(row![pidfile.name, pidfile.pid]);
    }

    table.printstd();
}

pub struct Pidfile {
    pid: Pid,
    name: String, // Container.ID
}

impl Pidfile {
    pub fn create(path: &Path, pid: Pid) -> io::Result<()> {
        fs::write(path, pid.to_string().as_bytes())
    }

    pub fn delete(path: &Path) -> io::Result<()> {
        fs::remove_file(path)
    }

    fn read(path: &Path) -> Self {
        Pidfile {
            pid: Pid::from_raw(1000),
            name: "library/alpine:latest".to_string(),
        }
    }
}
