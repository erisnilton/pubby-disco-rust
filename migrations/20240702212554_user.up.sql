-- Habilita a extensão UUID
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Tabela de usuários --
CREATE TABLE "users" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    "username" VARCHAR(255) NOT NULL UNIQUE,
    "display_name" VARCHAR(255) NOT NULL,
    "email" VARCHAR(255) NOT NULL,
    "password" VARCHAR(128) NOT NULL,
    "is_curator" BOOLEAN NOT NULL DEFAULT FALSE,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Adiciona um índice para o campo email para melhorar a performance de busca
CREATE INDEX "idx_users_email" ON "users" ("email");

-- Adiciona um índice para o campo is_curator para melhorar a performance de busca
CREATE INDEX "idx_users_is_curator" ON "users" ("is_curator");