use std::{
    error::Error,
    io::{self},
};

use crate::task_parser::{
    cmd::Cmd, create_cmd::process_task_create, delete_cmd::process_task_delete,
};

pub fn input() -> Result<Box<dyn Cmd>, Box<dyn Error + Sync + Send>> {
    use std::io::stdin;
    let mut input = String::new();

    stdin().read_line(&mut input).expect("No input");
    match input {
        _ if input.contains("task create") => match process_task_create() {
            Ok(t) => Ok(t),
            Err(e) => Err(e),
        },

        _ if input.contains("task delete") => match process_task_delete() {
            Ok(t) => Ok(t),
            Err(e) => Err(e),
        },

        _ => Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "unknown cmd",
        ))),
    }
}
