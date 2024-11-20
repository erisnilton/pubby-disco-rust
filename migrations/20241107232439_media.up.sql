-- Add up migration script here

CREATE TYPE "media_type" AS ENUM('Song', 'Instrumental');

CREATE TABLE "media" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    "name" VARCHAR(255) NOT NULL,
    "type" "media_type" NOT NULL DEFAULT 'Song',
    "slug" VARCHAR(255) NOT NULL,
    "cover" VARCHAR(255),
    "release_date" DATE,
    "is_single" BOOLEAN DEFAULT FALSE,
    "parental_rating" SMALLINT,
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX "media_slug_index" ON "media" ("slug");

CREATE TABLE "media_album" (
    "media_id" UUID NOT NULL,
    "album_id" UUID NOT NULL,
    PRIMARY KEY ("media_id", "album_id"),
    FOREIGN KEY ("media_id") REFERENCES "media" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("album_id") REFERENCES "album" ("id") ON DELETE CASCADE
);

CREATE TABLE "media_genre" (
    "media_id" UUID NOT NULL,
    "genre_id" UUID NOT NULL,
    PRIMARY KEY ("media_id", "genre_id"),
    FOREIGN KEY ("media_id") REFERENCES "media" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("genre_id") REFERENCES "genre" ("id") ON DELETE CASCADE
);

CREATE TABLE "media_composer" (
    "media_id" UUID NOT NULL,
    "composer_id" UUID NOT NULL,
    PRIMARY KEY ("media_id", "composer_id"),
    FOREIGN KEY ("media_id") REFERENCES "media" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("composer_id") REFERENCES "artist" ("id") ON DELETE CASCADE
);

CREATE TABLE "media_interpreter" (
    "media_id" UUID NOT NULL,
    "interpreter_id" UUID NOT NULL,
    PRIMARY KEY ("media_id", "interpreter_id"),
    FOREIGN KEY ("media_id") REFERENCES "media" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("interpreter_id") REFERENCES "artist" ("id") ON DELETE CASCADE
)