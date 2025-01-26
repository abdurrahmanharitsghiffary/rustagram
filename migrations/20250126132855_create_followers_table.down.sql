-- Add down migration script here

DROP TRIGGER set_updated_at_followersfollowings ON public.followers_followings;

ALTER TABLE public.followers_followings
DROP CONSTRAINT fk_followed_users;

ALTER TABLE public.followers_followings
DROP CONSTRAINT fk_following_users;

DROP TABLE followers_followings;

DROP TYPE following_status_options AS ENUM ('PENDING', 'ACCEPTED');