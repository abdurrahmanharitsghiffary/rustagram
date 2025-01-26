-- Add up migration script here

CREATE TYPE public.user_role AS ENUM (
    'ADMIN', 
    'MANAGER', 
    'CREATOR', 
    'BUSSINESS'
    );

CREATE TYPE public.activity_status_options AS ENUM (
    'OFFLINE', 
    'ONLINE', 
    'HIDDEN'
    );

CREATE TABLE IF NOT EXISTS public.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(100) NOT NULL UNIQUE,
    region VARCHAR(20) NOT NULL,
    "role" public.user_role NOT NULL DEFAULT 'CREATOR',
    activity_status public.activity_status_options NOT NULL DEFAULT 'OFFLINE',
    last_online TIMESTAMP DEFAULT NOW(),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);


CREATE TRIGGER set_updated_at_users
BEFORE UPDATE ON public.users
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();