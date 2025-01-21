-- Add down migration script here

DROP TABLE IF EXISTS user_notification_settings CASCADE;

DROP TYPE IF EXISTS notification_settings_options CASCADE;
DROP TYPE IF EXISTS notification_type_options CASCADE;

DROP TABLE IF EXISTS user_settings CASCADE;

DROP TYPE IF EXISTS sensitivity_content_control_options CASCADE;
DROP TYPE IF EXISTS application_lang_options CASCADE;
DROP TYPE IF EXISTS group_add_permission_options CASCADE;
DROP TYPE IF EXISTS followers_messaging_permission_options CASCADE;
DROP TYPE IF EXISTS others_messaging_permission_options CASCADE;
DROP TYPE IF EXISTS activity_status_visibility_options CASCADE;
DROP TYPE IF EXISTS tag_permission_options CASCADE;
DROP TYPE IF EXISTS mention_permission_options CASCADE;
DROP TYPE IF EXISTS comments_permission_options CASCADE;

