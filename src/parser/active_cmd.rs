use std::{error::Error, io::stdin};

use crate::{
    app::task_service::{TaskService, TaskServiceError},
    parser::cmd::Cmd,
    storage::app_repository::AppRepository,
};

pub struct StartWorkingCmd {
    id: u64,
}
pub struct StopWorkingCmd;

impl<R: AppRepository> Cmd<R> for StartWorkingCmd {
    fn execute(self: Box<Self>, service: &mut TaskService<R>) -> Result<(), TaskServiceError> {
        service.start_working(self.id)
    }
}

impl<R: AppRepository> Cmd<R> for StopWorkingCmd {
    fn execute(self: Box<Self>, service: &mut TaskService<R>) -> Result<(), TaskServiceError> {
        service.stop_working()
    }
}

pub fn process_task_stop() -> Result<StopWorkingCmd, Box<dyn Error + Sync + Send>> {
    Ok(StopWorkingCmd)
}

pub fn process_task_start() -> Result<StartWorkingCmd, Box<dyn Error + Sync + Send>> {
    let mut input = String::new();

    println!("Enter task id to start:");
    stdin().read_line(&mut input)?;

    Ok(StartWorkingCmd {
        id: input.trim().parse::<u64>()?,
    })
}
