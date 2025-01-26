-- Add up migration script here

CREATE TYPE public.post_state_options AS ENUM (
    'DRAFT', 
    'PUBLISHED', 
    'PENDING'
    );

CREATE TYPE public.post_visibility_options AS ENUM (
    'PUBLIC', 
    'FOLLOWER', 
    'FOLLOWED', 
    'PRIVATE',
    'ARCHIVED' 
    );

CREATE TABLE public.posts (
    id SERIAL PRIMARY KEY NOT NULL,
    content TEXT,
    "state" public.post_state_options NOT NULL DEFAULT 'DRAFT',
    visibility public.post_visibility_options NOT NULL DEFAULT 'PUBLIC',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()    
);

CREATE INDEX idx_posts_visibility_public ON public.posts (visibility);

ALTER TABLE public.posts
ADD COLUMN user_id UUID NOT NULL,
ADD CONSTRAINT fk_post_users
FOREIGN KEY (user_id)
REFERENCES public.users (id)
ON DELETE CASCADE;

CREATE TRIGGER set_updated_at_posts
BEFORE UPDATE ON public.posts
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();


CREATE TABLE public.shared_posts (
    id SERIAL PRIMARY KEY NOT NULL,
    content TEXT,
    "state" public.post_state_options NOT NULL DEFAULT 'DRAFT',
    visibility public.post_visibility_options NOT NULL DEFAULT 'PUBLIC',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()    
);

ALTER TABLE public.shared_posts
ADD COLUMN origin_post_id INT NOT NULL,
ADD COLUMN user_id UUID NOT NULL,
ADD CONSTRAINT fk_shrdpost_users
FOREIGN KEY (user_id)
REFERENCES public.users (id)
ON DELETE CASCADE,
ADD CONSTRAINT fk_post_shrdpst
FOREIGN KEY (origin_post_id)
REFERENCES public.posts (id)
ON DELETE CASCADE,
ADD CONSTRAINT uniq_shrdpost
UNIQUE (origin_post_id, user_id);

CREATE TRIGGER set_updated_at_sharedposts
BEFORE UPDATE ON public.shared_posts
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();


CREATE TABLE public.post_attachments (
    id SERIAL PRIMARY KEY NOT NULL,
    src VARCHAR(375) NOT NULL,
    mime_type VARCHAR(50) NOT NULL
);

ALTER TABLE public.post_attachments
ADD COLUMN post_id INT NOT NULL,
ADD CONSTRAINT fk_post_attach
FOREIGN KEY (post_id)
REFERENCES public.posts (id)
ON DELETE CASCADE,
ADD CONSTRAINT uniq_src
UNIQUE (src);

CREATE TABLE public.post_tags (
    id SERIAL PRIMARY KEY NOT NULL,
    tag VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TRIGGER set_updated_at_posttags
BEFORE UPDATE ON public.post_tags
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

CREATE TABLE public.posts_tags (
    tag_id INT NOT NULL,
    post_id INT NOT NULL,
    CONSTRAINT pk_post_tags PRIMARY KEY (tag_id, post_id),
    CONSTRAINT fk_reaction_user FOREIGN KEY (tag_id) REFERENCES public.post_tags (id),
    CONSTRAINT fk_reaction_post FOREIGN KEY (post_id) REFERENCES public.posts (id)
);

CREATE TYPE public.reactions_options AS ENUM (
    'THUMB', 
    'LOVE', 
    'ANGRY', 
    'HAPPY', 
    'SAD',
    'CRY',
    'OMG'
    );

CREATE TABLE public.post_reactions (
    reaction public.reactions_options NOT NULL DEFAULT 'THUMB',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),

    user_id UUID NOT NULL,
    post_id INT NOT NULL,
    CONSTRAINT pk_post_reactions PRIMARY KEY (user_id, post_id),
    CONSTRAINT fk_reaction_user FOREIGN KEY (user_id) REFERENCES public.users (id),
    CONSTRAINT fk_reaction_post FOREIGN KEY (post_id) REFERENCES public.posts (id)
);

CREATE TRIGGER set_updated_at_postreactions
BEFORE UPDATE ON public.post_reactions
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

CREATE TABLE public.post_comments (
    id SERIAL PRIMARY KEY NOT NULL,
    content TEXT,
    gif VARCHAR(325),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),

    user_id UUID NOT NULL,
    post_id INT NOT NULL,
    CONSTRAINT fk_reaction_user FOREIGN KEY (user_id) REFERENCES public.users (id),
    CONSTRAINT fk_reaction_post FOREIGN KEY (post_id) REFERENCES public.posts (id)
);

CREATE TRIGGER set_updated_at_postcomments
BEFORE UPDATE ON public.post_comments
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();