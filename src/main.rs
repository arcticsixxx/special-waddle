use special_waddle::{
    app::task_service::TaskService,
    storage::{self},
    tui::app::TuiApp,
};

fn main() {
    let file_repo = storage::file::FileTaskRepository::new("storage.json");
    let mut service = TaskService::new(file_repo).expect("Failed to create task service");
    let mut tui_app = TuiApp::new(&mut service);

    tui_app.run();
}
