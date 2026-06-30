use libp2p::identity::{
    Keypair,
    ed25519::{self, SecretKey},
};
use melon_crypto::ed25519::SigningKey;

pub fn from_signing_key(key: &SigningKey) -> Keypair {
    let mut x = key.0;
    ed25519::Keypair::from(SecretKey::try_from_bytes(&mut x).expect("Secret key failed!")).into()
}
