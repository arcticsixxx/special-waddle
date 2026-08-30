use special_waddle::{
    app,
    parser::{self, handler},
    storage,
};

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

    let file_repo = storage::file::FileTaskRepository::new("storage.json");
    let mut service =
        app::task_service::TaskService::new(file_repo).expect("Failed to create task service");

    while let Some(action) = rx.recv().await {
        handler::handle_cli(action, &mut service);
    }
}

// use special_waddle::tui::app::TuiApp;
// use special_waddle::tui::ui;
// use special_waddle::{app, storage};

// fn main() {
//     let file_repo = storage::file::FileTaskRepository::new("storage.json");
//     let service =
//         app::task_service::TaskService::new(file_repo).expect("Failed to create task service");
//     let state = TuiApp {
//         tasks: service.tasks().unwrap(),
//         active_task: service.active_task().unwrap(),
//         should_exit: false,
//     };

//     ratatui::run(|terminal| {
//         while !state.should_exit {
//             terminal.draw(|frame| ui::render(frame, &state)).unwrap();
//         }
//     })
// }
