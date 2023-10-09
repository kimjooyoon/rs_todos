use crate::domain::entities::Todo;

pub trait TodoRepository {
    fn by_id(&mut self, id: &i64) -> Result<Todo, String>;
    fn save(&mut self, todo: Todo, fn_save:fn() -> bool) -> i64;
    fn all(&self) -> Vec<Todo>;
}
