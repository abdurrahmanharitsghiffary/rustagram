-- Add down migration script here

DROP TRIGGER IF EXISTS set_updated_at_tokens ON tokens;

ALTER TABLE tokens
DROP CONSTRAINT fk_user_token;
ALTER TABLE tokens
DROP COLUMN user_id;

DROP TABLE IF EXISTS tokens CASCADE;

DROP TYPE IF EXISTS token_type;