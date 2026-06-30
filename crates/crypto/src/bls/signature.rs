use bls12_381::{G1Affine, G1Projective};
use minicbor::{Decode, Decoder, Encode, Encoder, decode, encode};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// An independent BLS signature (not a TSS share).
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Signature(#[serde_as(as = "super::serde::g1_projective::Format")] pub G1Projective);

impl From<G1Projective> for Signature {
    fn from(value: G1Projective) -> Self {
        Self(value)
    }
}

impl Signature {
    pub fn key(&self) -> G1Projective {
        self.0
    }

    pub fn to_compressed(&self) -> [u8; 48] {
        G1Affine::from(self.0).to_compressed()
    }
}

impl<C> Encode<C> for Signature {
    fn encode<W: encode::Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), encode::Error<W::Error>> {
        super::cbor::g1_projective::encode(&self.0, e, &mut ())
    }
}

impl<'b, C> Decode<'b, C> for Signature {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, decode::Error> {
        super::cbor::g1_projective::decode(d, &mut ()).map(Self)
    }
}

// The idiomatic Deref implementation
impl std::ops::Deref for Signature {
    type Target = G1Projective;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Add for Signature {
    type Output = Signature;
    fn add(self, rhs: Signature) -> Signature {
        Signature(self.0 + rhs.0)
    }
}

impl std::ops::Add<&Signature> for Signature {
    type Output = Signature;
    fn add(self, rhs: &Signature) -> Signature {
        Signature(self.0 + rhs.0)
    }
}

impl std::ops::Add<&Signature> for &Signature {
    type Output = Signature;
    fn add(self, rhs: &Signature) -> Signature {
        Signature(self.0 + rhs.0)
    }
}

impl std::ops::Add<Signature> for &Signature {
    type Output = Signature;
    fn add(self, rhs: Signature) -> Signature {
        Signature(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Signature {
    fn add_assign(&mut self, rhs: Signature) {
        self.0 += rhs.0;
    }
}

impl std::ops::AddAssign<&Signature> for Signature {
    fn add_assign(&mut self, rhs: &Signature) {
        self.0 += rhs.0;
    }
}

impl std::iter::Sum for Signature {
    fn sum<I: Iterator<Item = Signature>>(iter: I) -> Self {
        iter.fold(Signature(G1Projective::identity()), |acc, s| acc + s)
    }
}

impl<'a> std::iter::Sum<&'a Signature> for Signature {
    fn sum<I: Iterator<Item = &'a Signature>>(iter: I) -> Self {
        iter.fold(Signature(G1Projective::identity()), |acc, s| acc + s)
    }
}
