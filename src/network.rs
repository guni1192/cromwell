use std::net::IpAddr;
use std::path::Path;
use std::process;
use std::process::{Child, Command};

use super::commands;

pub struct Bridge {
    pub name: String,
    ip: IpAddr,
}

pub struct Network {
    pub namespace: String,
    pub bridge: Bridge,
    pub veth_guest: String,
    pub veth_host: String,
    container_ip: IpAddr,
    pid: u32,
}

// TODO: no use ip command

impl Bridge {
    pub fn new() -> Bridge {
        Bridge {
            name: "ace0".to_string(),
            ip: "172.0.0.1".parse().unwrap(),
        }
    }
    pub fn add_bridge_ace0(&self) -> Result<(), ()> {
        let commands = [
            format!("ip link add name {} type bridge", self.name),
            format!("ip addr add {}/16 dev {}", self.ip.to_string(), self.name),
            format!("ip link set dev {} up", self.name),
        ];

        match commands::exec_each(&commands) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
    pub fn del_bridge_ace0(&self) -> std::io::Result<Child> {
        Command::new("ip")
            .args(&["link", "del", "name", self.name.as_str()])
            .spawn()
    }

    pub fn existed(&self) -> bool {
        let addrs = nix::ifaddrs::getifaddrs().unwrap();
        let ifname = addrs
            .into_iter()
            .filter(|ifaddr| ifaddr.interface_name == self.name)
            .next();
        match ifname {
            Some(_) => true,
            None => false,
        }
    }
}

impl Network {
    pub fn new(
        namespace: String,
        bridge: Bridge,
        veth_host: String,
        veth_guest: String,
        container_ip: IpAddr,
    ) -> Network {
        Network {
            namespace: namespace,
            bridge: bridge,
            veth_host: veth_host,
            veth_guest: veth_guest,
            container_ip: container_ip,
            pid: process::id(),
        }
    }

    pub fn add_network_namespace(&self) -> Result<String, String> {
        let status = Command::new("ip")
            .args(&["netns", "add", self.namespace.as_str()])
            .status()
            .expect("");
        if status.success() {
            Ok("".to_string())
        } else {
            Err("".to_string())
        }
    }

    pub fn del_network_namespace(&self) -> Result<String, String> {
        let status = Command::new("ip")
            .args(&["netns", "del", self.namespace.as_str()])
            .status()
            .expect("");
        if status.success() {
            Ok("".to_string())
        } else {
            Err("".to_string())
        }
    }

    pub fn existed_namespace(&self) -> bool {
        let ns_path = Path::new("/var/run/netns");
        let ns_path = ns_path.to_str().unwrap();
        let my_ns_path = format!("{}/{}", ns_path, &self.namespace);
        let my_ns_path = Path::new(&my_ns_path);
        my_ns_path.exists()
    }

    pub fn add_veth(&self) -> Result<(), ()> {
        let veth_host = self.veth_host.as_str();
        let veth_guest = self.veth_guest.as_str();
        let commands = [
            format!(
                "ip link add {} type veth peer name {}",
                veth_host, veth_guest
            ),
            format!("ip link set {} up", veth_host),
            format!("ip link set dev {} master {}", veth_host, self.bridge.name),
        ];

        match commands::exec_each(&commands) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub fn del_veth(&self) -> Result<String, String> {
        let status = Command::new("ip")
            .args(&["link", "del", self.veth_host.as_str()])
            .status()
            .unwrap();
        if status.success() {
            Ok("".to_string())
        } else {
            Err("".to_string())
        }
    }

    pub fn exists_veth(&self) -> bool {
        let addrs = nix::ifaddrs::getifaddrs().unwrap();
        let ifname = addrs
            .into_iter()
            .filter(|ifaddr| ifaddr.interface_name == self.veth_host)
            .next();
        match ifname {
            Some(_) => true,
            None => false,
        }
    }

    pub fn add_container_network(&self) -> Result<(), ()> {
        let ns = &self.namespace;
        let guest = &self.veth_guest;
        let commands = [
            format!(
                "ip netns exec {} ip link set {} netns {}",
                ns, guest, &self.pid
            ),
            format!("ip netns exec {} ip link set lo up", ns),
            format!("ip link set {} netns {} up", guest, ns),
            format!(
                "ip netns exec {} ip addr add {}/16 dev {}",
                ns,
                self.container_ip.to_string(),
                guest
            ),
            format!(
                "ip netns exec {} ip route add default via {}",
                ns,
                self.bridge.ip.to_string()
            ),
        ];

        match commands::exec_each(&commands) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub fn del_container_network(&self) -> Result<(), ()> {
        let ns = &self.namespace;
        let commands = [format!("ip netns exec {} ip route del default", ns)];

        match commands::exec_each(&commands) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub fn clean(&self) -> Result<(), ()> {
        self.del_container_network()
            .expect("Could not delete container network");

        if self.exists_veth() {
            self.del_veth().expect("Could not delete veth peer");
        }

        if self.existed_namespace() {
            self.del_network_namespace()
                .expect("Could not delete network namespace");
        }
        Ok(())
    }
}

#[test]
#[ignore]
fn test_veth_new() {
    let network = generate_test_network();

    if network.bridge.existed() {
        network
            .bridge
            .del_bridge_ace0()
            .expect("Could not delete bridge");
    }

    network
        .bridge
        .add_bridge_ace0()
        .expect("faild create bridge ace0");

    assert!(network.add_network_namespace().is_ok());
    assert!(network.del_network_namespace().is_ok());

    assert!(network.add_veth().is_ok());
    assert!(network.del_veth().is_ok());

    network
        .bridge
        .del_bridge_ace0()
        .expect("faild create bridge ace0");
}

#[test]
#[ignore]
fn test_add_bridge() {
    let network = generate_test_network();
    let bridge_ip: IpAddr = "172.0.0.1".parse().unwrap();

    assert_eq!(network.bridge.name, "ace0".to_string());
    assert_eq!(network.bridge.ip, bridge_ip);

    assert!(network.bridge.existed());
    assert!(network.bridge.add_bridge_ace0().is_ok());
    assert!(network.bridge.existed());
    assert!(network.bridge.del_bridge_ace0().is_ok());
}

#[test]
#[ignore]
fn test_add_container_network() {
    let network = generate_test_network();

    network
        .bridge
        .add_bridge_ace0()
        .expect("failed create bridge ace0");

    network
        .add_network_namespace()
        .expect("failed adding network namespace");
    network.add_veth().expect("failed adding veth peer");

    assert!(network.add_container_network().is_ok());
    assert!(network.clean().is_ok())
}

fn generate_test_network() -> Network {
    Network::new(
        "test-ns".to_string(),
        Bridge::new(),
        "test_veth_host".to_string(),
        "test_veth_guest".to_string(),
        "172.0.0.2".parse().unwrap(),
    )
}
