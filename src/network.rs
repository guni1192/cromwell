use std::io;
use std::process::Command;

use nix::unistd::Pid;

pub struct Network {
    name: String,
}

impl Network {
    pub fn new(name: &str) -> Self {
        Network {
            name: name.to_string(),
        }
    }

    pub fn run(&self, pid: Pid) -> io::Result<()> {
        Command::new("slirp4netns")
            .arg("--configure")
            .arg("--mtu=65520")
            .arg(pid.to_string())
            .arg(&self.name)
            .output()?;
        Ok(())
    }
}
