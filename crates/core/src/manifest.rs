use minicbor::{Decode, Encode};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{Members, time::Duo};

/// Protocol-defining parameters carried in every Period block.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Manifest {
    /// Logic set required to process blocks in this Period.
    #[n(0)]
    pub protocol_version: u32,
    /// UNIX timestamp (ms) of Slot 0 for this Period.
    #[n(1)]
    pub genesis_time: u64,
    /// The period coord
    #[n(2)]
    pub period_coord: Duo,
    /// Fixed slot duration in ms (e.g. 4000).
    #[n(3)]
    pub slot_duration: u64,
    /// Number of slots between mandatory Epoch blocks.
    #[n(4)]
    pub epoch_length: u64,
    /// Signing scheme identifier (0 = BLS12-381).
    #[n(5)]
    pub signing_scheme: u8,
    /// Initial members of the period
    #[n(6)]
    pub members: Members,
    /// Unique network identifier to prevent cross-chain replay attacks.
    /// Present in the genesis manifest; omitted in subsequent Period blocks.
    #[n(7)]
    #[serde_as(as = "Option<serde_with::hex::Hex>")]
    pub network_id: Option<[u8; 32]>,
}

impl Manifest {
    /// Construct the genesis manifest for a new network.
    pub fn genesis(members: Members, network_id: [u8; 32]) -> Self {
        Self {
            protocol_version: 0,
            genesis_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            period_coord: Duo::default(),
            slot_duration: 4000,
            epoch_length: 256,
            signing_scheme: 0,
            members,
            network_id: Some(network_id),
        }
    }
}
