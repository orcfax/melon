/// Errors that can occur during PeerId to VerifyingKey conversion.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("PeerId does not contain an inlined identity key")]
    NoInlinedKey,
    #[error("Not an Ed25519 public key")]
    NotEd25519,
    #[error("Decoding error: {0}")]
    Decode(#[from] libp2p::identity::DecodingError),
    #[error("Cryptographic key error: {0}")]
    Crypto(#[from] ed25519_dalek::SignatureError),
}
