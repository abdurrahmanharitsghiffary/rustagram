-- Add down migration script here

DROP TRIGGER set_updated_at_accountsessions ON public.account_sessions;

ALTER TABLE public.account_sessions
    DROP CONSTRAINT uniq_refreshtoken,
    DROP CONSTRAINT fk_account_accsess,
    DROP COLUMN user_id;

DROP TABLE public.account_sessions;


DROP TRIGGER set_updated_at_accounts ON public.accounts;

ALTER TABLE public.accounts
    DROP CONSTRAINT uniq_accid_provid_provtype,
    DROP CONSTRAINT fk_user_accounts,
    DROP COLUMN user_id;

DROP TABLE public.accounts;

DROP TYPE public.account_type_options;