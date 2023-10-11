use domain::todo::entity::Todo;

#[derive(Debug, PartialEq, Eq)]
pub struct TodoDto {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub order_number: i32,
}

impl TodoDto {
    pub fn from_entity(todo: &Todo) -> Self {
        Self {
            id: todo.id.clone(),
            title: todo.title.clone(),
            content: todo.content.clone(),
            order_number: todo.order_number.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use domain::todo::entity::Todo;
    use crate::r#in::rest::dto::TodoDto;

    #[test]
    fn create_todo_dto_from_entity() {
        let id: i64 = 100;
        let title: String = String::from("todo1 example");
        let content: String = String::from("todo1 content");
        let order_number: i32 = 1;
        let todo = Todo::new(&id, &*title, &*content, &order_number);

        let todo_dto = TodoDto::from_entity(&todo);

        assert_eq!(id, todo_dto.id);
        assert_eq!(title, todo_dto.title);
        assert_eq!(content, todo_dto.content);
        assert_eq!(order_number, todo_dto.order_number);
    }
}
