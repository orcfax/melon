use minicbor::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// Frequently some possibly content aware hash, frequently blake3
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Encode, Decode,
)]
#[serde(transparent)]
#[cbor(transparent)]
pub struct Id(#[n(0)] [u8; 32]);

impl From<[u8; 32]> for Id {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl TryFrom<&[u8]> for Id {
    type Error = std::array::TryFromSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        <[u8; 32]>::try_from(value).map(Self::from)
    }
}

impl From<Id> for [u8; 32] {
    fn from(id: Id) -> Self {
        id.0
    }
}

impl AsRef<[u8; 32]> for Id {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl AsMut<[u8; 32]> for Id {
    fn as_mut(&mut self) -> &mut [u8; 32] {
        &mut self.0
    }
}

impl std::ops::Deref for Id {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl std::str::FromStr for Id {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(
            <[u8; 32]>::try_from(hex::decode(s).map_err(|e| e.to_string())?)
                .map_err(|_e| "Wrong length".to_string())?,
        ))
    }
}

// ── Idable ────────────────────────────────────────────────────────────────────

/// Anything that can produce a content-addressed [`Id`].
pub trait Idable {
    fn id(&self) -> Id;
}

/// Hash an arbitrary CBOR-encodable value with blake3.
pub fn blake3_bytes(bytes: &[u8]) -> Id {
    Id(*blake3::hash(bytes).as_bytes())
}

/// Hash an arbitrary CBOR-encodable value with blake3.
pub fn blake3_cbor<T: minicbor::Encode<()>>(value: &T) -> Id {
    blake3_bytes(&minicbor::to_vec(value).expect("Infallible"))
}

/// Hash two already-computed [`Id`]s together: `blake3(left ++ right)`.
pub fn blake3_pair(left: &Id, right: &Id) -> Id {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&left.0);
    hasher.update(&right.0);
    Id(*hasher.finalize().as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_zero() {
        let id = Id([0u8; 32]);
        let s = id.to_string();
        assert_eq!(s, "0".repeat(64));
        assert_eq!(s.parse::<Id>().unwrap(), id);
    }

    #[test]
    fn roundtrip_ones() {
        let id = Id([0xffu8; 32]);
        let s = id.to_string();
        assert_eq!(s, "ff".repeat(32));
        assert_eq!(s.parse::<Id>().unwrap(), id);
    }

    #[test]
    fn roundtrip_arbitrary() {
        let mut bytes = [0u8; 32];
        bytes[0] = 0xde;
        bytes[1] = 0xad;
        bytes[31] = 0xff;
        let id = Id(bytes);
        assert_eq!(id.to_string().parse::<Id>().unwrap(), id);
    }

    #[test]
    fn display_is_lowercase_hex() {
        let id = Id([0xabu8; 32]);
        assert!(
            id.to_string()
                .chars()
                .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
        );
    }

    #[test]
    fn err_on_odd_length() {
        assert!("abc".parse::<Id>().is_err());
    }

    #[test]
    fn err_on_wrong_length() {
        // valid hex but only 31 bytes
        assert!("aa".repeat(31).parse::<Id>().is_err());
    }

    #[test]
    fn err_on_invalid_hex() {
        assert!("zz".repeat(32).parse::<Id>().is_err());
    }
}
