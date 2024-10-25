-- Add up migration script here

CREATE TYPE "activity_status" AS ENUM(
    'Draft',
    'Pending',
    'Approved',
    'Rejected'
);

CREATE TYPE "activity_change_type" AS ENUM('Create', 'Update', 'Delete');

CREATE TABLE "activity" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    "status" activity_status DEFAULT 'Draft' NOT NULL,
    "user_id" UUID REFERENCES "users" ("id") NOT NULL,
    "curator_id" UUID REFERENCES "users" ("id"),
    "changes" JSONB,
    "entity_name" VARCHAR(32) NOT NULL,
    "entity_id" UUID,
    "change_type" activity_change_type NOT NULL,
    "revision_date" TIMESTAMP,
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX "idx_activity_entity_entity_id" ON "activity" ("entity_name", "entity_id");