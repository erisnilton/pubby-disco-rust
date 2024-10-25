-- Add down migration script here

DROP TABLE "activity";

DROP TYPE IF EXISTS "activity_status";

DROP TYPE IF EXISTS "activity_change_type";