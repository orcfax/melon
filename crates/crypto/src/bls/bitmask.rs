use minicbor::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// A packed-bit mask over an ordered node registry.
///
/// Position `i` corresponds to node `i` in the registry.
/// The mask grows as new nodes register.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, Encode, Decode)]
#[cbor(transparent)]
pub struct Bitmask(
    #[cbor(n(0))]
    #[serde(with = "serde_bytes_wrapper")]
    Vec<u8>,
);

impl Bitmask {
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// Allocate enough bytes for `n` positions (ceil(n/8) bytes), all clear.
    pub fn with_capacity(n: usize) -> Self {
        Self(vec![0u8; n.div_ceil(8)])
    }

    pub fn set(&mut self, pos: usize) {
        let byte = pos / 8;
        let bit = pos % 8;
        if byte >= self.0.len() {
            self.0.resize(byte + 1, 0);
        }
        self.0[byte] |= 1 << bit;
    }

    pub fn get(&self, pos: usize) -> bool {
        let byte = pos / 8;
        let bit = pos % 8;
        self.0.get(byte).is_some_and(|b| b & (1 << bit) != 0)
    }

    pub fn positions(&self) -> impl Iterator<Item = usize> + '_ {
        self.0.iter().enumerate().flat_map(|(byte_idx, &byte)| {
            (0..8).filter_map(move |bit| {
                if byte & (1 << bit) != 0 {
                    Some(byte_idx * 8 + bit)
                } else {
                    None
                }
            })
        })
    }

    pub fn count(&self) -> u32 {
        self.0.iter().map(|b| b.count_ones()).sum()
    }

    pub fn overlaps(&self, other: &Bitmask) -> bool {
        self.0.iter().zip(other.0.iter()).any(|(a, b)| a & b != 0)
    }

    pub fn union(&self, other: &Self) -> Self {
        let len = self.0.len().max(other.0.len());
        let mut out = vec![0u8; len];
        for (i, b) in self.0.iter().enumerate() {
            out[i] |= b;
        }
        for (i, b) in other.0.iter().enumerate() {
            out[i] |= b;
        }
        Self(out)
    }
}

impl std::fmt::Display for Bitmask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0b")?;
        for byte in self.0.iter().rev() {
            write!(f, "{:08b}", byte)?;
        }
        Ok(())
    }
}

// Newtype serde helper so Vec<u8> serializes as bytes not a sequence of ints.
mod serde_bytes_wrapper {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        if s.is_human_readable() {
            s.serialize_str(&hex::encode(v))
        } else {
            s.serialize_bytes(v)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        use serde::de::Error;
        if d.is_human_readable() {
            hex::decode(String::deserialize(d)?).map_err(D::Error::custom)
        } else {
            Vec::<u8>::deserialize(d)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get() {
        let mut mask = Bitmask::empty();
        mask.set(0);
        mask.set(7);
        mask.set(8);
        assert!(mask.get(0));
        assert!(!mask.get(1));
        assert!(mask.get(7));
        assert!(mask.get(8));
        assert!(!mask.get(9));
    }

    #[test]
    fn test_positions() {
        let mut mask = Bitmask::empty();
        mask.set(2);
        mask.set(5);
        mask.set(9);
        let pos: Vec<usize> = mask.positions().collect();
        assert_eq!(pos, vec![2, 5, 9]);
    }

    #[test]
    fn test_count() {
        let mut mask = Bitmask::with_capacity(10);
        mask.set(0);
        mask.set(3);
        mask.set(9);
        assert_eq!(mask.count(), 3);
    }

    #[test]
    fn test_cbor_roundtrip() {
        let mut mask = Bitmask::empty();
        mask.set(1);
        mask.set(10);
        let bytes = minicbor::to_vec(&mask).unwrap();
        let decoded: Bitmask = minicbor::decode(&bytes).unwrap();
        assert_eq!(mask, decoded);
    }
}
