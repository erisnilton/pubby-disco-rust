-- Add up migration script here

CREATE TABLE "genre" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    "name" VARCHAR(128) NOT NULL,
    "slug" VARCHAR(128) NOT NULL,
    "parent_id" UUID REFERENCES "genre" ("id"),
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX "idx_genre_slug" ON "genre" ("slug");