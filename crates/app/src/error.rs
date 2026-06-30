#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("Network :: {0}")]
    Network(String),
    #[error("Not yet implemented")]
    Code,
}
