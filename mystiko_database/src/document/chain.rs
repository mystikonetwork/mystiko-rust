#![forbid(unsafe_code)]
use anyhow::Result;
use mystiko_storage::document::{DocumentData, DocumentRawData, DocumentSchema};
use serde::{Deserialize, Serialize};

pub const CHAIN_ID_FIELD_NAME: &str = "chain_id";
pub const NAME_FIELD_NAME: &str = "name";
pub const NAME_OVERRIDE_FIELD_NAME: &str = "name_override";
pub const PROVIDERS_FIELD_NAME: &str = "providers";
pub const PROVIDER_OVERRIDE_FIELD_NAME: &str = "provider_override";
pub const SYNCED_BLOCK_NUMBER_FIELD_NAME: &str = "synced_block_number";

pub static CHAIN_SCHEMA: DocumentSchema = DocumentSchema {
    collection_name: "chains",
    migrations: &[
        "CREATE TABLE `chains` (\
            `id`                  VARCHAR(64) NOT NULL PRIMARY KEY,\
            `created_at`          INT          NOT NULL,\
            `updated_at`          INT          NOT NULL,\
            `chain_id`            BIGINT       NOT NULL,\
            `name`                VARCHAR(64) NOT NULL,\
            `name_override`       TINYINT,\
            `providers`           TEXT         NOT NULL,\
            `provider_override`   TINYINT,\
            `synced_block_number` BIGINT       NOT NULL,\
            CONSTRAINT `chains_chain_id_unique` UNIQUE (`chain_id`))",
        "CREATE INDEX chains_chain_id_index ON chains (chain_id)",
    ],
    field_names: &[
        CHAIN_ID_FIELD_NAME,
        NAME_FIELD_NAME,
        NAME_OVERRIDE_FIELD_NAME,
        PROVIDERS_FIELD_NAME,
        PROVIDER_OVERRIDE_FIELD_NAME,
        SYNCED_BLOCK_NUMBER_FIELD_NAME,
    ],
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Provider {
    pub url: String,
    pub timeout_ms: u32,
    pub max_try_count: u32,
    pub quorum_weight: u32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Chain {
    pub chain_id: u64,
    pub name: String,
    pub name_override: bool,
    pub providers: Vec<Provider>,
    pub provider_override: bool,
    pub synced_block_number: u64,
}

impl DocumentData for Chain {
    fn schema() -> &'static DocumentSchema {
        &CHAIN_SCHEMA
    }

    fn field_value_string(&self, field: &str) -> Option<String> {
        match field {
            CHAIN_ID_FIELD_NAME => Some(self.chain_id.to_string()),
            NAME_FIELD_NAME => Some(self.name.clone()),
            NAME_OVERRIDE_FIELD_NAME => Some(if self.name_override {
                String::from("1")
            } else {
                String::from("0")
            }),
            PROVIDERS_FIELD_NAME => Some(serde_json::to_string(&self.providers).unwrap()),
            PROVIDER_OVERRIDE_FIELD_NAME => Some(if self.provider_override {
                String::from("1")
            } else {
                String::from("0")
            }),
            SYNCED_BLOCK_NUMBER_FIELD_NAME => Some(self.synced_block_number.to_string()),
            _ => None,
        }
    }

    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self> {
        Ok(Chain {
            chain_id: raw.field_integer_value(CHAIN_ID_FIELD_NAME)?.unwrap(),
            name: raw.field_string_value(NAME_FIELD_NAME)?.unwrap(),
            name_override: raw
                .field_integer_value::<u8>(NAME_OVERRIDE_FIELD_NAME)?
                .unwrap()
                != 0,
            providers: serde_json::from_str(
                &raw.field_string_value(PROVIDERS_FIELD_NAME)?.unwrap(),
            )?,
            provider_override: raw
                .field_integer_value::<u8>(PROVIDER_OVERRIDE_FIELD_NAME)?
                .unwrap()
                != 0,
            synced_block_number: raw
                .field_integer_value(SYNCED_BLOCK_NUMBER_FIELD_NAME)?
                .unwrap(),
        })
    }
}
