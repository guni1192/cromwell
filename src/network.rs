use std::process::{Child, Command};

struct Network {
    veth_guest: String,
    veth_host: String,
    bridge_name: String,
    namespace: String,
}

impl Network {
    pub fn new(
        namespace: String,
        bridge_name: String,
        veth_host: String,
        veth_guest: String,
    ) -> Network {
        Network {
            namespace: namespace,
            bridge_name: bridge_name,
            veth_host: veth_host,
            veth_guest: veth_guest,
        }
    }

    fn add_network_namespace(&self) -> Result<String, String> {
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
    fn add_veth(&self) -> Result<String, String> {
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
    fn del_veth(&self) -> Result<String, String> {
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

pub fn make_bridge_ace0() -> std::io::Result<Child> {
    // TODO: IPコマンドを使わない
    Command::new("ip")
        .args(&["link", "add", "name", "ace0", "type", "bridge"])
        .spawn()
}
pub fn delete_bridge_ace0() -> std::io::Result<Child> {
    // TODO: IPコマンドを使わない
    Command::new("ip")
        .args(&["link", "del", "name", "ace0"])
        .spawn()
}

#[test]
#[ignore]
fn test_veth_new() {
    let network = Network::new(
        "test-ns".to_string(),
        "ace0".to_string(),
        "test_veth_host".to_string(),
        "test_veth_guest".to_string(),
    );

    assert!(network.add_veth().is_ok());
    assert!(network.del_veth().is_ok());
}
