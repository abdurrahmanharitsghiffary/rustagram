-- Add up migration script here

CREATE TYPE user_gender AS ENUM ('male', 'female', 'secret');

CREATE TABLE IF NOT EXISTS profiles (
    user_id UUID PRIMARY KEY,
    first_name VARCHAR(125) NOT NULL,
    last_name VARCHAR(125),
    bio TEXT,
    age INT,
    website VARCHAR(180),
    gender user_gender NOT NULL DEFAULT 'secret',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

ALTER TABLE profiles
ADD CONSTRAINT fk_user_profile
FOREIGN KEY (user_id)
REFERENCES users (id)
ON DELETE CASCADE;

CREATE TRIGGER set_updated_at_profiles
BEFORE UPDATE ON profiles
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();