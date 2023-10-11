use diesel::pg::PgConnection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use domain::todo::entity::Todo;
use application::out::repository::todo::TodoRepository;
use crate::diesel::models::{NewTodos, Todos};

pub struct TodoPostgresRepository<'a> {
    connection: Box<&'a mut PgConnection>,
}

impl<'a> TodoPostgresRepository<'a> {
    pub fn new(connection: &'a mut PgConnection) -> TodoPostgresRepository<'a> {
        TodoPostgresRepository {
            connection: Box::new(connection),
        }
    }
    fn persistent_to_domain(persist: &Todos) -> Todo {
        Todo {
            id: persist.id.clone(),
            title: persist.title.clone(),
            content: persist.content.clone(),
            order_number: persist.order_number.clone(),
        }
    }
}

use crate::diesel::schema::todos::dsl::*;

impl TodoRepository for TodoPostgresRepository<'_> {
    fn by_id(&mut self, id_i64: &i64) -> Result<Todo, String> {
        let results: Todos = todos
            .filter(id.eq(id_i64))
            .select(Todos::as_select())
            .first(*self.connection)
            .expect("Error loading todos");

        let todo: Todo = Todo::new(&results.id, &results.title, &results.content, &results.order_number);
        Ok(todo)
    }

    fn save(&mut self, todo: Todo, fn_save: fn() -> bool) -> i64 {
        let new_todo = NewTodos {
            title: todo.title,
            content: todo.content,
            order_number: todo.order_number,
            is_deleted: fn_save(),
        };

        let result = diesel::insert_into(todos)
            .values(&new_todo)
            .returning(Todos::as_returning())
            .get_result(*self.connection);

        result.expect("Error saving new todos").id
    }

    fn all(&mut self) -> Vec<Todo> {
        let result = todos.
            select(Todos::as_select())
            .load(*self.connection).expect("Error loading all todos");

        result.iter().map(Self::persistent_to_domain).collect()
    }
}


#[cfg(test)]
mod test {
    use diesel::{Connection, PgConnection};
    use diesel::result::Error;
    use domain::todo::entity::Todo;
    use application::out::repository::todo::TodoRepository;
    use crate::diesel::establish_connection;
    use crate::todo::repository::TodoPostgresRepository;

    #[test]
    fn by_id() {
        let connection: &mut PgConnection = &mut establish_connection();
        let mut repository = TodoPostgresRepository::new(connection);
        let id: i64 = 1;

        let result = repository.by_id(&id).expect("fail");

        assert_eq!(1, result.id);
        assert_eq!("이슈 만들기", result.title);
        assert_eq!("만든 이슈를 기준으로 id와 commit 값을 가져온다", result.content);
        assert_eq!(1, result.order_number);
    }

    #[test]
    fn save() {
        let connection: &mut PgConnection = &mut establish_connection();

        let _ = connection.transaction(|connection| -> Result<(), Error> {
            let mut repository = TodoPostgresRepository::new(connection);
            let todo = Todo {
                id: 0,
                title: "이슈 만들기2".to_string(),
                content: "두번째 이슈를 추가한다".to_string(),
                order_number: 0,
            };

            let result = TodoPostgresRepository::save(&mut repository, todo, || { false });

            assert_ne!(1i64, result);
            print!("{}", result);
            Err(Error::NotFound)
        });
    }

    #[test]
    fn all() {
        let connection: &mut PgConnection = &mut establish_connection();
        let mut repository = TodoPostgresRepository::new(connection);

        let result = TodoPostgresRepository::all(&mut repository);

        assert_eq!(1, result.len())
    }
}
