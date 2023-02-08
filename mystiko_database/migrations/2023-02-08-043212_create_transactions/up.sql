-- Your SQL goes here
CREATE TABLE transactions
(
    id                          VARCHAR(32) PRIMARY KEY,
    created_at                  VARCHAR(32) NOT NULL,
    updated_at                  VARCHAR(32) NOT NULL,
    chain_id                    INTEGER     NOT NULL,
    contract_address            VARCHAR(64) NOT NULL,
    asset_symbol                VARCHAR     NOT NULL,
    asset_decimals              INTEGER     NOT NULL,
    asset_address               VARCHAR,
    proof                       VARCHAR,
    root_hash                   VARCHAR(128),
    input_commitments           TEXT[] NOT NULL,
    output_commitments          TEXT[],
    serial_numbers              TEXT[],
    signature_public_key        VARCHAR(128),
    signature_public_key_hashes TEXT[],
    amount                      VARCHAR     NOT NULL,
    public_amount               VARCHAR     NOT NULL,
    rollup_fee_amount           VARCHAR     NOT NULL,
    gas_relayer_fee_amount      VARCHAR     NOT NULL,
    shielded_address            VARCHAR(128),
    public_address              VARCHAR(64),
    gas_relayer_address         VARCHAR,
    signature                   VARCHAR,
    random_auditing_public_key  VARCHAR,
    encrypted_auditor_notes     TEXT[],
    type                        VARCHAR(32) NOT NULL,
    status                      VARCHAR     NOT NULL,
    error_message               VARCHAR,
    transaction_hash            VARCHAR(128),
    wallet                      VARCHAR(32) REFERENCES wallets (id)
);

CREATE
INDEX transactions_created_at_index ON transactions (created_at);
CREATE
INDEX transactions_updated_at_index ON transactions (updated_at);
CREATE
INDEX transactions_chain_id_index ON transactions (chain_id);
CREATE
INDEX transactions_contract_address_index ON transactions (contract_address);
CREATE
INDEX transactions_signature_public_key_index  ON transactions (signature_public_key);
CREATE
INDEX transactions_root_hash_index  ON transactions (root_hash);
CREATE
INDEX transactions_shielded_address_index  ON transactions (shielded_address);
CREATE
INDEX transactions_public_address_index  ON transactions (public_address);
CREATE
INDEX transactions_type_index  ON transactions (type);
CREATE
INDEX transactions_transaction_hash_index  ON transactions (transaction_hash);
CREATE
INDEX transactions_wallet_index  ON transactions (wallet);