use libp2p::{Multiaddr, PeerId, multiaddr::Protocol};
use melon_core::{
    Members, node,
    time::{Duo, Trio},
};
use melon_crypto::ed25519;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub key: ed25519::SigningKey,
    pub listen_on: Option<Multiaddr>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bootstrap: Vec<Multiaddr>,
    /// The time of the epoch from which we are starting.
    pub time: Trio,
    /// The validator set of the epoch
    pub members: Members,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("invalid multiaddr: {0}")]
    Addr(#[from] libp2p::multiaddr::Error),
    #[error("invalid peer id: {0}")]
    PeerId(#[from] node::id::Error),
}

impl TryFrom<melon_core::Config> for Config {
    type Error = ConfigError;

    fn try_from(c: melon_core::Config) -> Result<Self, Self::Error> {
        let listen_on = c.addr.parse()?;
        let bootstrap = c
            .bootstrap
            .iter()
            .map(|(addr, id)| {
                let mut addr: Multiaddr = addr.parse()?;
                let peer_id: PeerId = PeerId::try_from(id.clone())?;
                addr.push(Protocol::P2p(peer_id));
                Ok(addr)
            })
            .collect::<Result<Vec<_>, ConfigError>>()?;
        let Duo { period, height } = c.manifest.period_coord;
        let time = Trio {
            period,
            epoch: 0,
            height,
        };
        Ok(Self {
            key: c.secrets.net.clone(),
            listen_on: Some(listen_on),
            bootstrap,
            time,
            members: c.manifest.members.clone(),
        })
    }
}
