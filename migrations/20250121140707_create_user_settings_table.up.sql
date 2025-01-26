-- Add up migration script here

CREATE TYPE public.sensitivity_content_control_options AS ENUM ('LESS', 'STANDARD', 'MORE');
CREATE TYPE public.group_add_permission_options AS ENUM ('EVERYONE', 'FOLLOWED');
CREATE TYPE public.followers_messaging_permission_options AS ENUM ('YES', 'NO');
CREATE TYPE public.others_messaging_permission_options AS ENUM ('YES', 'NO');
CREATE TYPE public.activity_status_visibility_options AS ENUM ('YES', 'NO');
CREATE TYPE public.tag_permission_options AS ENUM ('EVERYONE', 'FOLLOWED', 'NO');
CREATE TYPE public.mention_permission_options AS ENUM ('EVERYONE', 'FOLLOWED', 'NO');
CREATE TYPE public.comments_permission_options AS ENUM ('EVERYONE', 'FOLLOWED', 'FOLLOWERS', 'FOLLOWED_AND_FOLLOWERS');
CREATE TYPE public.account_visibility_options AS ENUM ('PRIVATE', 'PUBLIC');

CREATE TABLE public.user_settings (
    id SERIAL PRIMARY KEY NOT NULL,
    account_visibility_option public.account_visibility_options NOT NULL DEFAULT 'PUBLIC',
    sensitive_content_control_option public.sensitivity_content_control_options NOT NULL DEFAULT 'STANDARD',
    application_lang VARCHAR(50) NOT NULL DEFAULT 'en-US',
    group_add_permission public.group_add_permission_options NOT NULL DEFAULT 'EVERYONE',
    followers_messaging_permission public.followers_messaging_permission_options NOT NULL DEFAULT 'YES',
    others_messaging_permission public.others_messaging_permission_options NOT NULL DEFAULT 'YES',
    activity_status_visibility public.activity_status_visibility_options NOT NULL DEFAULT 'YES',
    tag_permission public.tag_permission_options NOT NULL DEFAULT 'FOLLOWED',
    mention_permission public.mention_permission_options NOT NULL DEFAULT 'FOLLOWED',
    comments_permission public.comments_permission_options NOT NULL DEFAULT 'EVERYONE',
    manually_approve_tag BOOLEAN NOT NULL DEFAULT FALSE,
    allow_others_share_post BOOLEAN NOT NULL DEFAULT TRUE,
    timezone VARCHAR(50) NOT NULL DEFAULT 'UTC',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

ALTER TABLE public.user_settings
    ADD COLUMN user_id UUID NOT NULL,
    ADD CONSTRAINT fk_user_usersettings
    FOREIGN KEY (user_id)
    REFERENCES public.users (id)
    ON DELETE CASCADE;

CREATE TRIGGER set_updated_at_usersettings
BEFORE UPDATE ON public.user_settings
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

CREATE TYPE public.notification_settings_options AS ENUM ('OFF', 'ON', 'EVERYONE', 'LESS', 'DEFAULT');
CREATE TYPE public.notification_type_options AS ENUM ('EMAIL', 'PUSH');


CREATE TABLE public.user_notification_settings (
    id SERIAL PRIMARY KEY NOT NULL,
    scope VARCHAR(50) NOT NULL,
    notification_type public.notification_type_options NOT NULL,
    notification_option public.notification_settings_options NOT NULL DEFAULT 'DEFAULT', 
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

ALTER TABLE public.user_notification_settings
    ADD COLUMN setting_id INT NOT NULL,
    ADD CONSTRAINT fk_usset_usernotificationsettings
    FOREIGN KEY (setting_id)
    REFERENCES public.user_settings (id),
    ADD CONSTRAINT uniq_usset_usernotificationsettings
    UNIQUE (setting_id, scope);

CREATE TRIGGER set_updated_at_usernotificationsettings
BEFORE UPDATE ON public.user_notification_settings
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

