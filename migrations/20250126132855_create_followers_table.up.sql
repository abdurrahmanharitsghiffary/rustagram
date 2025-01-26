-- Add up migration script here

CREATE TYPE following_status_options AS ENUM ('PENDING', 'ACCEPTED');

CREATE TABLE public.followers_followings (
    followed_user_id UUID NOT NULL, 
    following_user_id UUID NOT NULL,
    following_status public.following_status_options NOT NULL DEFAULT 'ACCEPTED',
    unfollowed_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    CONSTRAINT pk_following_followed PRIMARY KEY (followed_user_id, following_user_id)
);

ALTER TABLE public.followers_followings
ADD CONSTRAINT fk_followed_users
FOREIGN KEY (followed_user_id) 
REFERENCES public.users (id);

ALTER TABLE public.followers_followings
ADD CONSTRAINT fk_following_users
FOREIGN KEY (following_user_id) 
REFERENCES public.users (id);

CREATE TRIGGER set_updated_at_followersfollowings
BEFORE UPDATE ON public.followers_followings
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();