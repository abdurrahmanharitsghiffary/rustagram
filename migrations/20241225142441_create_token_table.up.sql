-- Add up migration script here

CREATE TYPE token_type AS ENUM ('acc_tkn', 'rsh_tkn', 'vry_tkn', 'rst_tkn');

CREATE TABLE IF NOT EXISTS tokens (
    id SERIAL PRIMARY KEY,
    token VARCHAR(255) NOT NULL UNIQUE,
    "token_type" token_type NOT NULL,
    exp INT NOT NULL DEFAULT 3600,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

ALTER TABLE tokens
ADD COLUMN user_id UUID NOT NULL,
ADD CONSTRAINT fk_user_token
FOREIGN KEY (user_id)
REFERENCES "users" (id)
ON DELETE CASCADE; 

CREATE TRIGGER set_updated_at_tokens
BEFORE UPDATE ON tokens
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();