use crate::storage::error::RepositoryError;

pub trait ActiveTaskRepository {
    fn active_task_id(&self) -> Result<Option<u64>, RepositoryError>;

    fn set_active_task(&mut self, id: Option<u64>) -> Result<(), RepositoryError>;
}
