use thiserror::Error;

use crate::domain::{Task, TaskStatus};
use crate::storage::app_repository::AppRepository;
use crate::storage::error::RepositoryError;

#[derive(Debug, Error)]
pub enum TaskServiceError {
    #[error("task title cannot be empty")]
    EmptyTitle,
    #[error("task not found")]
    TaskNotFound,
    #[error("repository error")]
    RepositoryError(#[from] RepositoryError),
}

pub struct TaskService<R: AppRepository> {
    repo: R,
    next_id: u64,
}

impl<R: AppRepository> TaskService<R> {
    pub fn new(repo: R) -> Result<Self, TaskServiceError> {
        let next_id = repo.get_all()?.iter().map(|t| t.id).max().unwrap_or(0) + 1;

        Ok(Self { repo, next_id })
    }

    pub fn create_task(
        &mut self,
        title: String,
        description: String,
    ) -> Result<Task, TaskServiceError> {
        if title.trim().is_empty() {
            return Err(TaskServiceError::EmptyTitle);
        }

        let task = Task {
            id: self.next_id,
            title: title,
            description: description,
            status: TaskStatus::Todo,
        };

        self.repo.save(&task)?;
        self.next_id += 1;

        Ok(task)
    }

    pub fn delete_task(&mut self, id: u64) -> Result<(), TaskServiceError> {
        self.get_task(id)?;

        Ok(self.repo.delete(id)?)
    }

    pub fn tasks(&self) -> Result<Vec<Task>, TaskServiceError> {
        Ok(self.repo.get_all()?)
    }

    pub fn get_task(&self, id: u64) -> Result<Task, TaskServiceError> {
        self.repo.get(id)?.ok_or(TaskServiceError::TaskNotFound)
    }

    pub fn toggle_done(&mut self, id: u64) -> Result<(), TaskServiceError> {
        let mut task = self.get_task(id)?;

        task.status = match task.status {
            TaskStatus::Todo => TaskStatus::Done,
            TaskStatus::Done => TaskStatus::Todo,
        };

        self.repo.save(&task)?;

        Ok(())
    }

    pub fn start_working(&mut self, id: u64) -> Result<(), TaskServiceError> {
        self.get_task(id)?;

        self.repo.set_active_task(Some(id))?;

        Ok(())
    }

    pub fn stop_working(&mut self) -> Result<(), TaskServiceError> {
        self.repo.set_active_task(None)?;

        Ok(())
    }

    pub fn active_task(&self) -> Result<Option<Task>, TaskServiceError> {
        let Some(id) = self.repo.active_task_id()? else {
            return Ok(None);
        };

        Ok(Some(self.get_task(id)?))
    }
}
