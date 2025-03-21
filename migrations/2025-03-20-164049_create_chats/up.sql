-- Your SQL goes here
CREATE TABLE chat (
                      id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                      nome VARCHAR(255) NOT NULL,
                      criado_em TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);