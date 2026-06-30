use bls12_381::Scalar;

use minicbor::{
    decode::{self, Decoder},
    encode::{Encoder, Error, Write},
};

pub fn encode<C, W: Write>(
    scalar: &Scalar,
    e: &mut Encoder<W>,
    _ctx: &mut C,
) -> Result<(), Error<W::Error>> {
    // Convert the Scalar to its 32-byte representation and encode as a byte string
    e.bytes(&scalar.to_bytes())?;
    Ok(())
}

pub fn decode<C>(d: &mut Decoder<'_>, _ctx: &mut C) -> Result<Scalar, decode::Error> {
    // Decode the byte string
    let bytes = d.bytes()?;
    // Attempt to convert the byte slice into a 32-byte array
    let array: [u8; 32] = bytes
        .try_into()
        .map_err(|_| decode::Error::message("invalid byte length for Scalar"))?;

    // Convert from bytes back to Scalar, handling the CtOption
    Option::from(Scalar::from_bytes(&array)).ok_or(decode::Error::message(
        "invalid byte representation for Scalar",
    ))
}
