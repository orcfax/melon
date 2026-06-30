use minicbor::{Decode, Encode};
use serde::{Deserialize, Serialize};

// ── Time variants ─────────────────────────────────────────────────────────────

/// Hard forks, protocol upgrades, and disaster recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, Default)]
pub struct Duo {
    #[n(0)]
    pub period: u64,
    #[n(1)]
    pub height: u64,
}

/// Validator set rotation and execution-layer state checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct Trio {
    #[n(0)]
    pub period: u64,
    #[n(1)]
    pub epoch: u64,
    #[n(2)]
    pub height: u64,
}

/// Regular ledger growth — the common case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct Quad {
    #[n(0)]
    pub period: u64,
    #[n(1)]
    pub epoch: u64,
    /// Absolute slot index since the start of the current Epoch.
    #[n(2)]
    pub slot: u64,
    #[n(3)]
    pub height: u64,
}

// ── Sealed trait ──────────────────────────────────────────────────────────────

mod sealed {
    pub trait TimeVariant {}
    impl TimeVariant for super::Duo {}
    impl TimeVariant for super::Trio {}
    impl TimeVariant for super::Quad {}
}

// ── TimeVariant: common accessors present on all three variants ───────────────
//
// `epoch` and `slot` return 0 for variants that do not carry those fields,
// matching the convention that absent sub-divisions default to zero.

pub trait TimeVariant: sealed::TimeVariant {
    fn period(&self) -> u64;
    fn epoch(&self) -> u64;
    fn slot(&self) -> u64;
    fn height(&self) -> u64;
}

impl TimeVariant for Duo {
    fn period(&self) -> u64 {
        self.period
    }
    /// Duo has no epoch — returns 0.
    fn epoch(&self) -> u64 {
        0
    }
    /// Duo has no slot — returns 0.
    fn slot(&self) -> u64 {
        0
    }
    fn height(&self) -> u64 {
        self.height
    }
}

impl TimeVariant for Trio {
    fn period(&self) -> u64 {
        self.period
    }
    fn epoch(&self) -> u64 {
        self.epoch
    }
    /// Trio has no slot — returns 0.
    fn slot(&self) -> u64 {
        0
    }
    fn height(&self) -> u64 {
        self.height
    }
}

impl TimeVariant for Quad {
    fn period(&self) -> u64 {
        self.period
    }
    fn epoch(&self) -> u64 {
        self.epoch
    }
    fn slot(&self) -> u64 {
        self.slot
    }
    fn height(&self) -> u64 {
        self.height
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duo_missing_fields_are_zero() {
        let d = Duo {
            period: 3,
            height: 42,
        };
        assert_eq!(d.period(), 3);
        assert_eq!(d.epoch(), 0);
        assert_eq!(d.slot(), 0);
        assert_eq!(d.height(), 42);
    }

    #[test]
    fn trio_missing_slot_is_zero() {
        let t = Trio {
            period: 1,
            epoch: 5,
            height: 99,
        };
        assert_eq!(t.period(), 1);
        assert_eq!(t.epoch(), 5);
        assert_eq!(t.slot(), 0);
        assert_eq!(t.height(), 99);
    }

    #[test]
    fn quad_all_fields() {
        let q = Quad {
            period: 2,
            epoch: 7,
            slot: 13,
            height: 200,
        };
        assert_eq!(q.period(), 2);
        assert_eq!(q.epoch(), 7);
        assert_eq!(q.slot(), 13);
        assert_eq!(q.height(), 200);
    }
}
