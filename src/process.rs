use nix::unistd::{getgid, getuid, Gid, Uid};

pub struct Process {
    pub cmd: String,
    pub host_uid: Uid,
    pub host_gid: Gid,
    pub cwd: String,
    pub become_daemon: bool,
    pub env: Vec<String>,
}

impl Process {
    pub fn new(cmd: &str, cwd: String, become_daemon: bool, env: Vec<String>) -> Self {
        Process {
            cwd,
            env,
            become_daemon,
            cmd: cmd.to_string(),
            host_uid: getuid(),
            host_gid: getgid(),
        }
    }
}
