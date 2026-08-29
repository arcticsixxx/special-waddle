use crate::storage::{
    active_task_repository::ActiveTaskRepository, task_repository::TaskRepository,
};

pub trait AppRepository: TaskRepository + ActiveTaskRepository {}

impl<T> AppRepository for T where T: TaskRepository + ActiveTaskRepository {}
