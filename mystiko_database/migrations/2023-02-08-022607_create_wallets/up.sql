-- Your SQL goes here
CREATE TABLE wallets
(
    id                    VARCHAR(32) PRIMARY KEY,
    created_at            VARCHAR(32)  NOT NULL,
    updated_at            VARCHAR(32)  NOT NULL,
    encrypted_master_seed VARCHAR(255) NOT NULL,
    hashed_password       VARCHAR(255) NOT NULL,
    account_nonce         INTEGER      NOT NULL default 0
);
CREATE
INDEX wallets_created_at_index ON wallets (created_at);
CREATE
INDEX wallets_updated_at_index ON wallets (updated_at);