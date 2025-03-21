use diesel::{Identifiable, Queryable, QueryableByName, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, PartialEq, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::usuario)]
pub struct Usuario {
    id: Uuid,
    nome: String,
    email: String,
}
