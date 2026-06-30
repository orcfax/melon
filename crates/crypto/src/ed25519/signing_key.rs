use ed25519_dalek::{self as ext, Signer};
use minicbor::{Decode, Encode};
use num_traits::ToBytes;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::signature::Signature;

pub const KEY_LEN: usize = ext::PUBLIC_KEY_LENGTH; // 32

#[serde_as]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, Encode, Decode,
)]
#[repr(transparent)]
#[cbor(transparent)]
#[serde(transparent)]
pub struct SigningKey(
    #[cbor(with = "minicbor::bytes")]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub [u8; KEY_LEN],
);

impl SigningKey {
    pub fn from_seed(seed: usize) -> Self {
        Self(int_to_arr32(seed))
    }

    pub fn generate<T: Rng>(rng: &mut T) -> Self {
        let mut buf = [0u8; KEY_LEN];
        rng.fill_bytes(&mut buf);
        Self(buf)
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        Signature(
            ext::SigningKey::from_bytes(&self.0)
                .sign(message)
                .to_bytes(),
        )
    }

    pub fn verifying_key(&self) -> super::VerifyingKey {
        super::VerifyingKey::from(ext::SigningKey::from_bytes(&self.0).verifying_key())
    }
}

impl TryFrom<&[u8]> for SigningKey {
    type Error = std::array::TryFromSliceError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(<[u8; KEY_LEN]>::try_from(slice)?))
    }
}

impl AsRef<[u8]> for SigningKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<&SigningKey> for ext::SigningKey {
    fn from(value: &SigningKey) -> Self {
        ext::SigningKey::from_bytes(&value.0)
    }
}

fn int_to_arr32<T: ToBytes>(n: T) -> [u8; 32] {
    let mut arr = [0u8; 32];
    let bytes = n.to_be_bytes();
    let slice = bytes.as_ref();
    let offset = 32 - slice.len();
    arr[offset..].copy_from_slice(slice);
    arr
}

// FIXME :: lost Cargo.toml stanza for this
// #[cfg(feature = "test-utils")]
// use crate::generate::Generate;
//
// #[cfg(feature = "test-utils")]
// impl Generate for SigningKey {
//     fn generate_random<R: Rng>(rng: &mut R) -> Self {
//         Self::generate(rng)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_test_config() -> SigningKey {
        SigningKey([1; 32])
    }

    #[test]
    fn test_cbor_config_roundtrip() {
        let original = create_test_config();
        let buffer = minicbor::to_vec(&original).expect("CBOR encoding failed");
        let decoded: SigningKey = minicbor::decode(&buffer).expect("CBOR decoding failed");
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_serde_config_roundtrip() {
        let original = create_test_config();
        let json_data = serde_json::to_string(&original).expect("Serde serialization failed");
        let decoded: SigningKey =
            serde_json::from_str(&json_data).expect("Serde deserialization failed");
        assert_eq!(original, decoded);
        let expected_peer_key_hex =
            "0101010101010101010101010101010101010101010101010101010101010101";
        println!("{:?}", json_data);
        assert!(json_data.contains(expected_peer_key_hex));
    }
}
