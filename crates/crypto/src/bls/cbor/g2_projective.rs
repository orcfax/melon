use bls12_381::{G2Affine, G2Projective};

use minicbor::{
    decode::{self, Decoder},
    encode::{Encoder, Error, Write},
};

pub fn encode<C, W: Write>(
    key: &G2Projective,
    e: &mut Encoder<W>,
    _ctx: &mut C,
) -> Result<(), Error<W::Error>> {
    e.bytes(G2Affine::from(key).to_compressed().as_slice())?;
    Ok(())
}

pub fn decode<C>(d: &mut Decoder<'_>, _ctx: &mut C) -> Result<G2Projective, decode::Error> {
    let bytes = d.bytes()?;
    let array: [u8; 96] = bytes
        .try_into()
        .map_err(|_| decode::Error::message("invalid byte length for G2Projective"))?;
    let affine: G2Affine = Option::from(G2Affine::from_compressed(&array)).ok_or(
        decode::Error::message("invalid byte representation for G2Affine"),
    )?;
    Ok(G2Projective::from(affine))
}
