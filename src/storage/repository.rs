use crate::domain::Task;

pub trait TaskRepository {
    fn save(&mut self, task: &Task);
    fn test(&mut self);
}
