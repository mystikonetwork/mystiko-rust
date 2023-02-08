-- Your SQL goes here
CREATE TABLE chains
(
    id                  VARCHAR(32) PRIMARY KEY,
    created_at          VARCHAR(32)  NOT NULL,
    updated_at          VARCHAR(32)  NOT NULL,
    chain_id            INTEGER      NOT NULL,
    name                VARCHAR(255) NOT NULL,
    name_override       INTEGER,
    providers           text         NOT NULL,
    provider_override   INTEGER,
    event_filter_size   INTEGER      NOT NULL,
    synced_block_number INTEGER      NOT NULL
);
CREATE
INDEX chains_created_at_index ON chains (created_at);
CREATE
INDEX chains_updated_at_index ON chains (updated_at);
CREATE
INDEX chains_chain_id_index ON chains (chain_id);
