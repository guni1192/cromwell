use std::net::IpAddr;
use std::process::{Child, Command};

use super::commands;

struct Bridge {
    name: String,
    ip: IpAddr,
}

pub struct Network {
    veth_guest: String,
    veth_host: String,
    namespace: String,
    bridge: Bridge,
}

impl Bridge {
    pub fn new() -> Bridge {
        Bridge {
            name: "ace0".to_string(),
            ip: "172.0.0.1".parse().unwrap(),
        }
    }
    pub fn add_bridge_ace0(&self) -> std::io::Result<Child> {
        Command::new("ip")
            .args(&["link", "add", "name", self.name.as_str(), "type", "bridge"])
            .spawn()
    }
    pub fn del_bridge_ace0(&self) -> std::io::Result<Child> {
        Command::new("ip")
            .args(&["link", "del", "name", self.name.as_str()])
            .spawn()
    }
}

impl Network {
    pub fn new(namespace: String, veth_host: String, veth_guest: String) -> Network {
        Network {
            namespace: namespace,
            bridge: Bridge::new(),
            veth_host: veth_host,
            veth_guest: veth_guest,
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

    pub fn add_veth(&self) -> Result<String, String> {
        let status = Command::new("ip")
            .args(&[
                "link",
                "add",
                self.veth_host.as_str(),
                "type",
                "veth",
                "peer",
                "name",
                self.veth_guest.as_str(),
            ])
            .status()
            .expect("");
        if status.success() {
            Ok("".to_string())
        } else {
            Err("".to_string())
        }
    }
    pub fn del_veth(&self) -> Result<String, String> {
        let status = Command::new("ip")
            .args(&["link", "del", self.veth_host.as_str()])
            .status()
            .expect("");
        if status.success() {
            Ok("".to_string())
        } else {
            Err("".to_string())
        }
    }
}

// CIだとrootでテストできないから[ignore]に設定
// ローカルでテストするなら
// $ sudo cargo test -- --ignored

#[test]
#[ignore]
fn test_veth_new() {
    let network = Network::new(
        "test-ns".to_string(),
        "test_veth_host".to_string(),
        "test_veth_guest".to_string(),
    );

    assert!(network.add_network_namespace().is_ok());
    assert!(network.del_network_namespace().is_ok());

    assert!(network.add_veth().is_ok());
    assert!(network.del_veth().is_ok());
}

#[test]
#[ignore]
fn test_add_bridge() {
    let network = Network::new(
        "test-ns".to_string(),
        "test_veth_host".to_string(),
        "test_veth_guest".to_string(),
    );

    let bridge_ip: IpAddr = "172.0.0.1".parse().unwrap();

    assert_eq!(network.bridge.name, "ace0".to_string());
    assert_eq!(network.bridge.ip, bridge_ip);

    assert!(network.bridge.add_bridge_ace0().is_ok());
    assert!(network.bridge.del_bridge_ace0().is_ok());
}
