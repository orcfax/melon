use melon_crypto::{bls, ed25519};
use minicbor::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::node;

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Register {
    #[n(0)]
    id: node::Id,
    #[n(1)]
    key: bls::VerifyingKey,
    #[n(2)]
    addr: String,
}

impl Register {
    pub fn new(id: node::Id, key: bls::VerifyingKey, addr: String) -> Self {
        Self { id, key, addr }
    }

    /// Derive registration from secrets. The `node::Id` is derived from
    /// the network key, the verifying key from the signing key.
    pub fn make(
        signing_key: &bls::SigningKey,
        network_key: &ed25519::SigningKey,
        addr: String,
    ) -> Self {
        Self {
            id: node::Id::from(network_key.verifying_key()),
            key: signing_key.verifying_key(),
            addr,
        }
    }

    pub fn id(&self) -> node::Id {
        self.id
    }

    pub fn key(&self) -> &bls::VerifyingKey {
        &self.key
    }

    pub fn addr(&self) -> &str {
        &self.addr
    }
}
