use crate::{Endorsement, Id, domain::Domain};
use melon_crypto::bls;
use minicbor::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct Proposal<T> {
    #[n(0)]
    endorsement: Endorsement,
    #[n(1)]
    body: T,
}

impl<T> Proposal<T> {
    pub fn body(&self) -> &T {
        &self.body
    }

    pub fn endorsement(&self) -> &Endorsement {
        &self.endorsement
    }

    pub fn id(&self) -> &Id {
        &self.endorsement.id
    }

    pub fn author(&self) -> &bls::VerifyingKey {
        &self.endorsement.key
    }

    pub fn signature(&self) -> &bls::Signature {
        &self.endorsement.sig
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("Encode Fail. Impossible?!")]
    Encode,
    #[error("Decode Fail")]
    Decode,
    #[error("Signature Fail")]
    Signature,
}

impl<T> Proposal<T>
where
    T: Clone + Domain + for<'a> Decode<'a, ()>,
{
    /// Constructs a `Proposal` from its constituent parts, verifying that
    /// `signature` is a valid signature by `author` over `body`.
    pub fn new(
        author: bls::VerifyingKey,
        body: T,
        signature: bls::Signature,
    ) -> Result<Self, Error> {
        if !author.verify(&body.to_g1(), &signature) {
            return Err(Error::Signature);
        }
        let endorsement = Endorsement {
            id: body.id(),
            kind: T::KIND,
            key: author,
            sig: signature,
        };
        Ok(Self {
            endorsement,
            body: body,
        })
    }

    /// Create and sign a proposal.
    pub fn make(signing_key: &bls::SigningKey, body: T) -> Self {
        let signature = signing_key.sign(&body.to_g1());
        let endorsement = Endorsement {
            id: body.id(),
            kind: T::KIND,
            key: signing_key.verifying_key(),
            sig: signature,
        };
        Self {
            endorsement,
            body: body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain;
    use rand::rng;

    fn random_sk() -> bls::SigningKey {
        let mut rng = rng();
        bls::SigningKey::generate(&mut rng)
    }

    #[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
    pub struct MyType(#[n(0)] pub String);

    impl Domain for MyType {
        const DST: &'static [u8] = b"MY_CRAZY_TYPE";
        const KIND: domain::Kind = domain::Kind::App;
    }

    #[test]
    fn test_proposal_roundtrip() {
        let sk = random_sk();
        let body = MyType("Test Proposal Content".to_string());
        let original = Proposal::make(&sk, body);
        let mut buffer = Vec::new();
        minicbor::encode(&original, &mut buffer).unwrap();
        let decoded: Proposal<MyType> = minicbor::decode(&buffer).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_invalid_proposal_fails_decode() {
        let sk = random_sk();
        let body = MyType("Legit content".to_string());
        let proposal = Proposal::make(&sk, body);
        let mut buffer = Vec::new();
        minicbor::encode(&proposal, &mut buffer).unwrap();
        if buffer.len() > 10 {
            buffer[5] ^= 0xFF;
            buffer[6] ^= 0xFF;
        }
        assert!(minicbor::decode::<Proposal<MyType>>(&buffer).is_err());
    }
}
