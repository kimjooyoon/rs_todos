use crate::domain::entities::Todo;

#[cfg(test)]
use mockall::*;

#[cfg_attr(test, automock)]
pub trait TodoRepository {
    fn by_id(&mut self, id: &i64) -> Result<Todo, String>;
    fn save(&mut self, todo: Todo, fn_save:fn() -> bool) -> i64;
    fn all(&mut self) -> Vec<Todo>;
}
