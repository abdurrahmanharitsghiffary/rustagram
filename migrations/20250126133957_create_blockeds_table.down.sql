-- Add down migration script here

DROP TRIGGER set_updated_at_blockersblocked ON public.blockers_blocked;

ALTER TABLE public.blockers_blocked
DROP CONSTRAINT fk_blocker_users;

ALTER TABLE public.blockers_blocked
DROP CONSTRAINT fk_blocked_users;

DROP TABLE blockers_blocked;