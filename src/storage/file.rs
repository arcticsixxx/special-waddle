use std::fs;

use crate::domain::Task;
use crate::storage::repository::TaskRepository;

// use std::path::PathBuf;
use serde_json::json;

pub struct FileTaskRepository {
    pub path: String, // PathBuf and private
}

// impl FileTaskRepository {
//     pub fn new( ) {
//     }
// }

impl TaskRepository for FileTaskRepository {
    fn save(&mut self, task: &Task) {
        let task_js= json!({
            "id": task.id,
            "title": task.title,
            "description": task.description,
        });

        let json = task_js.to_string();

        fs::write(&self.path, json).unwrap();

        // "name": full_name,
        // "age": age_last_year + 1,
        // "phones": [
        //     format!("+44 {}", random_phone())
        // ]
    }
}
