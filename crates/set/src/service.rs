use std::sync::Arc;
use std::time::Duration;

use dashmap::DashMap;
use melon_bus::{Bus, Event};
use melon_core::{Endorsement, Id, Members, domain};
use melon_crypto::bls::{self, Certificate, G1Point, Signature, VerifyingKey};

use crate::Error;

const STALE_THRESHOLD: Duration = Duration::from_secs(30);
const CLEAN_INTERVAL: Duration = Duration::from_secs(5);

const BAD_SIGNATURE_SEVERITY: u8 = 99;

// ---------------------------------------------------------------------------
// Batch internals
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
enum Trigger {
    Threshold,
    Slot,
}

#[derive(Clone)]
struct PendingEntry {
    pos: usize,
    vk: VerifyingKey,
    sig: Signature,
}

struct Batch {
    trigger: Trigger,
    entries: Vec<PendingEntry>,
    last_insert: tokio::time::Instant,
}

impl Batch {
    fn new(trigger: Trigger) -> Self {
        Self {
            trigger,
            entries: Vec::new(),
            last_insert: tokio::time::Instant::now(),
        }
    }

    fn insert(&mut self, pos: usize, vk: VerifyingKey, sig: Signature) -> bool {
        if self.entries.iter().any(|e| e.pos == pos) {
            return false;
        }
        self.entries.push(PendingEntry { pos, vk, sig });
        self.last_insert = tokio::time::Instant::now();
        true
    }

    fn stale(&self) -> bool {
        self.last_insert.elapsed() >= STALE_THRESHOLD
    }
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

type Key = (Id, domain::Kind);

pub struct Service {
    members: Vec<VerifyingKey>,
    dsts: fn(domain::Kind) -> &'static [u8],
    threshold: usize,
    store: DashMap<Key, Batch>,
    bus: Arc<Bus>,
}

impl Service {
    pub fn new(members: Members, dsts: fn(domain::Kind) -> &'static [u8], bus: Arc<Bus>) -> Self {
        Self {
            members: members.verifying_keys(),
            dsts,
            threshold: members.threshold(),
            store: DashMap::new(),
            bus,
        }
    }

    pub async fn run(mut self) -> Result<(), Error> {
        let mut rx = self.bus.subscribe();
        let mut clean_tick = tokio::time::interval(CLEAN_INTERVAL);

        loop {
            tokio::select! {
                Ok(event) = rx.recv() => {
                    match event {
                        Event::Proposed { endorsement, .. } => self.on_endorse(endorsement)?,
                        Event::Endorsed(endorsement) => self.on_endorse(endorsement)?,
                        Event::Slot { .. } => self.on_slot()?,
                        Event::Epoch { members, .. } => self.on_epoch(members),
                        _ => {}
                    }
                }
                _ = clean_tick.tick() => {
                    self.clean();
                }
            }
        }
    }

    // -- handlers -----------------------------------------------------------

    fn on_endorse(&mut self, endorsement: Endorsement) -> Result<(), Error> {
        let Some(pos) = self.members.iter().position(|m| *m == endorsement.key) else {
            return Ok(());
        };

        let key = (endorsement.id, endorsement.kind);
        let trigger = Self::trigger_for(endorsement.kind);

        {
            let mut entry = self.store.entry(key).or_insert_with(|| Batch::new(trigger));
            entry.insert(pos, endorsement.key, endorsement.sig);
        }

        if matches!(trigger, Trigger::Threshold) {
            let ready = self
                .store
                .get(&key)
                .is_some_and(|b| b.entries.len() >= self.threshold);
            if ready {
                self.try_finalize(key)?;
            }
        }

        Ok(())
    }

    fn on_slot(&mut self) -> Result<(), Error> {
        let block_keys: Vec<Key> = self
            .store
            .iter()
            .filter(|e| matches!(e.trigger, Trigger::Slot))
            .map(|e| *e.key())
            .collect();

        for key in block_keys {
            self.emit_block(key)?;
        }

        Ok(())
    }

    fn on_epoch(&mut self, members: Members) {
        self.members = members.verifying_keys();
        self.store.clear();
    }

    // -- trigger mapping ----------------------------------------------------

    fn trigger_for(kind: domain::Kind) -> Trigger {
        match kind {
            domain::Kind::Block => Trigger::Slot,
            _ => Trigger::Threshold,
        }
    }

    // -- finalization -------------------------------------------------------

    fn point(&self, id: &Id, kind: domain::Kind) -> G1Point {
        let dst = (self.dsts)(kind);
        bls::hash_to_curve::hash_to_g1_dst(id.as_ref(), dst)
    }

    /// Slot fired. Emit whatever we have, report bad signers.
    fn emit_block(&mut self, key: Key) -> Result<(), Error> {
        panic!("Re-enable emit block code");
        let Some((_, batch)) = self.store.remove(&key) else {
            return Ok(());
        };

        if batch.entries.is_empty() {
            return Ok(());
        }

        let point = self.point(&key.0, key.1);
        let (cert, bad) = validate(&point, &batch.entries, &self.members);

        if let Some(qc) = cert {
            self.bus.publish(Event::Agreed {
                id: key.0,
                kind: key.1,
                qc,
            });
        }

        for (vk, sig) in bad {
            let (id, kind) = key.clone();
            let endorsement = Endorsement {
                id,
                kind,
                key: vk,
                sig,
            };
            self.bus.publish(Event::BadMessage {
                endorsement,
                severity: BAD_SIGNATURE_SEVERITY,
            });
        }

        Ok(())
    }

    /// Threshold reached. If all valid, emit. If bad signers found,
    /// remove them, report, and keep waiting.
    fn try_finalize(&self, key: Key) -> Result<(), Error> {
        let Some(mut entry) = self.store.get_mut(&key) else {
            return Ok(());
        };

        let point = self.point(&key.0, key.1);
        let (cert, bad) = validate(&point, &entry.entries, &self.members);

        if bad.is_empty() {
            let qc = cert.unwrap();
            drop(entry);
            self.store.remove(&key);
            tracing::info!("Agreed : {}", key.0.to_string());
            self.bus.publish(Event::Agreed {
                id: key.0,
                kind: key.1,
                qc,
            });
        } else {
            entry
                .entries
                .retain(|e| !bad.iter().any(|(vk, _)| *vk == e.vk));
            drop(entry);

            for (vk, sig) in bad {
                let (id, kind) = key.clone();
                let endorsement = Endorsement {
                    id,
                    kind,
                    key: vk,
                    sig,
                };
                self.bus.publish(Event::BadMessage {
                    endorsement,
                    severity: BAD_SIGNATURE_SEVERITY,
                });
            }
        }

        Ok(())
    }

    // -- cleanup ------------------------------------------------------------

    fn clean(&self) {
        self.store.retain(|_, batch| !batch.stale());
    }
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// Optimistically build a certificate from all entries and verify it.
/// On failure, bisect to isolate bad signers. Returns the valid certificate
/// (if any good signers remain) and the list of bad verifying keys.
fn validate(
    point: &G1Point,
    entries: &[PendingEntry],
    members: &[VerifyingKey],
) -> (Option<Certificate>, Vec<(VerifyingKey, Signature)>) {
    let cert = Certificate::make(
        &entries
            .iter()
            .map(|e| (e.pos, e.sig.clone()))
            .collect::<Vec<_>>(),
    );

    if cert.verify(point, members) {
        return (Some(cert), vec![]);
    }

    // Single entry that failed — it's the bad one.
    if entries.len() == 1 {
        return (None, vec![(entries[0].vk.clone(), entries[0].sig.clone())]);
    }

    // Bisect.
    let mid = entries.len() / 2;
    let (left, right) = entries.split_at(mid);

    let (left_cert, mut bad) = validate(point, left, members);
    let (right_cert, right_bad) = validate(point, right, members);
    bad.extend(right_bad);

    let cert = match (left_cert, right_cert) {
        (Some(a), Some(b)) => Some(a.join(&b).expect("no overlap in disjoint halves")),
        (Some(c), None) | (None, Some(c)) => Some(c),
        (None, None) => None,
    };

    (cert, bad)
}
