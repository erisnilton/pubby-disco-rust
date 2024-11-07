-- Add up migration script here

CREATE TABLE "artist" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    "name" VARCHAR(128) NOT NULL,
    "slug" VARCHAR(128) NOT NULL,
    "country" VARCHAR(6),
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX "idx_artist_slug" ON "artist" ("slug");

CREATE INDEX "idx_artist_name" ON "artist" ("name");