use std::sync::Arc;

use melon_core::node;
use melon_crypto::bls::Certificate;
use tokio::sync::RwLock;

#[derive(Debug, Default, Clone)]
pub struct NodeState {
    pub latest_qc: Option<Certificate>,
    pub peers: Vec<node::Id>,
}

pub type SharedState = Arc<RwLock<NodeState>>;

pub fn new_shared() -> SharedState {
    Arc::new(RwLock::new(NodeState::default()))
}
