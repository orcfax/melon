use std::time::Duration;

use libp2p::{PeerId, gossipsub, identify, kad, noise, tcp, yamux};

use super::behaviour::Behaviour;

pub struct Swarm(pub libp2p::Swarm<Behaviour>);

const APP_PROTOCOL: &str = "/app/0.0.0";

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("Init :: {0}")]
    Init(String),
}

impl Swarm {
    pub fn new(local_key: libp2p::identity::Keypair) -> Result<Swarm, Error> {
        let peer_id = PeerId::from(local_key.public());
        // IDENTIFY
        let identify = identify::Behaviour::new(identify::Config::new(
            APP_PROTOCOL.into(),
            local_key.public(),
        ));
        // KAD
        let kad_store = kad::store::MemoryStore::new(peer_id);
        let kademlia = kad::Behaviour::new(peer_id, kad_store);
        // GOSSIP
        let gossip_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(1))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .duplicate_cache_time(Duration::from_secs(60))
            .build()
            .map_err(|msg| Error::Init(format!("Gossipsub config error: {}", msg)))?;
        let gossip = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossip_config,
        )
        .map_err(|msg| Error::Init(format!("Gossipsub behaviour error: {}", msg)))?;
        // SWARM
        let swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )
            .map_err(|msg| Error::Init(format!("swarm tcp error: {}", msg)))?
            .with_behaviour(|_keypair| Behaviour {
                identify,
                kademlia,
                gossip,
            })
            .map_err(|msg| Error::Init(format!("behaviour error: {}", msg)))?
            .build();
        Ok(Self(swarm))
    }
}
