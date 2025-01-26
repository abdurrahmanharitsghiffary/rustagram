-- Add up migration script here

CREATE TYPE public.account_type_options AS ENUM (
    'OIDC',
    'OAUTH_DOT_2',
    'SAML',
    'CREDENTIALS'
    );


CREATE TABLE public.accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id VARCHAR(125),
    account_hashed_password VARCHAR(255),
    is_identity_verified BOOLEAN NOT NULL DEFAULT FALSE,
    provider_id VARCHAR(125) NOT NULL DEFAULT 'app-credentials',
    provider_type public.account_type_options NOT NULL DEFAULT 'CREDENTIALS',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

ALTER TABLE public.accounts
    ADD COLUMN user_id UUID NOT NULL,
    ADD CONSTRAINT fk_user_accounts
    FOREIGN KEY (user_id)
    REFERENCES public.users (id),
    ADD CONSTRAINT uniq_accid_provid_provtype
    UNIQUE (account_id, provider_id, provider_type);

CREATE TRIGGER set_updated_at_accounts
BEFORE UPDATE ON public.accounts
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

CREATE TABLE public.account_sessions (
    id SERIAL PRIMARY KEY NOT NULL,
    refresh_token VARCHAR(255) NOT NULL,
    user_agent VARCHAR(255) NOT NULL,
    hashed_ipv4_address VARCHAR(255),
    hashed_ipv6_address VARCHAR(255),
    expires_at VARCHAR(50) NOT NULL DEFAULT '604800',
    is_revoked BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

ALTER TABLE public.account_sessions
    ADD COLUMN user_id UUID NOT NULL,
    ADD CONSTRAINT fk_account_accsess
    FOREIGN KEY (user_id)
    REFERENCES public.accounts (id),
    ADD CONSTRAINT uniq_refreshtoken
    UNIQUE (refresh_token);

CREATE TRIGGER set_updated_at_accountsessions
BEFORE UPDATE ON public.account_sessions
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

