mod storage;
mod domain;

use crate::storage::repository::TaskRepository;

fn main() {
    let new_task = domain::Task {
        id: 52,
        title: "title".to_string(),
        description: "desc".to_string(),
    };

    let mut repo = storage::file::FileTaskRepository { path: "storage.json".to_string(), };
    repo.save(&new_task);
}
