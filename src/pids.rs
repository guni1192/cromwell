use std::fs;
use std::io;
use std::path::Path;

use clap::ArgMatches;

use prettytable::Table;

use nix::unistd::Pid;

use super::config::Config;

pub fn show(_sub_m: &ArgMatches, config: Config) -> io::Result<()> {
    // Create the table
    let mut table = Table::new();

    table.add_row(row!["Container ID", "PID"]);

    let pids_path = format!("{}/pids", config.base_dir);
    let pid_dir = Path::new(&pids_path);

    if pid_dir.is_dir() {
        for entry in fs::read_dir(pid_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                continue;
            }
            let pidfile = Pidfile::read(&path).expect("Failed to read pidfile");
            table.add_row(row![pidfile.container_id, pidfile.pid]);
        }
    }
    table.printstd();
    Ok(())
}

pub struct Pidfile {
    pid: Pid,
    container_id: String, // as file_name
}

impl Pidfile {
    pub fn create(path: &Path, pid: Pid) -> io::Result<()> {
        fs::write(path, pid.to_string().as_bytes())
    }

    pub fn delete(path: &Path) -> io::Result<()> {
        fs::remove_file(path)
    }

    fn read(path: &Path) -> io::Result<Pidfile> {
        let pid: i32 = fs::read_to_string(path).unwrap().parse().unwrap();
        let pid = Pid::from_raw(pid);
        let container_id = path.file_stem().unwrap().to_str().unwrap();

        Ok(Pidfile {
            pid,
            container_id: container_id.to_string(),
        })
    }
}
