-- Add down migration script here

DROP TRIGGER IF EXISTS set_updated_at_users ON users;
DROP TABLE IF EXISTS users CASCADE;
