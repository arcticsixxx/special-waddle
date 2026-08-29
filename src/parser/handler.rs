use crate::{
    app::task_service::TaskService,
    parser::parser::{CliAction, CliQuery},
    storage::app_repository::AppRepository,
};

pub fn handle_cli<R: AppRepository>(action: CliAction<R>, service: &mut TaskService<R>) {
    match action {
        CliAction::Command(cmd) => {
            if let Err(e) = cmd.execute(service) {
                println!("Command failed: {e}");
            }
        }
        CliAction::Query(CliQuery::List) => match service.tasks() {
            Ok(tasks) => {
                for task in tasks {
                    println!("{:?}", task);
                }
            }
            Err(e) => {
                println!("Query failed: {e}");
            }
        },
        CliAction::Query(CliQuery::Active) => match service.active_task() {
            Ok(Some(task)) => {
                println!("Active task: {task:?}");
            }
            Ok(None) => {
                println!("There isnt active task");
            }
            Err(e) => {
                println!("Query failed: {e}");
            }
        },
    }
}
