-- Add down migration script here

DROP FUNCTION IF EXISTS update_timestamp;
DROP EXTENSION IF EXISTS "pgcrypto";
