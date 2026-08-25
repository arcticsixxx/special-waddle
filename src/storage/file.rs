use crate::domain::Task;
use crate::storage::repository::{RepositoryError, TaskRepository};

use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
enum FileRepositoryError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl From<std::io::Error> for FileRepositoryError {
    fn from(err: std::io::Error) -> Self {
        FileRepositoryError::Io(err)
    }
}

impl From<serde_json::Error> for FileRepositoryError {
    fn from(err: serde_json::Error) -> Self {
        FileRepositoryError::Json(err)
    }
}

pub struct FileTaskRepository {
    path: PathBuf,
}

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

// TODO: thiserror crate usage it will also remove the from trait impl
fn map_file_error(error: FileRepositoryError) -> RepositoryError {
    match error {
        FileRepositoryError::Io(_) => RepositoryError::Unavailable,
        FileRepositoryError::Json(_) => RepositoryError::Corrupted,
    }
}

impl TaskRepository for FileTaskRepository {
    fn save(&mut self, task: &Task) -> Result<(), RepositoryError> {
        let mut tasks = self.load_tasks().map_err(map_file_error)?;

        if let Some(exist) = tasks.iter_mut().find(|t| t.id == task.id) {
            *exist = task.clone();
        } else {
            tasks.push(task.clone());
        }

        self.write_tasks(&tasks).map_err(map_file_error)?;

        Ok(())
    }

    fn get(&self, id: u64) -> Result<Option<Task>, RepositoryError> {
        let tasks = self.load_tasks().map_err(map_file_error)?;

        Ok(tasks.into_iter().find(|t| t.id == id))
    }

    fn get_all(&self) -> Result<Vec<Task>, RepositoryError> {
        self.load_tasks().map_err(map_file_error)
    }

    fn delete(&self, id: u64) -> Result<(), RepositoryError> {
        let mut tasks = self.load_tasks().map_err(map_file_error)?;

        tasks.retain(|t| t.id == id);

        self.write_tasks(&tasks).map_err(map_file_error)?;

        Ok(())
    }
}
