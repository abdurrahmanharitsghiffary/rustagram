-- Add up migration script here

CREATE TABLE public.blockers_blocked (
    blocked_id UUID NOT NULL, 
    blocker_id UUID NOT NULL,
    unblocked_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    CONSTRAINT pk_blocker_blocked PRIMARY KEY (blocker_id, blocked_id)
);

ALTER TABLE public.blockers_blocked
ADD CONSTRAINT fk_blocked_users
FOREIGN KEY (blocked_id) 
REFERENCES public.users (id);

ALTER TABLE public.blockers_blocked
ADD CONSTRAINT fk_blocker_users
FOREIGN KEY (blocker_id) 
REFERENCES public.users (id);

CREATE TRIGGER set_updated_at_blockersblocked
BEFORE UPDATE ON public.blockers_blocked
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();