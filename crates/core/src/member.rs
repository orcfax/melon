use std::collections::HashSet;

use melon_crypto::bls;
use minicbor::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::node;

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Member {
    #[n(0)]
    pub(crate) id: node::Id,
    #[n(1)]
    pub(crate) key: bls::VerifyingKey,
}

impl Member {
    pub fn new(id: node::Id, key: bls::VerifyingKey) -> Self {
        Self { id, key }
    }

    pub fn id(&self) -> &node::Id {
        &self.id
    }
    pub fn key(&self) -> &bls::VerifyingKey {
        &self.key
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("unknown leaver: {0:?}")]
    UnknownLeaver(node::Id),
    #[error("duplicate member: {0:?}")]
    DuplicateMember(node::Id),
}

/// The active validator set for an epoch.
/// The ordering is used to determine the bitmask in certificates.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
#[cbor(transparent)]
#[serde(transparent)]
pub struct Members {
    #[n(0)]
    entries: Vec<Member>,
}

impl TryFrom<Vec<Member>> for Members {
    type Error = Error;

    fn try_from(entries: Vec<Member>) -> Result<Self, Self::Error> {
        let mut seen = HashSet::with_capacity(entries.len());
        for m in &entries {
            if !seen.insert(m.id) {
                return Err(Error::DuplicateMember(m.id));
            }
        }
        Ok(Self { entries })
    }
}

impl From<Members> for Vec<Member> {
    fn from(value: Members) -> Self {
        value.entries
    }
}

impl Members {
    /// A stable content-derived identifier for this validator set, computed as
    /// the blake3 hash of each member's `node::Id` bytes in set order.
    ///
    /// Useful for detecting set changes across epoch boundaries without
    /// comparing the full entry list.
    pub fn id(&self) -> blake3::Hash {
        let mut hasher = blake3::Hasher::new();
        for entry in &self.entries {
            // node::Id is a [u8; 32] newtype; bls_vk follows so the hash
            // commits to the full member, not just the identity.
            hasher.update(entry.id.as_ref());
            hasher.update(&entry.key.to_compressed());
        }
        hasher.finalize()
    }

    /// Return the bitmask position of `id`, or `None` if not in the set.
    pub fn position(&self, id: &node::Id) -> Option<usize> {
        self.entries.iter().position(|e| e.id == *id)
    }

    /// The verifying key at a given position.
    pub fn verifying_key(&self, pos: usize) -> Option<&bls::VerifyingKey> {
        self.entries.get(pos).map(|e| &e.key)
    }

    /// All verifying keys in validator-set order — used for Certificate::verify.
    pub fn verifying_keys(&self) -> Vec<bls::VerifyingKey> {
        self.entries.iter().map(|e| e.key.clone()).collect()
    }

    /// Number of validators.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// 2/3+ quorum threshold over the current set size.
    pub fn threshold(&self) -> usize {
        (2 * self.len()).div_ceil(3)
    }

    pub fn update(&self, delta: &Delta) -> Result<Self, Error> {
        let left_ids: HashSet<node::Id> = delta.left().iter().copied().collect();

        for id in delta.left() {
            if !self.entries.iter().any(|m| m.id == *id) {
                return Err(Error::UnknownLeaver(*id));
            }
        }

        for new in delta.joined() {
            if self.entries.iter().any(|m| m.id == new.id) {
                return Err(Error::DuplicateMember(new.id));
            }
        }

        let entries = delta
            .joined()
            .iter()
            .cloned()
            .chain(
                self.entries
                    .iter()
                    .filter(|m| !left_ids.contains(&m.id))
                    .cloned(),
            )
            .collect();

        Ok(Self { entries })
    }

    pub fn ids(&self) -> impl Iterator<Item = &node::Id> {
        self.entries.iter().map(|e| &e.id)
    }
}

#[derive(Debug, Clone)]
pub struct Delta {
    joined: Vec<Member>,
    left: Vec<node::Id>,
}

/// FIXME :: No longer used.
/// Became too complicated. TBD
impl Delta {
    pub fn new(joined: Vec<Member>, left: Vec<node::Id>) -> Result<Self, Error> {
        let mut seen = HashSet::with_capacity(joined.len());
        for m in &joined {
            if !seen.insert(m.id) {
                return Err(Error::DuplicateMember(m.id));
            }
        }

        let mut seen = HashSet::with_capacity(left.len());
        for &id in &left {
            if !seen.insert(id) {
                return Err(Error::DuplicateMember(id));
            }
        }

        for &id in &left {
            if joined.iter().any(|m| m.id == id) {
                return Err(Error::DuplicateMember(id));
            }
        }

        Ok(Self { joined, left })
    }

    pub fn joined(&self) -> &[Member] {
        &self.joined
    }

    pub fn left(&self) -> &[node::Id] {
        &self.left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_id(byte: u8) -> node::Id {
        node::Id::from([byte; 32])
    }

    fn make_vk() -> bls::VerifyingKey {
        use melon_crypto::bls::SigningKey;
        SigningKey::generate(&mut rand::rng()).verifying_key()
    }

    fn make_member(byte: u8) -> Member {
        Member {
            id: make_id(byte),
            key: make_vk(),
        }
    }

    #[test]
    fn test_try_from_rejects_duplicates() {
        let m1 = make_member(1);
        let m2 = make_member(1); // same id
        let err = Members::try_from(vec![m1, m2]).unwrap_err();
        assert_eq!(err, Error::DuplicateMember(make_id(1)));
    }

    #[test]
    fn test_threshold() {
        let members: Vec<Member> = (0..10u8).map(make_member).collect();
        let vs = Members::try_from(members).unwrap();
        assert_eq!(vs.threshold(), 7); // ceil(2/3 * 10)
    }

    #[test]
    fn test_id_changes_with_membership() {
        let vs1 = Members::try_from(vec![make_member(1), make_member(2)]).unwrap();
        let vs2 = Members::try_from(vec![make_member(1), make_member(3)]).unwrap();
        assert_ne!(vs1.id(), vs2.id());
    }

    #[test]
    fn test_id_stable_across_clones() {
        let vs = Members::try_from(vec![make_member(1)]).unwrap();
        assert_eq!(vs.id(), vs.clone().id());
    }
}
