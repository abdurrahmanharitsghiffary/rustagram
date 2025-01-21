-- Add up migration script here

CREATE TYPE user_role AS ENUM ('admin', 'manager', 'user', 'user_bussiness');
CREATE TYPE visibility_type AS ENUM ('private', 'public');

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(100) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    region VARCHAR(20) NOT NULL,
    "role" user_role NOT NULL DEFAULT 'user',
    account_visibility visibility_type NOT NULL DEFAULT 'public',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);


CREATE TRIGGER set_updated_at_users
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();