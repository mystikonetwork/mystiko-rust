#![forbid(unsafe_code)]
use anyhow::Result;
use mystiko_storage::document::{DocumentData, DocumentRawData, DocumentSchema};

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
            `event_filter_size`   BIGINT       NOT NULL,\
            `synced_block_number` BIGINT       NOT NULL)",
        "CREATE INDEX chains_chain_id_index ON chains (chain_id)",
    ],
    field_names: &[
        "chain_id",
        "name",
        "name_override",
        "providers",
        "provider_override",
        "event_filter_size",
        "synced_block_number",
    ],
};

#[derive(Clone, PartialEq, Debug)]
pub struct Chain {
    pub chain_id: u64,
    pub name: String,
    pub name_override: bool,
    pub providers: Vec<String>,
    pub provider_override: bool,
    pub event_filter_size: u64,
    pub synced_block_number: u64,
}

impl DocumentData for Chain {
    fn schema() -> &'static DocumentSchema {
        &CHAIN_SCHEMA
    }

    fn field_value_string(&self, field: &str) -> Option<String> {
        match field {
            "chain_id" => Some(self.chain_id.to_string()),
            "name" => Some(self.name.clone()),
            "name_override" => Some(if self.name_override {
                String::from("1")
            } else {
                String::from("0")
            }),
            "providers" => Some(serde_json::to_string(&self.providers.clone()).unwrap()),
            "provider_override" => Some(if self.provider_override {
                String::from("1")
            } else {
                String::from("0")
            }),
            "event_filter_size" => Some(self.event_filter_size.to_string()),
            "synced_block_number" => Some(self.synced_block_number.to_string()),
            _ => None,
        }
    }

    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self> {
        Ok(Chain {
            chain_id: raw.field_integer_value("chain_id")?.unwrap(),
            name: raw.field_string_value("name")?.unwrap(),
            name_override: raw.field_integer_value::<u8>("name_override")?.unwrap() != 0,
            providers: serde_json::from_str(&raw.field_string_value("providers")?.unwrap())?,
            provider_override: raw.field_integer_value::<u8>("provider_override")?.unwrap() != 0,
            event_filter_size: raw.field_integer_value("event_filter_size")?.unwrap(),
            synced_block_number: raw.field_integer_value("synced_block_number")?.unwrap(),
        })
    }
}
