-- Add down migration script here

DROP TRIGGER IF EXISTS set_updated_at_profiles ON profiles;

DROP TABLE IF EXISTS profiles CASCADE;

DROP TYPE IF EXISTS user_gender;
