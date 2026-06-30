use melon_bus::Event;
use melon_core::{Bytes, Endorsement, Id, domain, time::Trio};
use melon_crypto::bls;
use minicbor::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
pub struct Message {
    /// Prevent replays, and verify that the validator sets _ought_ to align.
    #[n(0)]
    pub epoch: Trio,
    #[n(1)]
    pub kind: domain::Kind,
    #[n(2)]
    pub id: Id,
    #[n(3)]
    pub sig: bls::Signature,
    #[n(4)]
    pub data: Option<Vec<u8>>,
}

impl Message {
    pub fn into_event(self, from: bls::VerifyingKey) -> Event {
        let endorsement = Endorsement {
            id: self.id,
            kind: self.kind,
            key: from,
            sig: self.sig,
        };

        match self.data {
            Some(data) => Event::Proposed {
                endorsement,
                data: data.into(),
            },
            None => Event::Endorsed(endorsement),
        }
    }

    pub fn from_endorsement(epoch: Trio, endorsemet: Endorsement) -> Self {
        let Endorsement { kind, id, sig, .. } = endorsemet;
        Self {
            epoch,
            kind,
            id,
            sig,
            data: None,
        }
    }

    pub fn from_proposal(epoch: Trio, endorsemet: Endorsement, data: Bytes) -> Self {
        let Endorsement { kind, id, sig, .. } = endorsemet;
        Self {
            epoch,
            kind,
            id,
            sig,
            data: Some(data.0),
        }
    }
}
