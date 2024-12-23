-- Drop the foreign key constraint on 'bookmaker_id'
ALTER TABLE bets
    DROP CONSTRAINT IF EXISTS fk_bookmaker_id;

-- Revert the column rename from 'bookmaker_id' back to 'casino'
ALTER TABLE bets RENAME COLUMN bookmaker_id TO casino;

-- Remove any rows from 'bets' that reference 'bookmakers' in 'bookmaker_id'
-- (if any exist, but ensure it doesn't cause data issues)
-- You may choose to delete or nullify rows depending on the business logic
UPDATE bets
SET casino = '1040465407128896523'
WHERE casino IN (SELECT id FROM bookmakers);

-- Drop the 'bookmakers' table if it's no longer needed
DROP TABLE IF EXISTS bookmakers;
