struct Veth {
    namespace: String,
    address: String,
}

struct VethPeer {
    host_veth: Veth,
    guest_veth: Veth,
}

impl Veth {
    pub fn new(namespace: String, address: String) -> Veth {
        Veth {
            namespace: namespace,
            address: address,
        }
    }
}

impl VethPeer {
    pub fn new(host_veth: Veth, guest_veth: Veth) -> VethPeer {
        VethPeer {
            guest_veth: guest_veth,
            host_veth: host_veth,
        }
    }
}

#[test]
fn test_veth_new() {
    let namespace = "ns00";
    let address = "127.0.0.1";

    let veth = Veth::new(namespace.to_string(), address.to_string());

    assert_eq!(namespace, veth.namespace);
    assert_eq!(address, veth.address);
}

#[test]
fn test_veth_peer_new() {
    let host_namespace = "ns00";
    let host_address = "127.0.0.1";

    let guest_namespace = "ns01";
    let guest_address = "10.0.0.1";

    let host_veth = Veth::new(host_namespace.to_string(), host_address.to_string());
    let guest_veth = Veth::new(guest_namespace.to_string(), guest_address.to_string());

    let veth_peer = VethPeer::new(host_veth, guest_veth);

    assert_eq!(guest_namespace, veth_peer.guest_veth.namespace);
    assert_eq!(guest_address, veth_peer.guest_veth.address);

    assert_eq!(host_namespace, veth_peer.host_veth.namespace);
    assert_eq!(host_address, veth_peer.host_veth.address);
}
