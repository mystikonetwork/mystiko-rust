#![forbid(unsafe_code)]
use anyhow::Result;
use mystiko_storage::document::{DocumentData, DocumentRawData, DocumentSchema};
use mystiko_storage::error::StorageError;
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

pub static COMMITMENT_INFO_SCHEMA: DocumentSchema = DocumentSchema {
    collection_name: "commitment_info",
    migrations: &[
        "CREATE TABLE `commitment_info` (\
            `id`                        VARCHAR(64)  NOT NULL PRIMARY KEY,\
            `created_at`                INT          NOT NULL,\
            `updated_at`                INT          NOT NULL,\
            `chain_id`                  BIGINT       NOT NULL,\
            `contract_address`          VARCHAR(64)  NOT NULL,\
            `commitment_hash`           VARCHAR(128) NOT NULL,\
            `block_number`              INT          NOT NULL,\
            `rollup_fee`                VARCHAR(128) NOT NULL,\
            `leaf_index`                INT          NOT NULL,\
            `tx_hash`                   VARCHAR(128) NOT NULL)",
        "CREATE INDEX `commitment_info_commitment_hash_index` ON `commitment_info` (`commitment_hash`);",
        "CREATE INDEX `commitment_info_leaf_index_index` ON `commitment_info` (`leaf_index`);",
    ],
    field_names: &[
        "chain_id",
        "contract_address",
        "commitment_hash",
        "block_number",
        "rollup_fee",
        "leaf_index",
        "tx_hash",
    ],
};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct CommitmentInfo {
    pub chain_id: u64,
    pub contract_address: String,
    pub commitment_hash: BigInt,
    pub block_number: u64,
    pub rollup_fee: String,
    pub leaf_index: u32,
    pub tx_hash: String,
}

impl DocumentData for CommitmentInfo {
    fn schema() -> &'static DocumentSchema {
        &COMMITMENT_INFO_SCHEMA
    }

    fn field_value_string(&self, field: &str) -> Option<String> {
        match field {
            "chain_id" => Some(self.chain_id.to_string()),
            "contract_address" => Some(self.contract_address.clone()),
            "commitment_hash" => Some(self.commitment_hash.to_string()),
            "block_number" => Some(self.block_number.to_string()),
            "rollup_fee" => Some(self.rollup_fee.to_string()),
            "leaf_index" => Some(self.leaf_index.to_string()),
            "tx_hash" => Some(self.tx_hash.clone()),
            _ => None,
        }
    }

    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self, StorageError> {
        Ok(CommitmentInfo {
            chain_id: raw.required_field_integer_value("chain_id")?,
            contract_address: raw.required_field_string_value("contract_address")?,
            block_number: raw.required_field_integer_value("block_number")?,
            commitment_hash: raw.required_bigint_value("commitment_hash")?,
            leaf_index: raw.required_field_integer_value("leaf_index")?,
            rollup_fee: raw.required_field_string_value("rollup_fee")?,
            tx_hash: raw.required_field_string_value("tx_hash")?,
        })
    }
}
