use crate::model::user::Usuario;
use crate::schema::chat::dsl::chat;
use crate::schema::chat::nome as chat_nome;
use crate::util::diesel_facade::establish_connection;
use diesel::{Associations, Identifiable, QueryDsl, QueryResult, Queryable, QueryableByName, RunQueryDsl, Selectable, SelectableHelper,ExpressionMethods};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Queryable,
    Selectable,
    Identifiable,
    PartialEq,
    QueryableByName,
    Debug,
    Clone,
    Hash,
    Eq,
    Serialize,
    Deserialize,
)]
#[diesel(table_name = crate::schema::chat)]
pub struct Chat {
    pub id: Uuid,
    pub nome: String,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Chat))]
#[diesel(belongs_to(Usuario))]
#[diesel(table_name = crate::schema::usuario_chat)]
#[diesel(primary_key(usuario_id, chat_id))]
pub struct UsuarioChat {
    usuario_id: Uuid,
    chat_id: Uuid,
}

pub fn find_chat_by_name(name: &str) -> QueryResult<Chat> {
    let mut connection = establish_connection();
    chat.filter(chat_nome.eq(name))
        .select(Chat::as_select())
        .get_result(&mut connection)
}
