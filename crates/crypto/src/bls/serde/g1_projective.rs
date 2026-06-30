use bls12_381::{G1Affine, G1Projective};
use serde::{Deserialize, Deserializer, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

pub struct Format;

impl SerializeAs<G1Projective> for Format {
    fn serialize_as<S>(value: &G1Projective, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = G1Affine::from(value).to_compressed();
        if serializer.is_human_readable() {
            serializer.serialize_str(&hex::encode(bytes))
        } else {
            serializer.serialize_bytes(&bytes)
        }
    }
}

impl<'de> DeserializeAs<'de, G1Projective> for Format {
    fn deserialize_as<D>(deserializer: D) -> Result<G1Projective, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let bytes = if deserializer.is_human_readable() {
            hex::decode(String::deserialize(deserializer)?).map_err(Error::custom)?
        } else {
            Vec::<u8>::deserialize(deserializer)?
        };

        let array: [u8; 48] = bytes
            .try_into()
            .map_err(|v: Vec<u8>| Error::custom(format!("Expected 48 bytes, got {}", v.len())))?;

        <Option<G1Affine>>::from(G1Affine::from_compressed(&array))
            .map(G1Projective::from)
            .ok_or_else(|| Error::custom("Invalid G1Affine bytes"))
    }
}
