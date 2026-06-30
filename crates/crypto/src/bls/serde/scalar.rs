use bls12_381::Scalar;
use serde::{Deserialize, Deserializer, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

pub struct Format;

impl SerializeAs<Scalar> for Format {
    fn serialize_as<S>(value: &Scalar, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = value.to_bytes();
        if serializer.is_human_readable() {
            serializer.serialize_str(&hex::encode(bytes))
        } else {
            serializer.serialize_bytes(&bytes)
        }
    }
}

impl<'de> DeserializeAs<'de, Scalar> for Format {
    fn deserialize_as<D>(deserializer: D) -> Result<Scalar, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let bytes = if deserializer.is_human_readable() {
            hex::decode(String::deserialize(deserializer)?).map_err(Error::custom)?
        } else {
            Vec::<u8>::deserialize(deserializer)?
        };

        let array: [u8; 32] = bytes
            .try_into()
            .map_err(|v: Vec<u8>| Error::custom(format!("Expected 32 bytes, got {}", v.len())))?;

        Option::from(Scalar::from_bytes(&array))
            .ok_or_else(|| Error::custom("Invalid Scalar bytes"))
    }
}
