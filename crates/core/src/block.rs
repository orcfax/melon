use melon_crypto::bls::Certificate;
use minicbor::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::id::{Idable, blake3_cbor, blake3_pair};
use crate::time::{Duo, Quad, Trio};
use crate::{Id, Manifest};

use crate::time::TimeVariant;

// ── Head ──────────────────────────────────────────────────────────────────────

/// Block header parameterised over a time variant.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Head<Time> {
    #[n(0)]
    pub at: Time,
    #[n(1)]
    pub parent: Id,
    #[n(2)]
    pub qc: Certificate,
}

impl<Time: TimeVariant> Head<Time> {
    pub fn period(&self) -> u64 {
        self.at.period()
    }
    pub fn epoch(&self) -> u64 {
        self.at.epoch()
    }
    pub fn slot(&self) -> u64 {
        self.at.slot()
    }
    pub fn height(&self) -> u64 {
        self.at.height()
    }
    pub fn parent(&self) -> &Id {
        &self.parent
    }
}

/// `Head` is identified by the blake3 hash of its CBOR encoding.
impl<Time: minicbor::Encode<()>> Idable for Head<Time> {
    fn id(&self) -> Id {
        blake3_cbor(self)
    }
}

// ── RawBlock ──────────────────────────────────────────────────────────────────

/// Polymorphic block container: header + body, both independently parameterised.
/// Use the concrete [`Block`](crate::block::Block) enum at call sites.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct RawBlock<H, B> {
    #[n(0)]
    pub head: H,
    #[n(1)]
    pub body: B,
}

impl<H, B> RawBlock<H, B> {
    pub fn head(&self) -> &H {
        &self.head
    }
    pub fn body(&self) -> &B {
        &self.body
    }
}

/// `RawBlock` id is `blake3(head.id() ++ body.id())`.
impl<H: Idable, B: Idable> Idable for RawBlock<H, B> {
    fn id(&self) -> Id {
        blake3_pair(&self.head.id(), &self.body.id())
    }
}

/// Time accessors delegated through `Head` for the common `RawBlock<Head<Time>, B>` shape.
impl<Time: TimeVariant, B> RawBlock<Head<Time>, B> {
    pub fn period(&self) -> u64 {
        self.head.period()
    }
    pub fn epoch(&self) -> u64 {
        self.head.epoch()
    }
    pub fn slot(&self) -> u64 {
        self.head.slot()
    }
    pub fn height(&self) -> u64 {
        self.head.height()
    }
}
// ── Body types ────────────────────────────────────────────────────────────────

/// Hard forks, protocol upgrades, and disaster recovery.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct PeriodBody {
    #[n(0)]
    pub state_root: Id,
    #[n(1)]
    pub manifest: Manifest,
}

impl Idable for PeriodBody {
    fn id(&self) -> Id {
        blake3_cbor(self)
    }
}

/// Validator set rotation and execution-layer state checkpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct EpochBody {
    #[n(0)]
    pub state_root: Id,
    /// Cryptographic anchor for the incoming validator set.
    #[n(1)]
    pub validator_set_root: Id,
}

impl Idable for EpochBody {
    fn id(&self) -> Id {
        blake3_cbor(self)
    }
}

/// Regular ledger growth — the common case.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct StandardBody {
    #[n(0)]
    pub items: Vec<Id>,
}

impl Idable for StandardBody {
    fn id(&self) -> Id {
        blake3_cbor(self)
    }
}

// ── Block ─────────────────────────────────────────────────────────────────────

/// The three concrete block kinds the chain produces.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum Block {
    #[n(0)]
    Period(#[n(0)] RawBlock<Head<Duo>, PeriodBody>),
    #[n(1)]
    Epoch(#[n(0)] RawBlock<Head<Trio>, EpochBody>),
    #[n(2)]
    Standard(#[n(0)] RawBlock<Head<Quad>, StandardBody>),
}

impl Block {
    pub fn period(&self) -> u64 {
        match self {
            Block::Period(b) => b.period(),
            Block::Epoch(b) => b.period(),
            Block::Standard(b) => b.period(),
        }
    }

    pub fn epoch(&self) -> u64 {
        match self {
            Block::Period(b) => b.epoch(),
            Block::Epoch(b) => b.epoch(),
            Block::Standard(b) => b.epoch(),
        }
    }

    pub fn slot(&self) -> u64 {
        match self {
            Block::Period(b) => b.slot(),
            Block::Epoch(b) => b.slot(),
            Block::Standard(b) => b.slot(),
        }
    }

    pub fn height(&self) -> u64 {
        match self {
            Block::Period(b) => b.height(),
            Block::Epoch(b) => b.height(),
            Block::Standard(b) => b.height(),
        }
    }

    pub fn parent(&self) -> &Id {
        match self {
            Block::Period(b) => b.head().parent(),
            Block::Epoch(b) => b.head().parent(),
            Block::Standard(b) => b.head().parent(),
        }
    }
}

impl Idable for Block {
    fn id(&self) -> Id {
        match self {
            Block::Period(b) => b.id(),
            Block::Epoch(b) => b.id(),
            Block::Standard(b) => b.id(),
        }
    }
}
