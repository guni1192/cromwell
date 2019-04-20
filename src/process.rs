use std::ffi::CString;

use nix::unistd::{getgid, getuid, Gid, Uid};

pub struct Process {
    pub cmd: Vec<CString>,
    pub host_uid: Uid,
    pub host_gid: Gid,
    pub cwd: String,
    pub become_daemon: bool,
    pub env: Vec<CString>,
}

impl Process {
    pub fn new(cmd: Vec<CString>, cwd: String, become_daemon: bool, env: Vec<CString>) -> Self {
        Process {
            cwd,
            env,
            become_daemon,
            cmd,
            host_uid: getuid(),
            host_gid: getgid(),
        }
    }
}
