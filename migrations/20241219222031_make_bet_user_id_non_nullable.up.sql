-- Add up migration script here
ALTER TABLE bets ALTER COLUMN user_id SET NOT NULL;
