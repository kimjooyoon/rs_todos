pub struct Todo {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub order_number: i32,
}

impl Todo {
    pub fn new(id: &i64, title: &str, content: &str, order_number: &i32) -> Self {
        Self {
            id: *id,
            title: title.to_string(),
            content: content.to_string(),
            order_number: *order_number,
        }
    }
}


#[cfg(test)]
mod test {
    use crate::domain::entities::Todo;

    #[test]
    fn create_todo() {
        let id: i64 = 100;
        let title: String = String::from("todo1 example");
        let content: String = String::from("todo1 content");
        let order_number: i32 = 1;

        let todo = Todo::new(&id, &*title, &*content, &order_number);

        assert_eq!(id, todo.id);
        assert_eq!(title, todo.title);
        assert_eq!(content, todo.content);
        assert_eq!(order_number, todo.order_number);

    }
}