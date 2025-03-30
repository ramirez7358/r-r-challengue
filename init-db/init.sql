CREATE SCHEMA IF NOT EXISTS cryptocurrency_transactions;

CREATE TABLE IF NOT EXISTS cryptocurrency_transactions.transactions (
    id SERIAL PRIMARY KEY,
    address_from VARCHAR(255) NOT NULL,
    address_to VARCHAR(255) NOT NULL,
    amount NUMERIC(30,10) NOT NULL,
    type VARCHAR(10) NOT NULL CHECK (type IN ('deposit', 'withdrawal')),
    created_at TIMESTAMP DEFAULT NOW()
);
