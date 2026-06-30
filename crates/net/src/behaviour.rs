use libp2p::{gossipsub, identify, kad, swarm::NetworkBehaviour};

#[derive(NetworkBehaviour)]
pub struct Behaviour {
    pub identify: identify::Behaviour,
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
    pub gossip: gossipsub::Behaviour,
}
