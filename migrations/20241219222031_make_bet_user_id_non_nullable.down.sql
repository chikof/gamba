-- Add down migration script here
ALTER TABLE bets ALTER COLUMN user_id DROP NOT NULL;
