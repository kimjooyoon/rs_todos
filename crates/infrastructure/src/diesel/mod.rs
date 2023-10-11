use diesel::Connection;
use diesel::pg::PgConnection;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    let database_url = "postgres://postgres:postgres@localhost:5432/postgres";
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
