use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::Task;
use crate::storage::active_task_repository::ActiveTaskRepository;
use crate::storage::error::RepositoryError;
use crate::storage::task_repository::TaskRepository;

use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct AppData {
    tasks: Vec<Task>,
    active_task: Option<u64>,
}

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

    fn load_data(&self) -> Result<AppData, FileRepositoryError> {
        if !self.path.exists() {
            return Ok(AppData::default());
        }

        let content = fs::read_to_string(&self.path)?;

        if content.trim().is_empty() {
            return Ok(AppData::default());
        }

        let tasks = serde_json::from_str(&content)?;

        Ok(tasks)
    }

    fn write_data(&self, appdata: &AppData) -> Result<(), FileRepositoryError> {
        let json = serde_json::to_string_pretty(appdata)?;

        // TODO: write into .tmp then rename?
        fs::write(&self.path, json)?;

        Ok(())
    }
}

impl TaskRepository for FileTaskRepository {
    fn save(&mut self, task: &Task) -> Result<(), RepositoryError> {
        let mut data = self.load_data()?;

        if let Some(exist) = data.tasks.iter_mut().find(|t| t.id == task.id) {
            *exist = task.clone();
        } else {
            data.tasks.push(task.clone());
        }

        self.write_data(&data)?;

        Ok(())
    }

    fn get(&self, id: u64) -> Result<Option<Task>, RepositoryError> {
        let data = self.load_data()?;

        Ok(data.tasks.into_iter().find(|t| t.id == id))
    }

    fn get_all(&self) -> Result<Vec<Task>, RepositoryError> {
        Ok(self.load_data()?.tasks)
    }

    fn delete(&mut self, id: u64) -> Result<(), RepositoryError> {
        let mut data = self.load_data()?;

        data.tasks.retain(|t| t.id != id);

        self.write_data(&data)?;

        Ok(())
    }
}

impl ActiveTaskRepository for FileTaskRepository {
    fn active_task_id(&self) -> Result<Option<u64>, RepositoryError> {
        let data = self.load_data()?;

        Ok(data.active_task)
    }

    fn set_active_task(&mut self, id: Option<u64>) -> Result<(), RepositoryError> {
        let mut data = self.load_data()?;

        data.active_task = id;

        self.write_data(&data)?;

        Ok(())
    }
}
