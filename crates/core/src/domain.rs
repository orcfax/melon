use melon_crypto::bls::{G1Point, hash_to_curve::hash_to_g1_dst};
use minicbor::{Decode, Encode};

use crate::{Id, blake3_cbor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, Hash)]
pub enum Kind {
    #[n(0)]
    Block,
    #[n(1)]
    Sys,
    #[n(2)]
    App,
}

pub trait Domain: Encode<()> {
    const DST: &'static [u8];
    const KIND: Kind;

    fn id(&self) -> Id {
        blake3_cbor(&self)
    }

    fn to_g1(&self) -> G1Point {
        hash_to_g1_dst(self.id().as_ref(), Self::DST)
    }
}
