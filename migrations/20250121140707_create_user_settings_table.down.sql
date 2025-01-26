-- Add down migration script here

DROP TRIGGER set_updated_at_usernotificationsettings ON public.user_notification_settings;

ALTER TABLE public.user_notification_settings
    DROP CONSTRAINT uniq_usset_usernotificationsettings,
    DROP CONSTRAINT fk_usset_usernotificationsettings,
    DROP COLUMN setting_id;

DROP TABLE public.user_notification_settings;


DROP TYPE public.notification_type_options;
DROP TYPE public.notification_settings_options;

DROP TRIGGER set_updated_at_usersettings ON public.user_settings;

ALTER TABLE public.user_settings
    DROP CONSTRAINT fk_user_usersettings,
    DROP COLUMN user_id;

DROP TABLE public.user_settings;

DROP TYPE public.account_visibility_options;
DROP TYPE public.comments_permission_options;
DROP TYPE public.mention_permission_options;
DROP TYPE public.tag_permission_options;
DROP TYPE public.activity_status_visibility_options;
DROP TYPE public.others_messaging_permission_options;
DROP TYPE public.followers_messaging_permission_options;
DROP TYPE public.group_add_permission_options;
DROP TYPE public.sensitivity_content_control_options;
