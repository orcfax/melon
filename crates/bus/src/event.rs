use melon_core::Members;
use melon_crypto::bls;

use melon_core::{
    Bytes, Endorsement, Id, Manifest, domain,
    time::{Duo, Quad, Trio},
};

/// Bus events represent facts about domain state, not component interactions.
/// Variants are named after what changed, not who caused or observed the change.
/// The structure is flat: grouping is expressed through naming, not nesting.
/// `kind` is carried on endorsement and certification events because it is a
/// required input to `hash_to_curve` and cannot be reconstructed from `id` alone.
///
/// OpX are an abuse of the above mantra. They permit Ctl to inject messages over the bus.
/// TODO: Consider alternatives.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Event {
    Proposed {
        endorsement: Endorsement,
        data: Bytes,
    },
    Endorsed(Endorsement),
    Agreed {
        id: Id,
        kind: domain::Kind,
        qc: bls::Certificate,
    },

    /// Indicates a thread requires a piece of data to proceed.
    /// The emitter is responsible or cleaning up if there is no
    /// corresponding `Have` is not received in time.
    Wanted {
        id: Id,
    },

    /// Emitted strictly in reaction to a `Want` event if the data is
    /// missing from the local Content-Addressed Storage (CAS).
    /// - `melon-blob` drops all state immediately after emitting this.
    /// - `melon-net` picks up on these events.
    Missed {
        id: Id,
    },

    /// Indicates the raw data for an ID is now available.
    /// - If emitted by `net`: `blob` intercepts it to cache it locally.
    /// - If emitted by `blob`/`net`: `app` intercepts it to fulfill its pending task.
    /// Note `blob` must debounce its own messages.
    Found {
        id: Id,
        data: Bytes,
    },

    /// A slot ticked
    Slot {
        time: Quad,
    },

    /// An epoch concluded. There is a new Members (complete).
    Epoch {
        time: Trio,
        members: Members,
    },

    /// A period concluded, with the following manifest.
    /// TODO!
    Period {
        time: Duo,
        manifest: Manifest,
    },

    /// TODO :: Network needs to cache messages breifly so that it can catch bad messages when they
    /// are discovered.
    BadMessage {
        endorsement: Endorsement,
        severity: u8,
    },

    /// Ctl to inject data into App.
    OpApp {
        data: Bytes,
    },
    OpStatus,
    OpShutdown,
}
