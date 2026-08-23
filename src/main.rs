mod domain;
mod parser;
mod storage;

use std::path::PathBuf;

use tokio::sync::mpsc;

use crate::storage::file::FileTaskRepository;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        loop {
            match parser::parser::input() {
                Ok(cmd) => tx.send(cmd).await.unwrap(),
                Err(e) => println!("Error occured {}", e),
            }
        }
    });

    while let Some(mut cmd) = rx.recv().await {
        let mut file_repo = FileTaskRepository::new(PathBuf::from("/tmp/xyz"));

        cmd.execute(&mut file_repo);
    }
}
