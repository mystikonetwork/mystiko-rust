#![forbid(unsafe_code)]

use mystiko_storage::document::{DocumentData, DocumentRawData, DocumentSchema};
use std::io::{Error, ErrorKind};
use std::str::FromStr;

pub static CONTRACT_SCHEMA: DocumentSchema = DocumentSchema {
    collection_name: "contracts",
    migrations: &[
        "CREATE TABLE `contracts` (\
            `id`                  VARCHAR(64) NOT NULL PRIMARY KEY,\
            `created_at`          INT          NOT NULL,\
            `updated_at`          INT          NOT NULL,\
            `contract_type`       VARCHAR(64) NOT NULL,\
            `chain_id`            INT          NOT NULL,\
            `contract_address`    VARCHAR(64)  NOT NULL,\
            `disabled`            INT          NOT NULL,\
            `sync_start`          INT NOT NULL,\
            `sync_size`           INT NOT NULL,\
            `synced_block_number` INT NOT NULL,\
            `checked_leaf_index`  INT)",
        "CREATE INDEX `contracts_chain_id_index` ON `contracts` (`chain_id`)",
        "CREATE INDEX `contracts_contract_address_index` ON `contracts` (`contract_address`)",
    ],
    field_names: &[
        "contract_type",
        "chain_id",
        "contract_address",
        "disabled",
        "sync_start",
        "sync_size",
        "synced_block_number",
        "checked_leaf_index",
    ],
};

#[derive(Clone, PartialEq, Debug)]
pub enum ContractType {
    Deposit,
    Pool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Contract {
    pub contract_type: ContractType,
    pub chain_id: u32,
    pub contract_address: String,
    pub disabled: u32,
    pub sync_start: u32,
    pub sync_size: u32,
    pub synced_block_number: u32,
    pub checked_leaf_index: Option<u32>,
}

impl DocumentData for Contract {
    fn schema() -> &'static DocumentSchema {
        &CONTRACT_SCHEMA
    }

    fn field_value_string(&self, field: &str) -> Option<String> {
        match field {
            "contract_type" => Some(self.contract_type.to_string()),
            "chain_id" => Some(self.chain_id.to_string()),
            "contract_address" => Some(self.contract_address.clone()),
            "disabled" => Some(self.disabled.to_string()),
            "sync_start" => Some(self.sync_start.to_string()),
            "sync_size" => Some(self.sync_size.to_string()),
            "synced_block_number" => Some(self.synced_block_number.to_string()),
            "checked_leaf_index" => Some(self.checked_leaf_index?.to_string()),
            _ => None,
        }
    }

    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self, Error> {
        Ok(Contract {
            contract_type: ContractType::from_str(
                &raw.field_string_value("contract_type")?.unwrap(),
            )?,
            chain_id: raw.field_integer_value("chain_id")?.unwrap(),
            contract_address: raw.field_string_value("contract_address")?.unwrap(),
            disabled: raw.field_integer_value("disabled")?.unwrap(),
            sync_start: raw.field_integer_value("sync_start")?.unwrap(),
            sync_size: raw.field_integer_value("sync_size")?.unwrap(),
            synced_block_number: raw.field_integer_value("synced_block_number")?.unwrap(),
            checked_leaf_index: raw.field_integer_value("checked_leaf_index")?,
        })
    }
}

impl ToString for ContractType {
    fn to_string(&self) -> String {
        match self {
            ContractType::Deposit => String::from("Deposit"),
            ContractType::Pool => String::from("Pool"),
        }
    }
}

impl FromStr for ContractType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Deposit" => Ok(ContractType::Deposit),
            "Pool" => Ok(ContractType::Pool),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("invalid contract type string {}", s),
            )),
        }
    }
}
