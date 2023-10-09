use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todos {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub order_number: i32,
}
