use bls12_381::{G1Affine, G1Projective};

use minicbor::{
    decode::{self, Decoder},
    encode::{Encoder, Error, Write},
};

pub fn encode<C, W: Write>(
    key: &G1Projective,
    e: &mut Encoder<W>,
    _ctx: &mut C,
) -> Result<(), Error<W::Error>> {
    e.bytes(G1Affine::from(key).to_compressed().as_slice())?;
    Ok(())
}

pub fn decode<C>(d: &mut Decoder<'_>, _ctx: &mut C) -> Result<G1Projective, decode::Error> {
    let bytes = d.bytes()?;
    let array: [u8; 48] = bytes
        .try_into()
        .map_err(|_| decode::Error::message("invalid byte length for G1Projective"))?;
    let affine: G1Affine = Option::from(G1Affine::from_compressed(&array)).ok_or(
        decode::Error::message("invalid byte representation for G1Affine"),
    )?;
    Ok(G1Projective::from(affine))
}
