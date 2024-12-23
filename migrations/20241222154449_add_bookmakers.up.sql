-- Add up migration script here
CREATE TABLE IF NOT EXISTS bookmakers (
    id VARCHAR(19) PRIMARY KEY DEFAULT id_generator(),
    label TEXT NOT NULL,
    slug TEXT NOT NULL,
    url TEXT NOT NULL,
    scope TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);

-- Ensure that there are valid values in bookmakers
-- (You can insert some sample values if needed)
INSERT INTO bookmakers (id, label, slug, url, scope)
VALUES ('1040465407128896523', 'Sample Bookmaker', 'sample-bookmaker', 'http://example.com', 'bookmaker')
ON CONFLICT (id) DO NOTHING;  -- Avoids duplicates

-- Rename the column 'casino' to 'bookmaker_id'
ALTER TABLE bets RENAME COLUMN casino TO bookmaker_id;

-- Ensure bookmaker_id has valid values (or NULL) before applying the foreign key
UPDATE bets
SET bookmaker_id = '1040465407128896523'  -- Set to NULL or a valid bookmaker_id
WHERE bookmaker_id NOT IN (SELECT id FROM bookmakers);

-- Add the foreign key constraint to reference the 'bookmakers.id' column
ALTER TABLE bets
    ADD CONSTRAINT fk_bookmaker_id FOREIGN KEY (bookmaker_id)
    REFERENCES bookmakers(id)
    ON DELETE SET NULL;
