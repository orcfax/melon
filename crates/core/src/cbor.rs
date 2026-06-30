use std::convert::Infallible;

use minicbor::{decode, encode};
pub use minicbor::{decode::Decode, encode::Encode};

pub trait ToCbor {
    fn to_cbor(&self) -> Vec<u8>;
}

impl<T: Encode<()>> ToCbor for T {
    fn to_cbor(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let _: Result<(), encode::Error<Infallible>> = encode(self, &mut bytes);
        bytes
    }
}

pub trait FromCbor<'d> {
    fn from_cbor(bytes: &'d [u8]) -> Result<Self, decode::Error>
    where
        Self: Sized;
}

impl<'d, T: Decode<'d, ()>> FromCbor<'d> for T {
    fn from_cbor(bytes: &'d [u8]) -> Result<Self, decode::Error> {
        decode(bytes)
    }
}
