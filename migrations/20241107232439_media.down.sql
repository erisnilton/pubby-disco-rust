-- Add down migration script here

DROP TABLE "media_interpreter" CASCADE;

DROP TABLE "media_composer" CASCADE;

DROP TABLE "media_genre" CASCADE;

DROP TABLE "media_album" CASCADE;

DROP TABLE "media" CASCADE;

DROP TYPE "media_type";