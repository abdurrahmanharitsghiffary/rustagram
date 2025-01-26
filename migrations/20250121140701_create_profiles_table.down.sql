-- Add down migration script here

DROP TRIGGER set_updated_at_profiles ON public.profiles;

ALTER TABLE public.profiles
    DROP CONSTRAINT fk_user_profile;

DROP TABLE public.profiles;

DROP TYPE public.gender_options;
