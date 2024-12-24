-- Add down migration script here
DROP TRIGGER IF EXISTS set_updated_at ON users;
DROP TABLE IF EXISTS users CASCADE;
DROP EXTENSION IF EXISTS "pgcrypto";
