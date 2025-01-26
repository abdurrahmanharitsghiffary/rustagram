-- Add down migration script here

DROP TRIGGER set_updated_at_postcomments ON public.post_comments;
DROP TABLE public.post_comments;

DROP TRIGGER set_updated_at_postreactions ON public.post_reactions;
DROP TABLE public.post_reactions;

DROP TABLE public.posts_tags;

DROP TRIGGER set_updated_at_posttags ON public.post_tags;
DROP TABLE public.post_tags;

ALTER TABLE public.post_attachments
DROP CONSTRAINT uniq_src,
DROP CONSTRAINT fk_post_attach;

DROP TABLE public.post_attachments;

DROP TRIGGER set_updated_at_sharedposts ON public.shared_posts;
DROP TABLE public.shared_posts;

DROP TRIGGER set_updated_at_posts ON public.posts;
DROP INDEX idx_posts_visibility_public;
DROP TABLE public.posts;

DROP TYPE public.reactions_options;
DROP TYPE public.post_visibility_options;
DROP TYPE public.post_state_options;
