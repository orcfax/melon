use ed25519_dalek::{self as ext, Verifier};
use minicbor::{Decode, Encode};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::{Signature, signing_key::SigningKey};

pub const KEY_LEN: usize = ext::PUBLIC_KEY_LENGTH; // 32

#[serde_as]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, Encode, Decode,
)]
#[repr(transparent)]
#[cbor(transparent)]
#[serde(transparent)]
pub struct VerifyingKey(
    #[cbor(with = "minicbor::bytes")]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub [u8; KEY_LEN],
);

impl VerifyingKey {
    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        ext::VerifyingKey::from_bytes(&self.0)
            .unwrap()
            .verify(message, &ext::Signature::from_bytes(&signature.0))
            .is_ok()
    }
}

impl From<SigningKey> for VerifyingKey {
    fn from(value: SigningKey) -> Self {
        Self::from(ext::SigningKey::from_bytes(&value.0).verifying_key())
    }
}

impl From<ext::VerifyingKey> for VerifyingKey {
    fn from(value: ext::VerifyingKey) -> Self {
        Self(value.to_bytes())
    }
}

impl TryFrom<&[u8]> for VerifyingKey {
    type Error = std::array::TryFromSliceError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(<[u8; KEY_LEN]>::try_from(slice)?))
    }
}

impl AsRef<[u8]> for VerifyingKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; KEY_LEN]> for VerifyingKey {
    fn from(bytes: [u8; KEY_LEN]) -> Self {
        Self(bytes)
    }
}

impl From<VerifyingKey> for [u8; KEY_LEN] {
    fn from(vk: VerifyingKey) -> Self {
        vk.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_test_config() -> VerifyingKey {
        VerifyingKey([1; 32])
    }

    #[test]
    fn test_cbor_config_roundtrip() {
        let original = create_test_config();
        let buffer = minicbor::to_vec(&original).expect("CBOR encoding failed");
        let decoded: VerifyingKey = minicbor::decode(&buffer).expect("CBOR decoding failed");
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_serde_config_roundtrip() {
        let original = create_test_config();
        let json_data = serde_json::to_string(&original).expect("Serde serialization failed");
        let decoded: VerifyingKey =
            serde_json::from_str(&json_data).expect("Serde deserialization failed");
        assert_eq!(original, decoded);
        let expected_peer_key_hex =
            "0101010101010101010101010101010101010101010101010101010101010101";
        println!("{:?}", json_data);
        assert!(json_data.contains(expected_peer_key_hex));
    }
}
