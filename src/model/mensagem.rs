use diesel::{Identifiable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = crate::schema::mensagem)]
pub struct Mensagem {
    id: Uuid,
    usuario_id: Uuid,
    chat_id: Uuid,
    conteudo: String,
}
