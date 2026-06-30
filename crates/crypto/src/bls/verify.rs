use bls12_381::{G1Projective, G2Projective, pairing};

pub fn verify(key: &G2Projective, msg_hash: &G1Projective, signature: &G1Projective) -> bool {
    pairing(&signature.into(), &G2Projective::generator().into())
        == pairing(&msg_hash.into(), &key.into())
}
