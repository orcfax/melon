use libp2p::PeerId;

use crate::{
    app,
    entry::{Entry, EntryId},
};

#[derive(Debug, Clone)]
pub enum Command {
    // Run
    Publish(app::Envelope),
    GetEntry(EntryId),
    PutEntry(Entry),
    // Peer management
    ReportPeer(PeerId),
}
