use crate::{
    app::task_service::{TaskService, TaskServiceError},
    storage::repository::TaskRepository,
};

// TODO: maybe it should not be in the parser dir?
pub trait Cmd<R: TaskRepository>: Send {
    // self is not reference to move task title and description) while executing
    fn execute(self: Box<Self>, service: &mut TaskService<R>) -> Result<(), TaskServiceError>;
}
