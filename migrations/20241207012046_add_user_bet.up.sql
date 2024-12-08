-- Add up migration script here
CREATE TABLE IF NOT EXISTS bets (
    id VARCHAR(19) PRIMARY KEY,
    user_id VARCHAR(19) REFERENCES users (id),
    amount DECIMAL(10, 2) NOT NULL,
    casino TEXT NOT NULL,
    date DATE DEFAULT CURRENT_DATE NOT NULL
);
