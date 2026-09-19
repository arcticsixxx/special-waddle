use crate::{domain::Task, storage::error::RepositoryError};

pub trait TaskRepository {
    fn save(&mut self, task: &Task) -> Result<(), RepositoryError>;

    fn get(&self, id: u64) -> Result<Option<Task>, RepositoryError>;

    fn get_all(&self) -> Result<Vec<Task>, RepositoryError>;

    fn delete(&mut self, id: u64) -> Result<(), RepositoryError>;
}
