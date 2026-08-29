use std::{error::Error, io};

use crate::{
    parser::{
        active_cmd::{process_task_start, process_task_stop},
        cmd::Cmd,
        create_cmd::process_task_create,
        delete_cmd::process_task_delete,
        toggle_done_cmd::process_task_toggle,
    },
    storage::app_repository::AppRepository,
};

pub enum CliAction<R: AppRepository> {
    Command(Box<dyn Cmd<R> + Send>),
    Query(CliQuery),
}

pub enum CliQuery {
    List,
    Active,
}

pub fn input<R>() -> Result<CliAction<R>, Box<dyn Error + Sync + Send>>
where
    R: AppRepository,
{
    use std::io::stdin;
    let mut input = String::new();

    stdin().read_line(&mut input)?;

    match input {
        _ if input.contains("task create") => {
            let task = process_task_create()?;
            Ok(CliAction::Command(Box::new(task)))
        }

        _ if input.contains("task delete") => {
            let task = process_task_delete()?;
            Ok(CliAction::Command(Box::new(task)))
        }

        _ if input.contains("task toggle") => {
            let task = process_task_toggle()?;
            Ok(CliAction::Command(Box::new(task)))
        }
        _ if input.contains("task start") => {
            let task = process_task_start()?;
            Ok(CliAction::Command(Box::new(task)))
        }
        _ if input.contains("task stop") => {
            let task = process_task_stop()?;
            Ok(CliAction::Command(Box::new(task)))
        }

        _ if input.contains("task list") => Ok(CliAction::Query(CliQuery::List)),
        _ if input.contains("task active") => Ok(CliAction::Query(CliQuery::Active)),

        _ => Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "unknown command",
        ))),
    }
}
