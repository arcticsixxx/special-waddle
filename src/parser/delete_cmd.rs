use std::{error::Error, io::stdin};

use crate::{
    app::task_service::{TaskService, TaskServiceError},
    parser::cmd::Cmd,
    storage::app_repository::AppRepository,
};

pub struct DeleteCmd {
    pub id: u64,
}

impl<R: AppRepository> Cmd<R> for DeleteCmd {
    fn execute(self: Box<Self>, service: &mut TaskService<R>) -> Result<(), TaskServiceError> {
        service.delete_task(self.id)
    }
}

pub fn process_task_delete() -> Result<DeleteCmd, Box<dyn Error + Sync + Send>> {
    let mut input = String::new();

    println!("Enter task id to delete:");
    stdin().read_line(&mut input)?;

    Ok(DeleteCmd {
        id: input.trim().parse::<u64>()?,
    })
}
