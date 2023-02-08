-- Your SQL goes here
CREATE TABLE nullifiers
(
    id               VARCHAR(32) PRIMARY KEY,
    created_at       VARCHAR(32)  NOT NULL,
    updated_at       VARCHAR(32)  NOT NULL,
    chain_id         INTEGER      NOT NULL,
    contract_address VARCHAR(64)  NOT NULL,
    serial_number VARCHAR(128) NOT NULL,
    transaction_hash VARCHAR(128) NOT NULL
);
CREATE INDEX nullifiers_created_at_index ON nullifiers (created_at);
CREATE INDEX nullifiers_updated_at_index ON nullifiers (updated_at);
CREATE INDEX nullifiers_chain_id_index ON nullifiers (chain_id);
CREATE INDEX nullifiers_contract_address_index ON nullifiers (contract_address);
CREATE INDEX nullifiers_serial_number_index ON nullifiers (serial_number);
CREATE INDEX nullifiers_transaction_hash_index ON nullifiers (transaction_hash);
