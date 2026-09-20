use crate::{
    app::task_service::{TaskService, TaskServiceError},
    storage::app_repository::AppRepository,
};

// TODO: maybe it should not be in the parser dir?
// todo(a6): logic layer?
pub trait Cmd<R: AppRepository>: Send {
    fn execute(self: Box<Self>, service: &mut TaskService<R>) -> Result<(), TaskServiceError>;
}
