mod task_parser;

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        loop {
            let res = task_parser::parser::input();
            match res {
                Ok(cmd) => tx.send(cmd).await.unwrap(),
                Err(e) => println!("Error occured {}", e),
            }
        }
    });

    while let Some(mut cmd) = rx.recv().await {
        println!(
            "Task {} created with description: {}",
            cmd.get_title(),
            cmd.get_description()
        )
    }
}
