use crate::{
    app::task_service::{TaskService, TaskServiceError},
    parser::cmd::Cmd,
    storage::repository::TaskRepository,
};

pub struct ListCmd;

impl<R: TaskRepository> Cmd<R> for ListCmd {
    fn execute(self: Box<Self>, service: &mut TaskService<R>) -> Result<(), TaskServiceError> {
        let tasks = service.tasks()?;

        for task in tasks {
            println!(
                "id: {}\ntitle: {}\ndescription: {}\n",
                task.id, task.title, task.description
            )
        }

        Ok(())
    }
}

pub fn process_task_list() -> ListCmd {
    ListCmd
}
