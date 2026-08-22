pub struct TaskManager {}

pub trait Cmd: Send {
    fn execute(&mut self, task_manager: &mut TaskManager) -> bool;
    fn get_title(&mut self) -> String;
    fn get_description(&mut self) -> String;
}
