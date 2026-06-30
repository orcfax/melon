use melon_crypto::bls::{G1Point, Signature, SigningKey, VerifyingKey};
use parking_lot::RwLock;

pub struct Signer {
    key: RwLock<SigningKey>,
}

impl Signer {
    pub fn new(key: SigningKey) -> Self {
        Self {
            key: RwLock::new(key),
        }
    }

    pub fn sign(&self, point: &G1Point) -> Signature {
        self.key.read().sign(point)
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.key.read().verifying_key()
    }

    pub fn rotate(&self, new_key: SigningKey) {
        *self.key.write() = new_key;
    }
}
