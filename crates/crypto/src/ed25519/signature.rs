use ed25519_dalek as ext;
use minicbor::{Decode, Encode};
use serde::{Deserialize, Serialize};

pub const SIG_LEN: usize = ext::SIGNATURE_LENGTH; // 64

pub mod hex_64_byte_array {
    use hex::{FromHex, ToHex};
    use serde::{Deserialize, Deserializer, Serializer, de};

    pub fn serialize<S>(data: &[u8; 64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&data.encode_hex::<String>())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)
            .and_then(|s| <[u8; 64]>::from_hex(s).map_err(de::Error::custom))
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, Encode, Decode,
)]
#[repr(transparent)]
#[cbor(transparent)]
pub struct Signature(
    #[cbor(with = "minicbor::bytes")]
    #[serde(with = "hex_64_byte_array")]
    pub [u8; SIG_LEN],
);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_test_config() -> Signature {
        Signature([1; 64])
    }

    #[test]
    fn test_cbor_config_roundtrip() {
        let original = create_test_config();
        let buffer = minicbor::to_vec(&original).expect("CBOR encoding failed");
        let decoded: Signature = minicbor::decode(&buffer).expect("CBOR decoding failed");
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_serde_config_roundtrip() {
        let original = create_test_config();
        let json_data = serde_json::to_string(&original).expect("Serde serialization failed");
        let decoded: Signature =
            serde_json::from_str(&json_data).expect("Serde deserialization failed");
        assert_eq!(original, decoded);
        let expected_peer_key_hex = "01010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101";
        println!("{:?}", json_data);
        assert!(json_data.contains(expected_peer_key_hex));
    }
}
