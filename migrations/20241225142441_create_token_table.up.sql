-- Add up migration script here

CREATE TYPE public.token_type_options AS ENUM (
    'VERIFY', 
    'RESET', 
    'OTP'
    );

CREATE TABLE IF NOT EXISTS public.tokens (
    id SERIAL PRIMARY KEY NOT NULL,
    token VARCHAR(255) NOT NULL UNIQUE,
    token_type public.token_type_options NOT NULL,
    exp INT NOT NULL DEFAULT 3600,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

ALTER TABLE public.tokens
    ADD COLUMN user_id UUID NOT NULL,
    ADD CONSTRAINT fk_user_token
    FOREIGN KEY (user_id)
    REFERENCES public.users (id)
    ON DELETE CASCADE; 

CREATE TRIGGER set_updated_at_tokens
BEFORE UPDATE ON public.tokens
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();