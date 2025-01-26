-- Add down migration script here

DROP TRIGGER set_updated_at_users ON public.users;

DROP TABLE public.users;

DROP TYPE public.activity_status_options;
DROP TYPE public.user_role;
