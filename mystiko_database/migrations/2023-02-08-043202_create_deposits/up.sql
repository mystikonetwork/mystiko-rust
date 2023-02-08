-- Your SQL goes here
CREATE TABLE deposits
(
    id                         VARCHAR(32) PRIMARY KEY,
    created_at                 VARCHAR(32)  NOT NULL,
    updated_at                 VARCHAR(32)  NOT NULL,
    chain_id                   INTEGER      NOT NULL,
    contract_address           VARCHAR(64)  NOT NULL,
    pool_address                VARCHAR      NOT NULL,
    commitment_hash             VARCHAR(128) NOT NULL,
    hash_k                      VARCHAR      NOT NULL,
    random_s                    VARCHAR      NOT NULL,
    encrypted_note              VARCHAR      NOT NULL,
    asset_symbol                VARCHAR      NOT NULL,
    asset_decimals              INTEGER      NOT NULL,
    asset_address               VARCHAR,
    bridge_type                 VARCHAR      NOT NULL,
    amount                     VARCHAR      NOT NULL,
    rollup_fee_amount            VARCHAR      NOT NULL,
    bridge_fee_amount            VARCHAR      NOT NULL,
    bridge_fee_asset_address      VARCHAR,
    executor_fee_amount          VARCHAR      NOT NULL,
    executor_fee_asset_address    VARCHAR,
    service_fee_amount           VARCHAR      NOT NULL,
    shielded_recipient_address   VARCHAR(128) NOT NULL,
    status                     VARCHAR      NOT NULL,
    errormessage               VARCHAR,
    wallet                     VARCHAR(32) REFERENCES wallets (id),
    dst_chain_id                 INTEGER      NOT NULL,
    dst_chain_contract_address    VARCHAR(64)  NOT NULL,
    dst_pool_address             VARCHAR      NOT NULL,
    asset_approve_transaction_hash VARCHAR(128),
    transaction_hash            VARCHAR(128),
    relay_transaction_hash        VARCHAR(128),
    rollup_transaction_hash       VARCHAR(128)
);
CREATE INDEX deposits_created_at_index ON deposits (created_at);
CREATE INDEX deposits_updated_at_index ON deposits (updated_at);
CREATE INDEX deposits_chain_id_index ON deposits (chain_id);
CREATE INDEX deposits_contract_address_index ON deposits (contract_address);
CREATE INDEX deposits_commitment_hash_index ON deposits (commitment_hash);
CREATE INDEX deposits_dst_chain_id_index  ON deposits (dst_chain_id);
CREATE INDEX deposits_dst_chain_contract_address_index  ON deposits (dst_chain_contract_address);
CREATE INDEX deposits_shielded_recipient_address_index  ON deposits (shielded_recipient_address);
CREATE INDEX deposits_asset_approve_transaction_hash_index  ON deposits (asset_approve_transaction_hash);
CREATE INDEX deposits_transaction_hash_index  ON deposits (transaction_hash);
CREATE INDEX deposits_relay_transaction_hash_index  ON deposits (relay_transaction_hash);
CREATE INDEX deposits_rollup_transaction_hash_index  ON deposits (rollup_transaction_hash);