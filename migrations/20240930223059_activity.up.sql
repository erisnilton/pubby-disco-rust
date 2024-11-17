-- Add up migration script here

CREATE TYPE "activity_status" AS ENUM(
    'Draft',
    'Pending',
    'Approved',
    'Rejected'
);

CREATE TABLE "activity" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    "status" activity_status DEFAULT 'Draft' NOT NULL,
    "user_id" UUID REFERENCES "users" ("id") NOT NULL,
    "curator_id" UUID REFERENCES "users" ("id"),
    "changes" JSONB,
    "revision_date" TIMESTAMP,
    "reason" VARCHAR(255),
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX "idx_activity_status" ON "activity" ("status");

CREATE INDEX "idx_activity_curator_id" ON "activity" ("curator_id");