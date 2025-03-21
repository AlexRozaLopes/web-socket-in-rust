-- Your SQL goes here
CREATE TABLE usuario_chat (
                              usuario_id UUID NOT NULL,
                              chat_id UUID NOT NULL,
                              PRIMARY KEY (usuario_id, chat_id),
                              FOREIGN KEY (usuario_id) REFERENCES usuario(id) ON DELETE CASCADE,
                              FOREIGN KEY (chat_id) REFERENCES chat(id) ON DELETE CASCADE
);