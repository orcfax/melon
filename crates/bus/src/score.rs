use melon_crypto::bls::Bitmask;

/// Per-validator signature counts derived from a collection of certificates.
///
/// `scores[i]` is the number of certificates in which validator `i` set their
/// bit. The vec is parallel to the validator set that the bitmasks were built
/// against, so it can be passed directly to [`ValidatorSet::update`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Score(Vec<u32>);

impl Score {
    /// Accumulate signature counts from `masks` for a known `validator_count`.
    ///
    /// Bit positions beyond `validator_count` are silently ignored, which
    /// handles bitmasks built against a different (larger) validator set.
    pub fn from_masks(masks: impl Iterator<Item = Bitmask>, validator_count: usize) -> Self {
        let mut scores = vec![0u32; validator_count];
        for mask in masks {
            for pos in mask.positions() {
                if let Some(s) = scores.get_mut(pos) {
                    *s += 1;
                }
            }
        }
        Self(scores)
    }

    /// The underlying score slice, parallel to the validator set.
    pub fn as_slice(&self) -> &[u32] {
        &self.0
    }

    /// Number of validators covered by this score.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Score> for Vec<u32> {
    fn from(s: Score) -> Self {
        s.0
    }
}

#[cfg(test)]
mod test {
    use melon_crypto::bls::Bitmask;

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
    fn test_score_from_masks() {
        // 3 validators, 3 certs:
        //   cert 0: validators 0 and 1 signed
        //   cert 1: validators 1 and 2 signed
        //   cert 2: all three signed
        let mut m0 = Bitmask::with_capacity(3);
        m0.set(0);
        m0.set(1);
        let mut m1 = Bitmask::with_capacity(3);
        m1.set(1);
        m1.set(2);
        let mut m2 = Bitmask::with_capacity(3);
        m2.set(0);
        m2.set(1);
        m2.set(2);

        let score = Score::from_masks([m0, m1, m2].into_iter(), 3);
        assert_eq!(score.as_slice(), &[2, 3, 2]);
    }

    #[test]
    fn test_score_ignores_out_of_range_positions() {
        // Bitmask has 2 bytes (16 positions) but validator_count is only 4.
        let mut m = Bitmask::with_capacity(16);
        m.set(0);
        m.set(12); // out of range for a 4-validator set

        let score = Score::from_masks(std::iter::once(m), 4);
        assert_eq!(score.len(), 4);
        assert_eq!(score.as_slice()[0], 1);
    }
}
