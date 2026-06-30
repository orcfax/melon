use bls12_381::{G2Affine, G2Projective};
use minicbor::{Decode, Decoder, Encode, Encoder, decode, encode};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::bls::{G1Point, Signature};

/// An independent BLS verifying key (not a TSS share).
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VerifyingKey(#[serde_as(as = "super::serde::g2_projective::Format")] pub G2Projective);

impl std::hash::Hash for VerifyingKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        G2Affine::from(self.0).to_compressed().hash(state);
    }
}

impl VerifyingKey {
    pub fn key(&self) -> G2Projective {
        self.0
    }

    pub fn verify(&self, msg_hash: &G1Point, signature: &Signature) -> bool {
        super::verify::verify(&self.0, msg_hash, &**signature)
    }

    pub fn to_compressed(&self) -> [u8; 96] {
        use bls12_381::G2Affine;
        G2Affine::from(self.0).to_compressed()
    }
}

impl<C> Encode<C> for VerifyingKey {
    fn encode<W: encode::Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), encode::Error<W::Error>> {
        super::cbor::g2_projective::encode(&self.0, e, &mut ())
    }
}

impl<'b, C> Decode<'b, C> for VerifyingKey {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, decode::Error> {
        super::cbor::g2_projective::decode(d, &mut ()).map(Self)
    }
}

// The idiomatic Deref implementation
impl std::ops::Deref for VerifyingKey {
    type Target = G2Projective;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Add for VerifyingKey {
    type Output = VerifyingKey;
    fn add(self, rhs: VerifyingKey) -> VerifyingKey {
        VerifyingKey(self.0 + rhs.0)
    }
}

impl std::ops::Add<&VerifyingKey> for VerifyingKey {
    type Output = VerifyingKey;
    fn add(self, rhs: &VerifyingKey) -> VerifyingKey {
        VerifyingKey(self.0 + rhs.0)
    }
}

impl std::ops::Add<&VerifyingKey> for &VerifyingKey {
    type Output = VerifyingKey;
    fn add(self, rhs: &VerifyingKey) -> VerifyingKey {
        VerifyingKey(self.0 + rhs.0)
    }
}

impl std::ops::Add<VerifyingKey> for &VerifyingKey {
    type Output = VerifyingKey;
    fn add(self, rhs: VerifyingKey) -> VerifyingKey {
        VerifyingKey(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for VerifyingKey {
    fn add_assign(&mut self, rhs: VerifyingKey) {
        self.0 += rhs.0;
    }
}

impl std::ops::AddAssign<&VerifyingKey> for VerifyingKey {
    fn add_assign(&mut self, rhs: &VerifyingKey) {
        self.0 += rhs.0;
    }
}

impl std::iter::Sum for VerifyingKey {
    fn sum<I: Iterator<Item = VerifyingKey>>(iter: I) -> Self {
        iter.fold(VerifyingKey(G2Projective::identity()), |acc, s| acc + s)
    }
}

impl<'a> std::iter::Sum<&'a VerifyingKey> for VerifyingKey {
    fn sum<I: Iterator<Item = &'a VerifyingKey>>(iter: I) -> Self {
        iter.fold(VerifyingKey(G2Projective::identity()), |acc, s| acc + s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bls::{SigningKey, hash_to_curve::hash_to_g1};

    use rand::rng;

    #[test]
    fn test_aggregate_all_sign() {
        let mut rng = rng();
        let keys: Vec<SigningKey> = (0..5).map(|_| SigningKey::generate(&mut rng)).collect();
        let vks: Vec<VerifyingKey> = keys.iter().map(|k| k.verifying_key()).collect();
        let msg = b"melon test message";
        let msg_hash = hash_to_g1(msg);
        let sig: Signature = keys.iter().map(|k| k.sign(&msg_hash)).sum();
        let key: VerifyingKey = vks.iter().sum();
        assert!(key.verify(&msg_hash, &sig));
    }

    #[test]
    fn test_aggregate_subset_sign() {
        let mut rng = rng();
        let keys: Vec<SigningKey> = (0..5).map(|_| SigningKey::generate(&mut rng)).collect();
        let msg = b"melon test message";
        let msg_hash = hash_to_g1(msg);
        // Only first 3 sign
        let signers = &keys[..3];
        let vks: Vec<VerifyingKey> = signers.iter().map(|k| k.verifying_key()).collect();
        let sig: Signature = signers.iter().map(|k| k.sign(&msg_hash)).sum();
        let key: VerifyingKey = vks.iter().sum();
        assert!(key.verify(&msg_hash, &sig));
        // Wrong key set should not verify
        let wrong_vks: Vec<VerifyingKey> = keys[1..4].iter().map(|k| k.verifying_key()).collect();
        let wrong_key: VerifyingKey = wrong_vks.iter().sum();
        assert!(!wrong_key.verify(&msg_hash, &sig));
    }
}
