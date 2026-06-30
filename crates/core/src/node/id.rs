use std::fmt;

use ed25519_dalek::VerifyingKey;
use libp2p::PeerId;
use libp2p::identity::ed25519::PublicKey as Libp2pPublicKey;
use minicbor::{Decode, Decoder, Encode, Encoder, decode, encode};
use serde::{Deserialize, Serialize};

mod error;
pub use error::Error;
mod external;

use external::{dalek_to_libp2p, try_verifying_key_from_peer_id};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id {
    pub id: [u8; 32],
}

impl Serialize for Id {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        if s.is_human_readable() {
            s.serialize_str(&hex::encode(self.id))
        } else {
            s.serialize_bytes(&self.id)
        }
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        let bytes = if d.is_human_readable() {
            hex::decode(String::deserialize(d)?).map_err(D::Error::custom)?
        } else {
            Vec::<u8>::deserialize(d)?
        };
        let id: [u8; 32] = bytes.try_into().map_err(|v: Vec<u8>| {
            D::Error::custom(format!("expected 32 bytes, got {}", v.len()))
        })?;
        Ok(Self { id })
    }
}

impl<C> Encode<C> for Id {
    fn encode<W: encode::Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), encode::Error<W::Error>> {
        e.bytes(&self.id)?;
        Ok(())
    }
}

impl<'b, C> Decode<'b, C> for Id {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, decode::Error> {
        let bytes = d.bytes()?;
        let id: [u8; 32] = bytes
            .try_into()
            .map_err(|_| decode::Error::message("expected 32 bytes for node::Id"))?;
        Ok(Self { id })
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.id {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl From<VerifyingKey> for Id {
    fn from(value: VerifyingKey) -> Self {
        Self::from(value.to_bytes())
    }
}

impl TryFrom<Id> for VerifyingKey {
    type Error = Error;
    fn try_from(value: Id) -> Result<Self, Self::Error> {
        let vk = Self::from_bytes(value.as_ref())?;
        Ok(vk)
    }
}

impl From<melon_crypto::ed25519::VerifyingKey> for Id {
    fn from(value: melon_crypto::ed25519::VerifyingKey) -> Self {
        Self::from(value.0)
    }
}

impl From<Id> for melon_crypto::ed25519::VerifyingKey {
    fn from(value: Id) -> Self {
        Self::from(value.id)
    }
}

impl AsRef<[u8]> for Id {
    fn as_ref(&self) -> &[u8] {
        &self.id
    }
}

impl From<[u8; 32]> for Id {
    fn from(value: [u8; 32]) -> Self {
        Self { id: value }
    }
}

impl AsRef<[u8; 32]> for Id {
    fn as_ref(&self) -> &[u8; 32] {
        &self.id
    }
}

impl TryFrom<Id> for Libp2pPublicKey {
    type Error = Error;
    fn try_from(value: Id) -> Result<Self, Self::Error> {
        let x = Self::try_from_bytes(&value.id)?;
        Ok(x)
    }
}

impl From<Libp2pPublicKey> for Id {
    fn from(value: Libp2pPublicKey) -> Self {
        Self::from(value.to_bytes())
    }
}

impl TryFrom<PeerId> for Id {
    type Error = Error;
    fn try_from(value: PeerId) -> Result<Self, Self::Error> {
        try_verifying_key_from_peer_id(value).map(Self::from)
    }
}

impl TryFrom<Id> for PeerId {
    type Error = Error;
    fn try_from(value: Id) -> Result<Self, Self::Error> {
        let vk = VerifyingKey::try_from(value)?;
        Ok(PeerId::from_public_key(&dalek_to_libp2p(&vk).into()))
    }
}
