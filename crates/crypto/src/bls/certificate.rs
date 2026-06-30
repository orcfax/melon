use minicbor::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::bls::{G1Point, Signature, VerifyingKey};

use super::Bitmask;

/// A Quorum Certificate: a bitmask of signers plus their aggregated signature.
///
/// The bitmask indexes into the Registry that was active when this certificate
/// was produced (a snapshot taken at the start of the round). The signature is
/// the plain sum of the individual BLS signatures for the set bits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
#[cbor(array)]
pub struct Certificate {
    #[n(0)]
    pub bitmask: Bitmask,
    #[n(1)]
    pub signature: Signature,
}

impl Certificate {
    pub fn new(capacity: usize, pos: usize, signature: Signature) -> Self {
        let mut bitmask = Bitmask::with_capacity(capacity);
        bitmask.set(pos);
        Self { bitmask, signature }
    }

    pub fn has(&self, pos: usize) -> bool {
        self.bitmask.get(pos)
    }

    pub fn add(&mut self, pos: usize, signature: Signature) {
        self.bitmask.set(pos);
        self.signature += signature;
    }

    /// Build a certificate from a set of (registry-position, signature) pairs.
    pub fn make(entries: &[(usize, Signature)]) -> Self {
        let mut bitmask = Bitmask::empty();
        let signature = entries
            .iter()
            .map(|(pos, sig)| {
                bitmask.set(*pos);
                sig
            })
            .sum();
        Self { bitmask, signature }
    }

    /// Verify this certificate against a message hash and the ordered registry keys.
    ///
    /// `registry_keys` must be the snapshot of `[bls::VerifyingKey]` that was
    /// active when this certificate was produced, in the same order.
    pub fn verify(&self, msg_hash: &G1Point, registry_keys: &[VerifyingKey]) -> bool {
        let key: VerifyingKey = self
            .bitmask
            .positions()
            .filter_map(|pos| registry_keys.get(pos).cloned())
            .sum();
        // FIXME :: Reinstate this check when we know how to.
        // if keys.len() != self.bitmask.count() as usize {
        //     return false;
        // }
        key.verify(&msg_hash, &self.signature)
    }

    pub fn signer_count(&self) -> u32 {
        self.bitmask.count()
    }

    pub fn join(&self, other: &Certificate) -> Result<Certificate, JoinError> {
        // Overlap = any bit set in both masks
        if self.bitmask.overlaps(&other.bitmask) {
            return Err(JoinError::OverlappingSigners);
        }
        Ok(Certificate {
            bitmask: self.bitmask.union(&other.bitmask),
            signature: self.signature.clone() + other.signature.clone(),
        })
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum JoinError {
    #[error("Overlap in signers is not allowed")]
    OverlappingSigners, // same position set in both bitmasks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bls::{SigningKey, hash_to_curve::hash_to_g1};
    use rand::rng;

    #[test]
    fn test_make_verify() {
        let mut rng = rng();
        let keys: Vec<SigningKey> = (0..5).map(|_| SigningKey::generate(&mut rng)).collect();
        let vks: Vec<VerifyingKey> = keys.iter().map(|k| k.verifying_key()).collect();
        let msg = b"test QC message";
        let msg_hash = hash_to_g1(msg);
        // Signers at positions 0, 2, 4
        let entries: Vec<(usize, Signature)> = [0usize, 2, 4]
            .iter()
            .map(|&i| (i, keys[i].sign(&msg_hash)))
            .collect();
        let cert = Certificate::make(&entries);
        assert_eq!(cert.signer_count(), 3);
        assert!(cert.verify(&msg_hash, &vks));
    }

    #[test]
    fn test_wrong_keys_fail() {
        let mut rng = rng();
        let keys: Vec<SigningKey> = (0..5).map(|_| SigningKey::generate(&mut rng)).collect();
        let wrong_keys: Vec<VerifyingKey> = (0..5)
            .map(|_| SigningKey::generate(&mut rng).verifying_key())
            .collect();
        let msg_hash = hash_to_g1(b"test");
        let entries: Vec<(usize, Signature)> = [0usize, 1]
            .iter()
            .map(|&i| (i, keys[i].sign(&msg_hash)))
            .collect();
        let cert = Certificate::make(&entries);
        assert!(!cert.verify(&msg_hash, &wrong_keys));
    }
}
