use std::{fmt, str::FromStr};

use minicbor::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[repr(transparent)]
#[cbor(transparent)]
pub struct FeedId(#[cbor(with = "minicbor::bytes")] Vec<u8>);

impl From<&str> for FeedId {
    fn from(value: &str) -> Self {
        FeedId::from(value.as_bytes())
    }
}

impl FeedId {
    pub fn ada_usd() -> Self {
        FeedId::from("CER/ADA-USD/V3")
    }
}

impl From<&[u8]> for FeedId {
    fn from(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }
}

impl From<Vec<u8>> for FeedId {
    fn from(vec: Vec<u8>) -> Self {
        Self(vec)
    }
}

impl AsRef<[u8]> for FeedId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl FromStr for FeedId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FeedId(s.as_bytes().to_vec()))
    }
}

impl fmt::Display for FeedId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}
