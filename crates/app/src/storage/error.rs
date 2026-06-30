use melon_core::Id;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("Entry already exists for ID: {0:?}")]
    AlreadyExists(Id),
    #[error("Entry not found for ID: {0:?}")]
    NotFound(Id),
    #[error("Lock acquisition failed")]
    LockError,
}
