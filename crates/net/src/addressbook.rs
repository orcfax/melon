use melon_core::{Delta, Member, Members, node};
use melon_crypto::bls;
use std::collections::HashMap;

/// Bidirectional lookup between `node::Id` and `bls::VerifyingKey`.
///
/// Immutable after construction — swap the whole thing at epoch boundaries.
#[derive(Debug, Clone)]
pub struct Addressbook {
    by_node: HashMap<node::Id, bls::VerifyingKey>,
    by_bls: HashMap<bls::VerifyingKey, node::Id>,
}

impl Addressbook {
    pub fn get_bls(&self, node_id: &node::Id) -> Option<&bls::VerifyingKey> {
        self.by_node.get(node_id)
    }

    pub fn get_node(&self, bls_key: &bls::VerifyingKey) -> Option<&node::Id> {
        self.by_bls.get(bls_key)
    }

    pub fn contains_node(&self, node_id: &node::Id) -> bool {
        self.by_node.contains_key(node_id)
    }

    pub fn contains_bls(&self, bls_key: &bls::VerifyingKey) -> bool {
        self.by_bls.contains_key(bls_key)
    }

    pub fn len(&self) -> usize {
        self.by_node.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_node.is_empty()
    }

    pub fn update(&self, delta: &Delta) -> Self {
        let mut by_node = self.by_node.clone();
        let mut by_bls = self.by_bls.clone();

        for id in delta.left() {
            if let Some(key) = by_node.remove(id) {
                by_bls.remove(&key);
            }
        }

        for m in delta.joined() {
            by_node.insert(m.id().clone(), m.key().clone());
            by_bls.insert(m.key().clone(), m.id().clone());
        }

        Self { by_node, by_bls }
    }
}

impl From<Members> for Addressbook {
    fn from(vs: Members) -> Self {
        let vs: Vec<Member> = vs.into();
        let cap = vs.len();
        let mut by_node = HashMap::with_capacity(cap);
        let mut by_bls = HashMap::with_capacity(cap);
        for member in vs.into_iter() {
            by_node.insert(member.id().clone(), member.key().clone());
            by_bls.insert(member.key().clone(), member.id().clone());
        }
        Self { by_node, by_bls }
    }
}
