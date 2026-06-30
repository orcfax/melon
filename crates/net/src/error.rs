use crate::swarm;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("Swarm :: {0}")]
    Swarm(#[from] swarm::Error),
    #[error("Gossip :: {0}")]
    Gossip(String),
    #[error("KademliaId :: Unexpect size")]
    KademliaId,
    #[error("Bus :: {0}")]
    Bus(String),
    #[error("Init :: {0}")]
    Init(String),
}
