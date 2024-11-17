CREATE TYPE "album_type" AS ENUM(
    'Album',
    'EP',
    'Single',
    'Compilation'
);

CREATE TABLE "album" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),


    "name" VARCHAR(255) NOT NULL,
    "album_type" "album_type" NOT NULL DEFAULT 'Album',
    "cover" VARCHAR(255),
    "release_date" DATE,
    "parental_rating" SMALLINT,
    
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX "idx_album_name" ON "album" ("name");

CREATE TABLE "album_artist" (
    "album_id" UUID NOT NULL,
    "artist_id" UUID NOT NULL,
    PRIMARY KEY ("album_id", "artist_id"),
    FOREIGN KEY ("album_id") REFERENCES "album" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("artist_id") REFERENCES "artist" ("id") ON DELETE CASCADE
);