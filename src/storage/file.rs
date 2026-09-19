use thiserror::Error;

use crate::domain::Task;
use crate::storage::repository::{RepositoryError, TaskRepository};

use std::fs;
use std::path::PathBuf;

#[derive(Debug, Error)]
enum FileRepositoryError {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("json error")]
    Json(#[from] serde_json::Error),
}

impl From<FileRepositoryError> for RepositoryError {
    fn from(err: FileRepositoryError) -> Self {
        match err {
            FileRepositoryError::Io(_) => RepositoryError::Unavailable,
            FileRepositoryError::Json(_) => RepositoryError::Corrupted,
        }
    }
}

pub struct FileTaskRepository {
    path: PathBuf,
}
// TODO: use file as a persistance storage
impl FileTaskRepository {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    fn load_tasks(&self) -> Result<Vec<Task>, FileRepositoryError> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.path)?;

        if content.trim().is_empty() {
            return Ok(Vec::new());
        }

        let tasks = serde_json::from_str(&content)?;

        Ok(tasks)
    }

    fn write_tasks(&self, tasks: &[Task]) -> Result<(), FileRepositoryError> {
        let json = serde_json::to_string_pretty(tasks)?;

        // TODO: write into .tmp then rename?
        fs::write(&self.path, json)?;

        Ok(())
    }
}

impl TaskRepository for FileTaskRepository {
    fn save(&mut self, task: &Task) -> Result<(), RepositoryError> {
        let mut tasks = self.load_tasks()?;

        if let Some(exist) = tasks.iter_mut().find(|t| t.id == task.id) {
            *exist = task.clone();
        } else {
            tasks.push(task.clone());
        }

        self.write_tasks(&tasks)?;

        Ok(())
    }

    fn get(&self, id: u64) -> Result<Option<Task>, RepositoryError> {
        let tasks = self.load_tasks()?;

        Ok(tasks.into_iter().find(|t| t.id == id))
    }

    fn get_all(&self) -> Result<Vec<Task>, RepositoryError> {
        Ok(self.load_tasks()?)
    }

    fn delete(&self, id: u64) -> Result<(), RepositoryError> {
        let mut tasks = self.load_tasks()?;

        tasks.retain(|t| t.id != id);

        self.write_tasks(&tasks)?;

        Ok(())
    }
}
