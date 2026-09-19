use std::{
    error::Error,
    io::{self},
};

use crate::{
    parser::{
        cmd::Cmd, create_cmd::process_task_create, delete_cmd::process_task_delete,
        list_cmd::process_task_list,
    },
    storage::repository::TaskRepository,
};

pub fn input<R>() -> Result<Box<dyn Cmd<R>>, Box<dyn Error + Sync + Send>>
where
    R: TaskRepository,
{
    use std::io::stdin;
    let mut input = String::new();

    stdin().read_line(&mut input).expect("No input");
    match input {
        _ if input.contains("task create") => {
            let cmd = process_task_create()?;
            Ok(Box::new(cmd))
        }

        _ if input.contains("task delete") => {
            let cmd = process_task_delete()?;
            Ok(Box::new(cmd))
        }

        _ if input.contains("task list") => Ok(Box::new(process_task_list())),

        _ => Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "unknown cmd",
        ))),
    }
}
