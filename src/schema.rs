// @generated automatically by Diesel CLI.

diesel::table! {
    chat (id) {
        id -> Uuid,
        #[max_length = 255]
        nome -> Varchar,
        criado_em -> Nullable<Timestamp>,
    }
}

diesel::table! {
    mensagem (id) {
        id -> Uuid,
        usuario_id -> Uuid,
        chat_id -> Uuid,
        conteudo -> Text,
        enviado_em -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        password -> Text,
        #[max_length = 50]
        nickname -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    usuario (id) {
        id -> Uuid,
        #[max_length = 255]
        nome -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        criado_em -> Nullable<Timestamp>,
    }
}

diesel::table! {
    usuario_chat (usuario_id, chat_id) {
        usuario_id -> Uuid,
        chat_id -> Uuid,
    }
}

diesel::joinable!(mensagem -> chat (chat_id));
diesel::joinable!(mensagem -> usuario (usuario_id));
diesel::joinable!(usuario_chat -> chat (chat_id));
diesel::joinable!(usuario_chat -> usuario (usuario_id));

diesel::allow_tables_to_appear_in_same_query!(
    chat,
    mensagem,
    users,
    usuario,
    usuario_chat,
);
