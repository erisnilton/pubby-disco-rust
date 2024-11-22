-- Add up migration script here

CREATE TABLE "source" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    "source_type" VARCHAR(80) NOT NULL,
    "src" VARCHAR(255) NOT NULL,
    "media_id" UUID NOT NULL,
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY ("media_id") REFERENCES "media" ("id") ON DELETE CASCADE
);

CREATE INDEX "idx_source_type" ON "source" ("source_type");