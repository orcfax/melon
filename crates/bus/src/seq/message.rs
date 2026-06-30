use minicbor::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
pub enum Message {
    #[n(0)]
    Proposal(#[n(0)] crate::Proposal<super::Block>),
    #[n(1)]
    Endorsement(#[n(0)] crate::Endorsement),
}

impl From<crate::Proposal<super::Block>> for Message {
    fn from(value: crate::Proposal<super::Block>) -> Self {
        Message::Proposal(value)
    }
}

impl From<crate::Endorsement> for Message {
    fn from(value: crate::Endorsement) -> Self {
        Message::Endorsement(value)
    }
}
