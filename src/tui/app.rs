use crate::{
    app::task_service::TaskService,
    domain::Task,
    parser::{self, handler::handle_cli},
    storage::app_repository::AppRepository,
    tui::ui::{Action, Ui},
};

pub struct TuiApp<'a, R: AppRepository> {
    // model
    task_service: &'a mut TaskService<R>,

    pub tasks: Vec<Task>,
    pub active_task: Option<Task>,

    // logic
    pub should_exit: bool,

    // view
    ui: Ui,
}

impl<'a, R: AppRepository> TuiApp<'a, R> {
    pub fn new(service: &'a mut TaskService<R>) -> Self {
        let tasks = service.tasks().unwrap();
        let active_task = service.active_task().unwrap();
        Self {
            task_service: service,
            tasks,
            active_task,
            ui: Ui::new(),
            should_exit: false,
        }
    }

    pub fn run(&mut self) {
        ratatui::run(|terminal| {
            while !self.should_exit {
                terminal.draw(|frame| self.ui.render(frame, self)).unwrap();
                match self.ui.process_key_events() {
                    Action::None => {}
                    Action::Submit => self.submit_input(),
                    Action::Exit => self.should_exit = true,
                }
            }
        });
    }

    fn show_error(&mut self, error: &str) {
        self.ui.show_error(error);
    }

    pub fn submit_input(&mut self) {
        match parser::parser::process_input(&self.ui.input_field.input) {
            Ok(action) => {
                handle_cli::<R>(action, &mut self.task_service);
                self.ui.input_field.submit_input();
            }
            Err(error) => {
                self.show_error(&error.to_string());
                self.ui.input_field.submit_input();
            }
        };
    }
}
