use bls12_381::{G1Projective, G2Projective, Scalar};
use ff::Field;
use minicbor::{Decode, Decoder, Encode, Encoder, decode, encode};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::bls::Signature;

use super::VerifyingKey;

/// An independent BLS signing key (not a TSS share).
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SigningKey(#[serde_as(as = "super::serde::scalar::Format")] Scalar);

impl SigningKey {
    pub fn generate<R: Rng>(rng: &mut R) -> Self {
        Self(Scalar::random(rng))
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        VerifyingKey(G2Projective::generator() * self.0)
    }

    pub fn sign(&self, msg_hash: &G1Projective) -> Signature {
        (msg_hash * self.0).into()
    }
}

impl<C> Encode<C> for SigningKey {
    fn encode<W: encode::Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), encode::Error<W::Error>> {
        super::cbor::scalar::encode(&self.0, e, &mut ())
    }
}

impl<'b, C> Decode<'b, C> for SigningKey {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, decode::Error> {
        super::cbor::scalar::decode(d, &mut ()).map(Self)
    }
}

// The idiomatic Deref implementation
impl std::ops::Deref for SigningKey {
    type Target = Scalar;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
