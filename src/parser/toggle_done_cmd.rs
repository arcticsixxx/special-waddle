use std::{error::Error, io::stdin};

use crate::{parser::cmd::Cmd, storage::app_repository::AppRepository};

pub struct ToggleDoneCmd {
    pub id: u64,
}

impl<R: AppRepository> Cmd<R> for ToggleDoneCmd {
    fn execute(
        self: Box<Self>,
        service: &mut crate::app::task_service::TaskService<R>,
    ) -> Result<(), crate::app::task_service::TaskServiceError> {
        service.toggle_done(self.id)
    }
}

pub fn process_task_toggle() -> Result<ToggleDoneCmd, Box<dyn Error + Sync + Send>> {
    let mut input = String::new();

    println!("Enter task id to toggle:");
    stdin().read_line(&mut input)?;

    Ok(ToggleDoneCmd {
        id: input.trim().parse::<u64>()?,
    })
}
