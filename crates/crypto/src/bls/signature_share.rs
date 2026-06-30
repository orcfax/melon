use bls12_381::{G1Projective, Scalar};
use minicbor::{Decode, Encode};

use super::cbor;

/// A share of the secret
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct SignatureShare {
    #[cbor(n(0))]
    id: u64,
    #[cbor(n(1), with = "cbor::g1_projective")]
    signature: G1Projective,
}

impl SignatureShare {
    pub fn new(id: u64, signature: G1Projective) -> Self {
        Self { id, signature }
    }
    pub fn id_u64(&self) -> u64 {
        self.id
    }

    pub fn id(&self) -> Scalar {
        Scalar::from(self.id)
    }

    pub fn signature(&self) -> G1Projective {
        self.signature
    }
}

impl From<(&u64, &G1Projective)> for SignatureShare {
    fn from(value: (&u64, &G1Projective)) -> Self {
        Self::new(*value.0, *value.1)
    }
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum AggregationError {
    #[error("Not enough signature. Expected at least {expected}, got {got}.")]
    NotEnoughSignatures { expected: usize, got: usize },
    #[error("Duplicate signature id {id}")]
    DuplicateId { id: usize },
}

pub fn aggregate(signatures: &[SignatureShare]) -> Result<G1Projective, AggregationError> {
    for (i, item_i) in signatures.iter().enumerate() {
        for item_j in signatures.iter().skip(i + 1) {
            if item_i.id == item_j.id {
                return Err(AggregationError::DuplicateId { id: i });
            }
        }
    }
    let mut final_signature = G1Projective::identity();
    for (i, item_i) in signatures.iter().enumerate() {
        let (x_i, sig_i) = (item_i.id(), item_i.signature());
        let mut num = Scalar::one();
        let mut den = Scalar::one();
        for (j, item_j) in signatures.iter().enumerate() {
            if i == j {
                continue;
            }
            let x_j = item_j.id();
            num *= x_j;
            den *= x_j - x_i;
        }
        let inv_den = den.invert().unwrap();
        let lagrange_coeff = num * inv_den;
        final_signature += sig_i * lagrange_coeff;
    }
    Ok(final_signature)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bls::hash_to_curve::hash_to_g1;
    use bls12_381::{G1Projective, G2Projective};

    /// Build a proper Shamir (threshold, n) setup.
    ///
    /// Polynomial: f(x) = a_0 + a_1*x + ... + a_{t-1}*x^{t-1}  (degree t-1)
    /// Secret = f(0) = a_0; vk = G2 * secret.
    /// Returns `n_shares` shares evaluated at x = 1..=n_shares.
    fn toy_setup(
        n_shares: usize,
        threshold: usize,
    ) -> (Vec<SignatureShare>, G2Projective, G1Projective) {
        use ff::Field;
        use rand::rng;
        let mut rng = rng();
        let msg_hash = hash_to_g1(b"test message");
        let coeffs: Vec<Scalar> = (0..threshold).map(|_| Scalar::random(&mut rng)).collect();
        let eval = |x: Scalar| -> Scalar {
            let mut acc = Scalar::zero();
            let mut xpow = Scalar::one();
            for c in &coeffs {
                acc += c * xpow;
                xpow *= x;
            }
            acc
        };
        let shares = (1..=(n_shares as u64))
            .map(|i| {
                let sk = eval(Scalar::from(i));
                SignatureShare::new(i, msg_hash * sk)
            })
            .collect();
        let vk = G2Projective::generator() * coeffs[0];
        (shares, vk, msg_hash)
    }

    #[test]
    fn test_aggregate_threshold_met_5_3() {
        use crate::bls::verify::verify;
        let (shares, vk, msg_hash) = toy_setup(3, 3);
        let agg = aggregate(&shares).expect("Aggregation failed");
        assert!(verify(&vk, &msg_hash, &agg));
    }

    #[test]
    fn test_aggregate_threshold_met_10_7() {
        use crate::bls::verify::verify;
        let (shares, vk, msg_hash) = toy_setup(7, 7);
        let agg = aggregate(&shares).expect("Aggregation failed");
        assert!(verify(&vk, &msg_hash, &agg));
    }

    #[test]
    fn test_aggregate_threshold_unmet_10_7() {
        use crate::bls::verify::verify;
        // Only 6 shares from a degree-6 polynomial (threshold=7) — cannot recover secret
        let (shares, vk, msg_hash) = toy_setup(6, 7);
        let agg = aggregate(&shares).expect("Aggregation failed");
        assert!(!verify(&vk, &msg_hash, &agg));
    }
}
