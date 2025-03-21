-- Your SQL goes here
CREATE TABLE mensagem (
                          id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                          usuario_id UUID NOT NULL,
                          chat_id UUID NOT NULL,
                          conteudo TEXT NOT NULL,
                          enviado_em TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                          FOREIGN KEY (usuario_id) REFERENCES usuario(id) ON DELETE CASCADE,
                          FOREIGN KEY (chat_id) REFERENCES chat(id) ON DELETE CASCADE
);