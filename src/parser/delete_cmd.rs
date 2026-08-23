use std::{
    error::Error,
    io::{self, stdin},
};

use crate::{parser::cmd::Cmd, storage::repository::TaskRepository};

pub struct DeleteCmd {
    id: u64,
}

impl Cmd for DeleteCmd {
    fn execute(&mut self, repo: &mut dyn TaskRepository) -> bool {
        repo.test();
        true
    }

    fn get_title(&mut self) -> String {
        String::new()
    }

    fn get_description(&mut self) -> String {
        String::new()
    }
}

pub fn process_task_delete() -> Result<Box<dyn Cmd>, Box<dyn Error + Sync + Send>> {
    let mut input = String::new();

    println!("Enter task id to delete:");
    stdin().read_line(&mut input).expect("");

    if input.is_empty() || input == "\n" {
        Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "id is empty",
        )))
    } else {
        Ok(Box::new(DeleteCmd {
            id: input.trim().parse::<u64>()?,
        }))
    }
}
