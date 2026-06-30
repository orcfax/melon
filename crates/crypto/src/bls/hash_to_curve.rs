use bls12_381::{
    G1Projective, G2Projective,
    hash_to_curve::{ExpandMsgXmd, HashToCurve},
};
use sha2::Sha256;

// FIXME :: Can we reuse this for both curves?
const DST: &[u8; 56] = b"BLS_SIG_BLS12381G1_XMD:SHA-256_SSWU_RO_THRESHOLD_EXAMPLE";

pub fn hash_to_g1(msg: &[u8]) -> G1Projective {
    <G1Projective as HashToCurve<ExpandMsgXmd<Sha256>>>::hash_to_curve([msg], DST)
}

pub fn hash_to_g1_dst(msg: &[u8], dst: &[u8]) -> G1Projective {
    <G1Projective as HashToCurve<ExpandMsgXmd<Sha256>>>::hash_to_curve([msg], dst)
}

pub fn hash_to_g2(msg: &[u8]) -> G2Projective {
    <G2Projective as HashToCurve<ExpandMsgXmd<Sha256>>>::hash_to_curve([msg], DST)
}
