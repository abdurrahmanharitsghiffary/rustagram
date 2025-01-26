-- Add up migration script here

CREATE TYPE public.gender_options AS ENUM ('MALE', 'FEMALE', 'SECRET');

CREATE TABLE public.profiles (
    user_id UUID PRIMARY KEY,
    first_name VARCHAR(125) NOT NULL,
    last_name VARCHAR(125),
    bio TEXT,
    age INT,
    website VARCHAR(180),
    gender public.gender_options NOT NULL DEFAULT 'SECRET',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

ALTER TABLE public.profiles
ADD CONSTRAINT fk_user_profile
FOREIGN KEY (user_id)
REFERENCES public.users (id)
ON DELETE CASCADE;

CREATE TRIGGER set_updated_at_profiles
BEFORE UPDATE ON public.profiles
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();