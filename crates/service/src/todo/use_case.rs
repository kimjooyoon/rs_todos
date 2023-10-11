use todos_rs::todo::entity::Todo;
use application::out::repository::todo::TodoRepository;
use application::r#in::rest::dto::TodoDto;

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
        self.repository.save(todo, || { false })
    }
}

#[cfg(test)]
mod test {
    use mockall::predicate::eq;
    use todos_rs::todo::entity::Todo;
    use application::out::repository::todo::{MockTodoRepository, TodoRepository};
    use crate::todo::use_case::{TodoService, TodoServiceAdapter};

    #[test]
    fn retrieve_todo_by_id<'a>() {
        let mut mock = MockTodoRepository::new();
        mock.expect_by_id()
            .with(eq(1i64))
            .returning(|_| Ok(Todo {
                id: 1i64,
                title: "이슈 만들기 title".to_string(),
                content: "이슈 만들기 content".to_string(),
                order_number: 1i32,
            })).once();
        let repository: Box<dyn TodoRepository + 'a> = Box::new(mock);

        let result = TodoServiceAdapter::new(repository).retrieve_todo_by_id(1i64).expect("fail test");

        assert_eq!(1i64, result.id);
        assert_eq!("이슈 만들기 title".to_string(), result.title);
        assert_eq!("이슈 만들기 content".to_string(), result.content);
        assert_eq!(1i32, result.order_number);
    }

    #[test]
    fn retrieve_todo_all<'a>() {
        let mut mock = MockTodoRepository::new();
        mock.expect_all()
            .returning(|| vec![Todo {
                id: 1i64,
                title: "이슈 만들기 title".to_string(),
                content: "이슈 만들기 content".to_string(),
                order_number: 1i32,
            }]).once();
        let repository: Box<dyn TodoRepository + 'a> = Box::new(mock);

        let result = TodoServiceAdapter::new(repository).retrieve_todo_all();

        assert_eq!(1i64, result[0].id);
        assert_eq!("이슈 만들기 title".to_string(), result[0].title);
        assert_eq!("이슈 만들기 content".to_string(), result[0].content);
        assert_eq!(1i32, result[0].order_number);
    }

    #[test]
    fn save<'a>() {
        let mut mock = MockTodoRepository::new();
        mock.expect_save()
            .returning(|_, _| 100_i64).once();
        let repository: Box<dyn TodoRepository + 'a> = Box::new(mock);

        let result = TodoServiceAdapter::new(repository).save(Todo {
            id: 0,
            title: "".to_string(),
            content: "".to_string(),
            order_number: 0,
        });

        assert_eq!(100_i64, result);
    }
}
