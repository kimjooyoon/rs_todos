use diesel::pg::PgConnection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use crate::domain::entities::Todo;
use crate::domain::repositories::TodoRepository;
use crate::infrastructure::diesel::models::Todos;

pub struct TodoPostgresRepository<'a> {
    connection: Box<&'a mut PgConnection>,
}

impl<'a> TodoPostgresRepository<'a> {
    pub fn new(connection: &'a mut PgConnection) -> TodoPostgresRepository<'a> {
        TodoPostgresRepository {
            connection: Box::new(connection),
        }
    }
}

use crate::infrastructure::diesel::schema::todos::dsl::*;

impl TodoRepository for TodoPostgresRepository<'_> {
    fn by_id(&mut self, id_i64: &i64) -> Result<Todo, String> {
        let connection: &mut PgConnection = *self.connection;
        let results: Todos = todos
            .filter(id.eq(id_i64))
            .select(Todos::as_select())
            .first(connection)
            .expect("Error loading todos");

        let todo: Todo = Todo::new(&results.id, &results.title, &results.content, &results.order_number);
        Ok(todo)
    }

    fn save(&self, _todo: Todo) {
        return;
    }

    fn all(&self) -> Vec<Todo> {
        return vec![]
    }
}

#[cfg(test)]
mod test {
    use diesel::PgConnection;
    use crate::domain::repositories::TodoRepository;
    use crate::infrastructure::diesel::establish_connection;
    use crate::infrastructure::todo::repositories::TodoPostgresRepository;

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
}
