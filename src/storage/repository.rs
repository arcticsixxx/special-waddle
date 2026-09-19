use thiserror::Error;

use crate::domain::Task;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("repository not available")]
    Unavailable,
    #[error("data corrupted")]
    Corrupted,
    #[error("internal repository error")]
    Internal,
}

pub trait TaskRepository {
    fn save(&mut self, task: &Task) -> Result<(), RepositoryError>;

    fn get(&self, id: u64) -> Result<Option<Task>, RepositoryError>;

    fn get_all(&self) -> Result<Vec<Task>, RepositoryError>;

    fn delete(&self, id: u64) -> Result<(), RepositoryError>;
}
