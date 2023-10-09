use crate::domain::entities::Todo;

pub trait TodoRepository {
    fn by_id(&self, id: &i64) -> Result<Todo, String>;
    fn save(&self, todo: Todo);
    fn all(&self) -> Vec<Todo>;
}
