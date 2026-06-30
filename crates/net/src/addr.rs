use std::net::Ipv4Addr;

use libp2p::{Multiaddr, PeerId, multiaddr::Protocol};

pub fn extract_peer_id(addr: &Multiaddr) -> Result<PeerId, String> {
    addr.iter()
        .find_map(|p| match p {
            Protocol::P2p(peer_id) => Some(peer_id),
            _ => None,
        })
        .ok_or_else(|| "Multiaddr does not contain a PeerID".to_string())
}

// Helper for generating default configs
pub fn zero_addr(peer_id: PeerId) -> Multiaddr {
    addr_with_port(peer_id, 54321)
}

pub fn addr_with_port(peer_id: PeerId, port: u16) -> Multiaddr {
    let mut addr = Multiaddr::empty();
    addr.push(Protocol::Ip4(Ipv4Addr::new(127, 0, 0, 1)));
    addr.push(Protocol::Tcp(port));
    addr.push(Protocol::P2p(peer_id));
    addr
}

pub fn default_addr() -> Multiaddr {
    "/ip4/0.0.0.0/tcp/0".parse().unwrap()
}
