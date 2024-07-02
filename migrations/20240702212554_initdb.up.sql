-- Habilita a extensão UUID
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Tabela de usuários --
CREATE TABLE "users" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    "name" VARCHAR(255) NOT NULL,
    "email" VARCHAR(255) NOT NULL UNIQUE,
    "password" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Tabela Posts --
CREATE TABLE "posts" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    "title" VARCHAR(255) NOT NULL,
    "content" TEXT NOT NULL,
    "user_id" UUID REFERENCES "users" ("id"),
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Tabela Comments --
CREATE TABLE "comments" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    "content" TEXT NOT NULL,
    "user_id" UUID REFERENCES "users" ("id"),
    "post_id" UUID REFERENCES "posts" ("id"),
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);