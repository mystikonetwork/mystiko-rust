#![forbid(unsafe_code)]
use anyhow::Result;
use mystiko_storage::document::{DocumentData, DocumentRawData, DocumentSchema};
use mystiko_storage::error::StorageError;
use mystiko_types::ContractType;

pub const CONTRACT_TYPE_FIELD_NAME: &str = "contract_type";
pub const CHAIN_ID_FIELD_NAME: &str = "chain_id";
pub const CONTRACT_ADDRESS_FIELD_NAME: &str = "contract_address";
pub const DISABLED_FIELD_NAME: &str = "disabled";
pub const SYNC_START_FIELD_NAME: &str = "sync_start";
pub const SYNC_SIZE_FIELD_NAME: &str = "sync_size";
pub const SYNCED_BLOCK_NUMBER_FIELD_NAME: &str = "synced_block_number";
pub const CHECKED_LEAF_INDEX_FIELD_NAME: &str = "checked_leaf_index";

pub static CONTRACT_SCHEMA: DocumentSchema = DocumentSchema {
    collection_name: "contracts",
    migrations: &[
        "CREATE TABLE `contracts` (\
            `id`                  VARCHAR(64) NOT NULL PRIMARY KEY,\
            `created_at`          INT          NOT NULL,\
            `updated_at`          INT          NOT NULL,\
            `contract_type`       VARCHAR(64) NOT NULL,\
            `chain_id`            BIGINT      NOT NULL,\
            `contract_address`    VARCHAR(64)  NOT NULL,\
            `disabled`            TINYINT      NOT NULL,\
            `sync_start`          BIGINT NOT NULL,\
            `sync_size`           BIGINT NOT NULL,\
            `synced_block_number` BIGINT NOT NULL,\
            `checked_leaf_index`  BIGINT,\
            CONSTRAINT `contracts_address_unique` UNIQUE (`chain_id`, `contract_address`))",
        "CREATE INDEX `contracts_chain_id_index` ON `contracts` (`chain_id`)",
        "CREATE INDEX `contracts_contract_address_index` ON `contracts` (`contract_address`)",
    ],
    field_names: &[
        CONTRACT_TYPE_FIELD_NAME,
        CHAIN_ID_FIELD_NAME,
        CONTRACT_ADDRESS_FIELD_NAME,
        DISABLED_FIELD_NAME,
        SYNC_START_FIELD_NAME,
        SYNC_SIZE_FIELD_NAME,
        SYNCED_BLOCK_NUMBER_FIELD_NAME,
        CHECKED_LEAF_INDEX_FIELD_NAME,
    ],
};

#[derive(Clone, PartialEq, Debug)]
pub struct Contract {
    pub contract_type: ContractType,
    pub chain_id: u64,
    pub contract_address: String,
    pub disabled: bool,
    pub sync_start: u64,
    pub sync_size: u64,
    pub synced_block_number: u64,
    pub checked_leaf_index: Option<u64>,
}

impl DocumentData for Contract {
    fn schema() -> &'static DocumentSchema {
        &CONTRACT_SCHEMA
    }

    fn field_value_string(&self, field: &str) -> Option<String> {
        match field {
            CONTRACT_TYPE_FIELD_NAME => Some(serde_json::to_string(&self.contract_type).unwrap()),
            CHAIN_ID_FIELD_NAME => Some(self.chain_id.to_string()),
            CONTRACT_ADDRESS_FIELD_NAME => Some(self.contract_address.clone()),
            DISABLED_FIELD_NAME => Some(if self.disabled {
                String::from("1")
            } else {
                String::from("0")
            }),
            SYNC_START_FIELD_NAME => Some(self.sync_start.to_string()),
            SYNC_SIZE_FIELD_NAME => Some(self.sync_size.to_string()),
            SYNCED_BLOCK_NUMBER_FIELD_NAME => Some(self.synced_block_number.to_string()),
            CHECKED_LEAF_INDEX_FIELD_NAME => Some(self.checked_leaf_index?.to_string()),
            _ => None,
        }
    }

    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self, StorageError> {
        Ok(Contract {
            contract_type: serde_json::from_str(&raw.required_field_string_value(CONTRACT_TYPE_FIELD_NAME)?)?,
            chain_id: raw.required_field_integer_value(CHAIN_ID_FIELD_NAME)?,
            contract_address: raw.required_field_string_value(CONTRACT_ADDRESS_FIELD_NAME)?,
            disabled: raw.required_field_integer_value::<u8>(DISABLED_FIELD_NAME)? != 0,
            sync_start: raw.required_field_integer_value(SYNC_START_FIELD_NAME)?,
            sync_size: raw.required_field_integer_value(SYNC_SIZE_FIELD_NAME)?,
            synced_block_number: raw.required_field_integer_value(SYNCED_BLOCK_NUMBER_FIELD_NAME)?,
            checked_leaf_index: raw.field_integer_value(CHECKED_LEAF_INDEX_FIELD_NAME)?,
        })
    }
}
