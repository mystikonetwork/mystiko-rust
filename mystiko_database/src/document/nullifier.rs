#![forbid(unsafe_code)]

use anyhow::Result;
use mystiko_storage::document::{DocumentData, DocumentRawData, DocumentSchema};
use mystiko_storage::error::StorageError;
use num_bigint::BigInt;

pub const CHAIN_ID_FIELD_NAME: &str = "chain_id";
pub const CONTRACT_ADDRESS_FIELD_NAME: &str = "contract_address";
pub const NULLIFIER_FIELD_NAME: &str = "nullifier";
pub const TRANSACTION_HASH_FIELD_NAME: &str = "transaction_hash";

pub static NULLIFIER_SCHEMA: DocumentSchema = DocumentSchema {
    collection_name: "nullifiers",
    migrations: &[
        "CREATE TABLE `nullifiers` (\
            `id` VARCHAR(64) NOT NULL PRIMARY KEY,\
            `created_at` INT NOT NULL,\
            `updated_at` INT NOT NULL,\
            `chain_id`   BIGINT NOT NULL,\
            `contract_address` VARCHAR(64) NOT NULL,\
            `nullifier`        VARCHAR(128) NOT NULL,\
            `transaction_hash` VARCHAR(128) NOT NULL,\
            CONSTRAINT `nullifiers_nullifier_unique` UNIQUE (`chain_id`, `contract_address`, `nullifier`),\
            CONSTRAINT `nullifiers_transaction_hash_unique` UNIQUE (`chain_id`, `transaction_hash`))",
        "CREATE INDEX `nullifiers_created_at_index` ON `nullifiers` (`created_at`)",
        "CREATE INDEX `nullifiers_updated_at_index` ON `nullifiers` (`updated_at`)",
        "CREATE INDEX `nullifiers_chain_id_index` ON `nullifiers` (`chain_id`)",
        "CREATE INDEX `nullifiers_contract_address_index` ON `nullifiers` (`contract_address`)",
        "CREATE INDEX `nullifiers_nullifier_index` ON `nullifiers` (`nullifier`)",
        "CREATE INDEX `nullifiers_transaction_hash_index` ON `nullifiers` (`transaction_hash`)",
    ],
    field_names: &[
        CHAIN_ID_FIELD_NAME,
        CONTRACT_ADDRESS_FIELD_NAME,
        NULLIFIER_FIELD_NAME,
        TRANSACTION_HASH_FIELD_NAME,
    ],
};

#[derive(Clone, PartialEq, Debug)]
pub struct Nullifier {
    pub chain_id: u64,
    pub contract_address: String,
    pub nullifier: BigInt,
    pub transaction_hash: String,
}

impl DocumentData for Nullifier {
    fn schema() -> &'static DocumentSchema {
        &NULLIFIER_SCHEMA
    }

    fn field_value_string(&self, field: &str) -> Option<String> {
        match field {
            CHAIN_ID_FIELD_NAME => Some(self.chain_id.to_string()),
            CONTRACT_ADDRESS_FIELD_NAME => Some(self.contract_address.clone()),
            NULLIFIER_FIELD_NAME => Some(self.nullifier.to_string()),
            TRANSACTION_HASH_FIELD_NAME => Some(self.transaction_hash.clone()),
            _ => None,
        }
    }

    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self, StorageError> {
        Ok(Nullifier {
            chain_id: raw.required_field_integer_value::<u64>(CHAIN_ID_FIELD_NAME)?,
            contract_address: raw.required_field_string_value(CONTRACT_ADDRESS_FIELD_NAME)?,
            nullifier: raw.required_bigint_value(NULLIFIER_FIELD_NAME)?,
            transaction_hash: raw.required_field_string_value(TRANSACTION_HASH_FIELD_NAME)?,
        })
    }
}
