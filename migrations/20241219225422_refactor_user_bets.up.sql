-- Drop foreign key constraint if it exists
ALTER TABLE bets DROP CONSTRAINT bets_user_id_fkey;

-- Create user_bets table without foreign keys
CREATE TABLE IF NOT EXISTS user_bets (
    user_id VARCHAR(19) REFERENCES users (id),
    bet_id VARCHAR(19) REFERENCES bets (id)
);

-- Drop the outdated columns from the bets table
ALTER TABLE bets DROP COLUMN IF EXISTS user_id, DROP COLUMN IF EXISTS date;

-- Add created_at column to the bets table
ALTER TABLE bets
ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;
