use std::env;
use std::ffi::CString;
use std::fs;

use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{execv, fork, sethostname, ForkResult};

use super::mounts;

use super::network::{Bridge, Network};

pub struct Container {
    pub name: String,
    pub path: String,
    pub command: String,
    pub network: Network,
}

impl Container {
    pub fn new(name: String, command: String) -> Container {
        let path = format!("{}/{}", get_containers_path().unwrap(), name.clone());

        Container {
            name: name.clone(),
            path: path,
            command: command,
            network: initialize_network(name),
        }
    }
    pub fn path_str(&self) -> &str {
        self.path.as_str()
    }

    pub fn struct_network(&self) {
        let network = &self.network;
        if !network.bridge.existed() {
            println!("Creating {} ...", network.bridge.name);
            network
                .bridge
                .add_bridge_ace0()
                .expect("Could not create bridge");
        }
        if !network.existed_namespace() {
            network
                .add_network_namespace()
                .expect("failed adding network namespace");
            println!("Created namespace {}", network.bridge.name);
        }

        if !network.existed_veth() {
            network.add_veth().expect("failed adding veth peer");
            println!("Created veth_host: {}", network.veth_host);
            println!("Created veth_guest: {}", network.veth_guest);
        }
        network
            .add_container_network()
            .expect("Could not add container network");
    }

    pub fn prepare(&self) {
        println!("Started initialize Container!");
        let c_hosts = format!("{}/etc/hosts", self.path);
        let c_resolv = format!("{}/etc/resolv.conf", self.path);

        fs::copy("/etc/hosts", c_hosts).expect("Failed copy file: ");
        fs::copy("/etc/resolv.conf", c_resolv).expect("Failed copy file: ");
    }

    pub fn run(&self) {
        println!("fork(2) start!");
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                println!("container pid: {}", child);
                match waitpid(child, None).expect("waitpid faild") {
                    WaitStatus::Exited(_, _) => {}
                    WaitStatus::Signaled(_, _, _) => {}
                    _ => eprintln!("Unexpected exit."),
                }
            }
            Ok(ForkResult::Child) => {
                sethostname(&self.name).expect("Could not set hostname");

                fs::create_dir_all("proc").unwrap_or_else(|why| {
                    eprintln!("{:?}", why.kind());
                });

                println!("Mount procfs ... ");
                mounts::mount_proc().expect("mount procfs faild.");

                let cmd = CString::new(self.command.clone()).unwrap();
                let default_shell = CString::new("/bin/bash").unwrap();
                let shell_opt = CString::new("-c").unwrap();

                execv(&default_shell, &[default_shell.clone(), shell_opt, cmd])
                    .expect("execution faild.");
            }
            Err(_) => eprintln!("Fork failed"),
        }
    }
    pub fn delete(&self) -> std::io::Result<()> {
        fs::remove_dir_all(&self.path)
    }
}

fn initialize_network(name: String) -> Network {
    Network::new(
        format!("{}-ns", name),
        Bridge::new(),
        format!("{}_host", name),
        format!("{}_guest", name),
        "172.0.0.2".parse().unwrap(),
    )
}

pub fn get_containers_path() -> Result<String, env::VarError> {
    let ace_container_env = "ACE_CONTAINER_PATH";
    env::var(ace_container_env)
}

#[test]
fn test_get_container_path() {
    let ace_container_env = "ACE_CONTAINER_PATH";
    let ace_container_path = "/var/lib/ace-containers";
    env::set_var(ace_container_env, ace_container_path);

    assert_eq!(ace_container_path, get_containers_path().unwrap())
}
