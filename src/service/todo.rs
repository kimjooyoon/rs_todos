use crate::domain::entities::Todo;
use crate::domain::repositories::TodoRepository;
use crate::application::dtos::TodoDto;

trait TodoService {
    fn retrieve_todo_by_id(&mut self, id: i64) -> Result<Todo, String>;
    fn retrieve_todo_all(&mut self) -> Vec<TodoDto>;
    fn save(&mut self, todo: Todo) -> i64;
}

pub struct TodoServiceAdapter<'a> {
    repository: Box<dyn TodoRepository + 'a>,
}

impl<'a> TodoServiceAdapter<'a> {
    fn new(repository: Box<dyn TodoRepository + 'a>) -> Box<TodoServiceAdapter> {
        Box::new(TodoServiceAdapter { repository })
    }
}

impl TodoService for TodoServiceAdapter<'_> {
    fn retrieve_todo_by_id(&mut self, id: i64) -> Result<Todo, String> {
        self.repository.by_id(&id)
    }

    fn retrieve_todo_all(&mut self) -> Vec<TodoDto> {
        self.repository.all().iter().map(TodoDto::from_entity).collect()
    }

    fn save(&mut self, todo: Todo) -> i64 {
        self.repository.save(todo, || {false})
    }
}
