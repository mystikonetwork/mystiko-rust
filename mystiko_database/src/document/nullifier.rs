#![forbid(unsafe_code)]

use anyhow::Result;
use mystiko_storage::document::{DocumentData, DocumentRawData, DocumentSchema};
use num_bigint::BigInt;
use std::str::FromStr;

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
    field_names: &["chain_id", "contract_address", "nullifier", "transaction_hash"],
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
            "chain_id" => Some(self.chain_id.to_string()),
            "contract_address" => Some(self.contract_address.clone()),
            "nullifier" => Some(self.nullifier.to_string()),
            "transaction_hash" => Some(self.transaction_hash.clone()),
            _ => None,
        }
    }

    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self> {
        Ok(Nullifier {
            chain_id: raw.field_integer_value::<u64>("chain_id")?.unwrap(),
            contract_address: raw.field_string_value("contract_address")?.unwrap(),
            nullifier: BigInt::from_str(&raw.field_string_value("nullifier")?.unwrap())?,
            transaction_hash: raw.field_string_value("transaction_hash")?.unwrap(),
        })
    }
}
