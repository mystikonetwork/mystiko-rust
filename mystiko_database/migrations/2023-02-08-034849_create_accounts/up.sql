-- Your SQL goes here
CREATE TABLE accounts
(
    id                   VARCHAR(32) PRIMARY KEY,
    created_at           VARCHAR(32)  NOT NULL,
    updated_at           VARCHAR(32)  NOT NULL,
    name                 VARCHAR      NOT NULL,
    shielded_address     VARCHAR(128) NOT NULL,
    public_key           VARCHAR(160) NOT NULL,
    encrypted_secret_key VARCHAR      NOT NULL,
    status               VARCHAR      NOT NULL,
    scan_size            INTEGER      NOT NULL,
    wallet               VARCHAR(32) REFERENCES wallets (id)
);
CREATE
INDEX accounts_created_at_index ON accounts (created_at);
CREATE
INDEX accounts_updated_at_index ON accounts (updated_at);
CREATE
INDEX accounts_shielded_address_index ON accounts (shielded_address);
CREATE
INDEX accounts_public_key_index ON accounts (public_key);