use crate::{Certificate, Id, node};

type Data = String;

#[derive(Debug, Clone)]
pub enum Event {
    Make {
        data: Data,
    },
    Seen {
        id: Id,
    },
    Know {
        id: Id,
        certificate: Certificate,
    },
    Net {
        from: node::Id,
        message: super::Message,
    },
}
