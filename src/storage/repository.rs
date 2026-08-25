use crate::domain::Task;


#[derive(Debug)]
pub enum RepositoryError {
    Unavailable, 
    Corrupted,
    Internal
}
    
pub trait TaskRepository {
    fn save(&mut self, task: &Task) -> Result<(), RepositoryError>;

    fn get(&self, id: u64) -> Result<Option<Task>, RepositoryError>;

    fn get_all(&self) -> Result<Vec<Task>, RepositoryError>;

    fn delete(&self, id: u64) -> Result<(), RepositoryError>;
}

