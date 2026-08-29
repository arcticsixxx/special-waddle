use std::{error::Error, io::stdin};

use crate::{
    app::task_service::{TaskService, TaskServiceError},
    parser::cmd::Cmd,
    storage::repository::TaskRepository,
};

pub struct CreateCmd {
    pub title: String,
    pub description: String,
}

impl<R: TaskRepository> Cmd<R> for CreateCmd {
    fn execute(self: Box<Self>, service: &mut TaskService<R>) -> Result<(), TaskServiceError> {
        service.create_task(self.title, self.description)?;

        Ok(())
    }
}

pub fn process_task_create() -> Result<CreateCmd, Box<dyn Error + Sync + Send>> {
    let mut title = String::new();
    println!("Input task title:");

    stdin().read_line(&mut title)?;

    let mut description = String::new();
    println!("Input task description:");

    stdin().read_line(&mut description)?;

    Ok(CreateCmd {
        title: title.trim().to_string(),
        description: description.trim().to_string(),
    })
}
