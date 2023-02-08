-- Your SQL goes here
CREATE TABLE commitments
(
    id                        VARCHAR(32) PRIMARY KEY,
    created_at                VARCHAR(32)  NOT NULL,
    updated_at                VARCHAR(32)  NOT NULL,
    chain_id                  INTEGER      NOT NULL,
    contract_address          VARCHAR(64)  NOT NULL,
    commitment_hash           VARCHAR(128) NOT NULL,
    asset_symbol              VARCHAR(255) NOT NULL,
    asset_decimals            INTEGER      NOT NULL,
    asset_address             VARCHAR(255),
    status                    VARCHAR(255) NOT NULL,
    rollup_fee_amount         VARCHAR(255),
    encrypted_note            VARCHAR(255),
    leaf_index                VARCHAR(255),
    amount                    VARCHAR(255),
    serial_number             VARCHAR(128),
    shielded_address          VARCHAR(128),
    creation_transaction_hash VARCHAR(128),
    spending_transaction_hash VARCHAR(128),
    rollup_transaction_hash   VARCHAR(128)
);

CREATE
INDEX commitments_created_at_index ON commitments (created_at);
CREATE
INDEX commitments_updated_at_index ON commitments (updated_at);
CREATE
INDEX commitments_chain_id_index ON commitments (chain_id);
CREATE
INDEX commitments_contract_address_index ON commitments (contract_address);
CREATE
INDEX commitments_commitment_hash_index ON commitments (commitment_hash);
CREATE
INDEX commitments_shielded_address_index ON commitments (shielded_address);
CREATE
INDEX commitments_serial_number_index ON commitments (serial_number);
CREATE
INDEX commitments_creation_transaction_hash_index ON commitments (creation_transaction_hash);
CREATE
INDEX commitments_spending_transaction_hash_index ON commitments (spending_transaction_hash);
CREATE
INDEX commitments_rollup_transaction_hash_index ON commitments (rollup_transaction_hash);