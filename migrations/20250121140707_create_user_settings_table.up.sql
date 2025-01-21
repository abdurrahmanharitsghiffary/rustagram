-- Add up migration script here

CREATE TYPE sensitivity_content_control_options AS ENUM ('less', 'standard', 'more');
CREATE TYPE application_lang_options AS ENUM ('en-US', 'id')
CREATE TYPE group_add_permission_options AS ENUM ('everyone', 'followed')
CREATE TYPE followers_messaging_permission_options AS ENUM ('yes', 'no')
CREATE TYPE others_messaging_permission_options AS ENUM ('yes', 'no')
CREATE TYPE activity_status_visibility_options AS ENUM ('yes', 'no')
CREATE TYPE tag_permission_options AS ENUM ('everyone', 'followed', 'no')
CREATE TYPE mention_permission_options AS ENUM ('everyone', 'followed', 'no')
CREATE TYPE comments_permission_options AS ENUM ('everyone', 'followed', 'followers', 'followed_and_followers')


CREATE TABLE IF NOT EXISTS user_settings (
    id SERIAL NOT NULL,
    sensitive_content_control_option sensitive_content_control_options NOT NULL DEFAULT 'standard',
    application_lang application_lang_options NOT NULL DEFAULT 'en-US',
    group_add_permission group_add_permission_options NOT NULL DEFAULT 'everyone',
    followers_messaging_permission followers_messaging_permission_options NOT NULL DEFAULT 'yes',
    others_messaging_permission others_messaging_permission_options NOT NULL DEFAULT 'yes',
    activity_status_visibility activity_status_visibility_options NOT NULL DEFAULT 'yes',
    tag_permission tag_permission_options NOT NULL DEFAULT 'followed',
    mention_permission mention_permission_options NOT NULL DEFAULT 'followed',
    comments_permission comments_permission_options NOT NULL DEFAULT 'everyone',
    manually_approve_tag BOOLEAN NOT NULL DEFAULT false,
    allow_others_share_post BOOLEAN NOT NULL DEFAULT true,
    timezone VARCHAR(50) NOT NULL DEFAULT 'UTC',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

ALTER TABLE user_settings
ADD COLUMN user_id UUID NOT NULL,
ADD CONSTRAINT fk_user_usersettings
FOREIGN KEY (user_id)
REFERENCES users (id)
ON DELETE CASCADE;

CREATE TRIGGER set_updated_at_usersettings
BEFORE UPDATE ON user_settings
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();


CREATE TYPE notification_settings_options AS ENUM ('off', 'on', 'everyone', 'less', 'default');
CREATE TYPE notification_type_options AS ENUM ('email', 'push');

CREATE TABLE IF NOT EXISTS user_notification_settings (
    id SERIAL PRIMARY KEY,
    scope VARCHAR(50) NOT NULL,
    notification_type notification_type_options NOT NULL,
    notification_option notification_settings_options NOT NULL DEFAULT 'default', 
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

ALTER TABLE user_notification_settings
ADD COLUMN setting_id INT NOT NULL,
ADD CONSTRAINT fk_usset_usernotificationsettings
FOREIGN KEY (setting_id)
REFERENCES user_notification_settings (id),
ADD CONSTRAINT uniq_usset_usernotificationsettings
UNIQUE KEY (setting_id)
ON DELETE CASCADE;

ALTER TABLE user_notification_settings
ADD CONSTRAINT unique_userid_scope
UNIQUE KEY (user_id, scope);

CREATE TRIGGER set_updated_at_usernotificationsettings
BEFORE UPDATE ON user_notification_settings
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();