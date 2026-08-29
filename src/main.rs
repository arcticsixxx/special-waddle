use special_waddle::{app, domain, parser, storage};
use std::path::PathBuf;

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        loop {
            match parser::parser::input::<storage::file::FileTaskRepository>() {
                Ok(cmd) => tx.send(cmd).await.unwrap(),
                Err(e) => println!("Error occured {}", e),
            }
        }
    });

    let mut file_repo = storage::file::FileTaskRepository::new("storage.json");
    let mut service =
        app::task_service::TaskService::new(file_repo).expect("Failed to create task service");

    while let Some(mut cmd) = rx.recv().await {
        if let Err(e) = cmd.execute(&mut service) {
            println!("Command failed: {}", e);
        }
    }
}
