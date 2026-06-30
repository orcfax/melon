use melon_crypto::bls;
use minicbor::{Decode, Encode};

use crate::{Id, domain};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct Endorsement {
    #[n(0)]
    pub id: Id,
    #[n(1)]
    pub kind: domain::Kind,
    #[n(2)]
    pub key: bls::VerifyingKey,
    #[n(3)]
    pub sig: bls::Signature,
}
