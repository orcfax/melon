use melon_crypto::{bls, ed25519};
use minicbor::{Decode, Encode};
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

/// Node secrets
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Secrets {
    #[n(0)]
    pub net: ed25519::SigningKey,
    #[n(1)]
    pub member: Option<bls::SigningKey>,
}

impl Secrets {
    pub fn validator(net: ed25519::SigningKey, member: bls::SigningKey) -> Self {
        Self {
            net,
            member: Some(member),
        }
    }

    pub fn observer(net: ed25519::SigningKey) -> Self {
        Self { net, member: None }
    }

    pub fn id(&self) -> super::Id {
        super::Id::from(self.net.verifying_key())
    }

    pub fn is_validator(&self) -> bool {
        self.member.is_some()
    }

    /// Always generate both keys
    pub fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            net: ed25519::SigningKey::generate(rng),
            member: Some(bls::SigningKey::generate(rng)),
        }
    }

    /// Great for testing!
    pub fn from_seed(seed: usize) -> Self {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
        Self::generate(&mut rng)
    }
}
