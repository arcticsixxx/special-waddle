use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("repository not available")]
    Unavailable,
    #[error("data corrupted")]
    Corrupted,
    #[error("internal repository error")]
    Internal,
}
