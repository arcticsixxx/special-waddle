use std::fs;
use std::path::PathBuf;

use crate::domain::Task;
use crate::storage::repository::TaskRepository;

use serde_json::json;

pub struct FileTaskRepository {
    path: PathBuf,
}

impl FileTaskRepository {
    pub fn new(path: PathBuf) -> Self {
        Self { path: path }
    }
}

impl TaskRepository for FileTaskRepository {
    fn save(&mut self, task: &Task) {
        let task_js = json!({
            "id": task.id,
            "title": task.title,
            "description": task.description,
        });

        let json = task_js.to_string();

        fs::write(&self.path, json).unwrap();
    }

    fn test(&mut self) {
        println!("invoked")
    }
}
