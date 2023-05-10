#![forbid(unsafe_code)]

use anyhow::Result;
use mystiko_storage::document::{DocumentData, DocumentRawData, DocumentSchema};
use mystiko_storage::error::StorageError;
use mystiko_types::CommitmentStatus;
use num_bigint::BigInt;

pub const CHAIN_ID_FIELD_NAME: &str = "chain_id";
pub const CONTRACT_ADDRESS_FIELD_NAME: &str = "contract_address";
pub const COMMITMENT_HASH_FIELD_NAME: &str = "commitment_hash";
pub const ASSET_SYMBOL_FIELD_NAME: &str = "asset_symbol";
pub const ASSET_DECIMALS_FIELD_NAME: &str = "asset_decimals";
pub const ASSET_ADDRESS_FIELD_NAME: &str = "asset_address";
pub const STATUS_FIELD_NAME: &str = "status";
pub const ROLLUP_FEE_AMOUNT_FIELD_NAME: &str = "rollup_fee_amount";
pub const ENCRYPTED_NOTE_FIELD_NAME: &str = "encrypted_note";
pub const LEAF_INDEX_FIELD_NAME: &str = "leaf_index";
pub const AMOUNT_FIELD_NAME: &str = "amount";
pub const NULLIFIER_FIELD_NAME: &str = "nullifier";
pub const SHIELDED_ADDRESS_FIELD_NAME: &str = "shielded_address";
pub const CREATION_TRANSACTION_HASH_FIELD_NAME: &str = "creation_transaction_hash";
pub const SPENDING_TRANSACTION_HASH_FIELD_NAME: &str = "spending_transaction_hash";
pub const ROLLUP_TRANSACTION_HASH_FIELD_NAME: &str = "rollup_transaction_hash";

pub static COMMITMENT_SCHEMA: DocumentSchema = DocumentSchema {
    collection_name: "commitments",
    migrations: &[
        "CREATE TABLE `commitments` (\
            `id`                        VARCHAR(64) NOT NULL PRIMARY KEY,\
            `created_at`                INT          NOT NULL,\
            `updated_at`                INT          NOT NULL,\
            `chain_id`                  BIGINT       NOT NULL,\
            `contract_address`          VARCHAR(64)  NOT NULL,\
            `commitment_hash`           VARCHAR(128) NOT NULL,\
            `asset_symbol`              VARCHAR(16) NOT NULL,\
            `asset_decimals`            INT          NOT NULL,\
            `asset_address`             VARCHAR(64),\
            `status`                    VARCHAR(32) NOT NULL,\
            `rollup_fee_amount`         VARCHAR(128),\
            `encrypted_note`            TEXT,\
            `leaf_index`                VARCHAR(128),\
            `amount`                    VARCHAR(128),\
            `nullifier`                 VARCHAR(128),\
            `shielded_address`          VARCHAR(128),\
            `creation_transaction_hash` VARCHAR(128),\
            `spending_transaction_hash` VARCHAR(128),\
            `rollup_transaction_hash`   VARCHAR(128),\
            CONSTRAINT `commitments_commitment_hash_unique` UNIQUE (`chain_id`, `contract_address`, `commitment_hash`))",
        "CREATE INDEX `commitments_chain_id_index` ON `commitments` (`chain_id`);",
        "CREATE INDEX `commitments_contract_address_index` ON `commitments` (`contract_address`);",
        "CREATE INDEX `commitments_commitment_hash_index` ON `commitments` (`commitment_hash`);",
        "CREATE INDEX `commitments_shielded_address_index` ON `commitments` (`shielded_address`);",
        "CREATE INDEX `commitments_nullifier_index` ON `commitments` (`nullifier`);",
        "CREATE INDEX `commitments_creation_transaction_hash_index` ON `commitments` (`creation_transaction_hash`);",
        "CREATE INDEX `commitments_spending_transaction_hash_index` ON `commitments` (`spending_transaction_hash`);",
        "CREATE INDEX `commitments_rollup_transaction_hash_index` ON `commitments` (`rollup_transaction_hash`);",
    ],
    field_names: &[
        CHAIN_ID_FIELD_NAME,
        CONTRACT_ADDRESS_FIELD_NAME,
        COMMITMENT_HASH_FIELD_NAME,
        ASSET_SYMBOL_FIELD_NAME,
        ASSET_DECIMALS_FIELD_NAME,
        ASSET_ADDRESS_FIELD_NAME,
        STATUS_FIELD_NAME,
        ROLLUP_FEE_AMOUNT_FIELD_NAME,
        ENCRYPTED_NOTE_FIELD_NAME,
        LEAF_INDEX_FIELD_NAME,
        AMOUNT_FIELD_NAME,
        NULLIFIER_FIELD_NAME,
        SHIELDED_ADDRESS_FIELD_NAME,
        CREATION_TRANSACTION_HASH_FIELD_NAME,
        SPENDING_TRANSACTION_HASH_FIELD_NAME,
        ROLLUP_TRANSACTION_HASH_FIELD_NAME,
    ],
};

#[derive(Clone, PartialEq, Debug)]
pub struct Commitment {
    pub chain_id: u64,
    pub contract_address: String,
    pub commitment_hash: BigInt,
    pub asset_symbol: String,
    pub asset_decimals: u32,
    pub asset_address: Option<String>,
    pub status: CommitmentStatus,
    pub rollup_fee_amount: Option<BigInt>,
    pub encrypted_note: Option<String>,
    pub leaf_index: Option<BigInt>,
    pub amount: Option<BigInt>,
    pub nullifier: Option<BigInt>,
    pub shielded_address: Option<String>,
    pub creation_transaction_hash: Option<String>,
    pub spending_transaction_hash: Option<String>,
    pub rollup_transaction_hash: Option<String>,
}

impl DocumentData for Commitment {
    fn schema() -> &'static DocumentSchema {
        &COMMITMENT_SCHEMA
    }

    fn field_value_string(&self, field: &str) -> Option<String> {
        match field {
            CHAIN_ID_FIELD_NAME => Some(self.chain_id.to_string()),
            CONTRACT_ADDRESS_FIELD_NAME => Some(self.contract_address.clone()),
            COMMITMENT_HASH_FIELD_NAME => Some(self.commitment_hash.to_string()),
            ASSET_SYMBOL_FIELD_NAME => Some(self.asset_symbol.clone()),
            ASSET_DECIMALS_FIELD_NAME => Some(self.asset_decimals.to_string()),
            ASSET_ADDRESS_FIELD_NAME => self.asset_address.clone(),
            STATUS_FIELD_NAME => Some(serde_json::to_string(&self.status).unwrap()),
            ROLLUP_FEE_AMOUNT_FIELD_NAME => self.rollup_fee_amount.as_ref().map(|r| r.to_string()),
            ENCRYPTED_NOTE_FIELD_NAME => self.encrypted_note.clone(),
            LEAF_INDEX_FIELD_NAME => self.leaf_index.as_ref().map(|r| r.to_string()),
            AMOUNT_FIELD_NAME => self.amount.as_ref().map(|r| r.to_string()),
            NULLIFIER_FIELD_NAME => self.nullifier.as_ref().map(|sn| sn.to_string()),
            SHIELDED_ADDRESS_FIELD_NAME => self.shielded_address.clone(),
            CREATION_TRANSACTION_HASH_FIELD_NAME => self.creation_transaction_hash.clone(),
            SPENDING_TRANSACTION_HASH_FIELD_NAME => self.spending_transaction_hash.clone(),
            ROLLUP_TRANSACTION_HASH_FIELD_NAME => self.rollup_transaction_hash.clone(),
            _ => None,
        }
    }

    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self, StorageError> {
        Ok(Commitment {
            chain_id: raw.required_field_integer_value(CHAIN_ID_FIELD_NAME)?,
            contract_address: raw.required_field_string_value(CONTRACT_ADDRESS_FIELD_NAME)?,
            commitment_hash: raw.required_bigint_value(COMMITMENT_HASH_FIELD_NAME)?,
            asset_symbol: raw.required_field_string_value(ASSET_SYMBOL_FIELD_NAME)?,
            asset_decimals: raw.required_field_integer_value(ASSET_DECIMALS_FIELD_NAME)?,
            asset_address: raw.field_string_value(ASSET_ADDRESS_FIELD_NAME)?,
            status: serde_json::from_str(&raw.required_field_string_value(STATUS_FIELD_NAME)?)?,
            rollup_fee_amount: raw.field_bigint_value(ROLLUP_FEE_AMOUNT_FIELD_NAME)?,
            encrypted_note: raw.field_string_value(ENCRYPTED_NOTE_FIELD_NAME)?,
            leaf_index: raw.field_bigint_value(LEAF_INDEX_FIELD_NAME)?,
            amount: raw.field_bigint_value(AMOUNT_FIELD_NAME)?,
            nullifier: raw.field_bigint_value(NULLIFIER_FIELD_NAME)?,
            shielded_address: raw.field_string_value(SHIELDED_ADDRESS_FIELD_NAME)?,
            creation_transaction_hash: raw.field_string_value(CREATION_TRANSACTION_HASH_FIELD_NAME)?,
            spending_transaction_hash: raw.field_string_value(SPENDING_TRANSACTION_HASH_FIELD_NAME)?,
            rollup_transaction_hash: raw.field_string_value(ROLLUP_TRANSACTION_HASH_FIELD_NAME)?,
        })
    }
}
