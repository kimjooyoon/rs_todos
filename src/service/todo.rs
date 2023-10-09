use crate::domain::entities::Todo;
use crate::domain::repositories::TodoRepository;
use crate::application::dtos::TodoDto;

trait TodoService {
    fn retrieve_todo_by_id(&self, id: i64) -> Result<Todo, String>;
    fn retrieve_todo_all(&self) -> Vec<Todo>;
    fn save(&self, todo: Todo);
}

pub struct TodoServiceAdapter {
    repository: dyn TodoRepository,
}

impl TodoServiceAdapter {
    fn new(repository: Box<dyn TodoRepository>) -> Box<TodoServiceAdapter> {
        Box::new(TodoServiceAdapter { repository: (repository) })
    }
}

impl TodoService for TodoServiceAdapter {
    fn retrieve_todo_by_id(&self, id: i64) -> Result<Todo, String> {
        self.repository.by_id(&id)
    }

    fn save(&self, todo: Todo) {
        self.repository.save(todo)
    }

    fn retrieve_todo_all(&self) -> Vec<TodoDto> {
        self.repository.all().iter().map(TodoDto::from_entity).collect()
    }
}
