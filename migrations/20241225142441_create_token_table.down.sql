-- Add down migration script here

DROP TRIGGER set_updated_at_tokens ON public.tokens;

ALTER TABLE public.tokens
    DROP CONSTRAINT fk_user_token;

ALTER TABLE public.tokens
    DROP COLUMN user_id;

DROP TABLE public.tokens;

DROP TYPE public.token_type_options;