use crate::domain::Task;

pub struct TuiApp {
    pub tasks: Vec<Task>,
    pub active_task: Option<Task>,

    pub should_exit: bool,
}
