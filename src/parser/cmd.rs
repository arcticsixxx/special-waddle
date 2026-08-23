use crate::storage::repository::TaskRepository;

pub trait Cmd: Send {
    fn execute(&mut self, repo: &mut dyn TaskRepository) -> bool;
    fn get_title(&mut self) -> String;
    fn get_description(&mut self) -> String;
}
