-- Your SQL goes here
CREATE TABLE contracts
(
    id                  VARCHAR(32) PRIMARY KEY,
    created_at          VARCHAR(32)  NOT NULL,
    updated_at          VARCHAR(32)  NOT NULL,
    type                VARCHAR(255) NOT NULL,
    chain_id            INTEGER      NOT NULL,
    contract_address    VARCHAR(64)  NOT NULL,
    disabled            INTEGER      NOT NULL default 0,
    sync_start          VARCHAR(255) NOT NULL,
    sync_size           VARCHAR(255) NOT NULL,
    synced_block_number VARCHAR(255) NOT NULL,
    checked_leaf_index  VARCHAR(255)
);

CREATE
INDEX contracts_created_at_index ON contracts (created_at);
CREATE
INDEX contracts_updated_at_index ON contracts (updated_at);
CREATE
INDEX contracts_chain_id_index ON contracts (chain_id);
CREATE
INDEX contracts_contract_address_index ON contracts (contract_address);