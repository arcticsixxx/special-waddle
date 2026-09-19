use crossterm::event::{self, KeyCode, KeyEventKind};
use special_waddle::{
    app,
    parser::{self, handler},
    storage,
    tui::app::{
        Direction::{Left, Right},
        InputMode, UserInput,
    },
};

use tokio::sync::mpsc;

// #[tokio::main]
// async fn main() {
//     let file_repo = storage::file::FileTaskRepository::new("storage.json");
//     let mut service =
//         app::task_service::TaskService::new(file_repo).expect("Failed to create task service");

//     let (tx, mut rx) = mpsc::channel(32);

//     tokio::spawn(async move {
//         loop {
//             match parser::parser::input::<storage::file::FileTaskRepository>() {
//                 Ok(cmd) => tx.send(cmd).await.unwrap(),
//                 Err(e) => println!("Error occured {}", e),
//             }
//         }
//     });

//     while let Some(action) = rx.recv().await {
//         handler::handle_cli(action, &mut service);
//     }
// }

use special_waddle::tui::app::TuiApp;
use special_waddle::tui::ui;
// use special_waddle::{app, storage};

fn run_tui(state: &mut TuiApp) {
    ratatui::run(|terminal| {
        while !state.should_exit {
            terminal.draw(|frame| ui::render(frame, &state)).unwrap();

            if let Some(key) = event::read().unwrap().as_key_press_event() {
                match state.user_input.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            state.user_input.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            state.should_exit = true;
                        }
                        _ => {}
                    },
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => state.user_input.submit_input(),
                        KeyCode::Char(to_insert) => state.user_input.enter_char(to_insert),
                        KeyCode::Backspace => state.user_input.delete_char(),
                        KeyCode::Left => state.user_input.move_cursor(Left),
                        KeyCode::Right => state.user_input.move_cursor(Right),
                        KeyCode::Esc => state.user_input.input_mode = InputMode::Normal,
                        _ => {}
                    },
                    InputMode::Editing => {}
                }
            }
        }
    })
}

fn main() {
    let file_repo = storage::file::FileTaskRepository::new("storage.json");
    let service =
        app::task_service::TaskService::new(file_repo).expect("Failed to create task service");
    let mut state = TuiApp {
        tasks: service.tasks().unwrap(),
        active_task: service.active_task().unwrap(),
        should_exit: false,
        user_input: UserInput::new(),
    };

    run_tui(&mut state);
}
