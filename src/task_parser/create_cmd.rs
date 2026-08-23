use std::{
    error::Error,
    io::{self, stdin},
};

use crate::task_parser::cmd::{Cmd, TaskManager};

pub struct CreateTask {
    title: String,
    _description: String,
}

impl Cmd for CreateTask {
    fn execute(&mut self, _task_manager: &mut TaskManager) -> bool {
        true
    }

    fn get_title(&mut self) -> String {
        self.title.clone()
    }

    fn get_description(&mut self) -> String {
        self._description.clone()
    }
}

pub fn process_task_create() -> Result<Box<dyn Cmd>, Box<dyn Error + Sync + Send>> {
    let mut title = String::new();
    println!("Input task title:");

    stdin().read_line(&mut title).expect("Failed to read");

    let mut description = String::new();
    println!("Input task description:");

    stdin().read_line(&mut description).expect("Failed to read");

    if title.is_empty() || title == "\n" {
        Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "title is empty",
        )))
    } else {
        Ok(Box::new(CreateTask {
            title: title,
            _description: description,
        }))
    }
}
