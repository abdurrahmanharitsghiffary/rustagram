-- Add up migration script here
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();  -- Set the updated_at field to the current timestamp
    RETURN NEW;  -- Return the new row
END;
$$ LANGUAGE plpgsql;