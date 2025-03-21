-- Your SQL goes here

CREATE TABLE usuario (
                         id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                         nome VARCHAR(255) NOT NULL,
                         email VARCHAR(255) UNIQUE NOT NULL,
                         criado_em TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);