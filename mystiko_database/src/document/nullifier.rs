#![forbid(unsafe_code)]

use mystiko_storage::document::{DocumentData, DocumentRawData, DocumentSchema};
use std::io::Error;

pub static NULLIFIER_SCHEMA: DocumentSchema = DocumentSchema {
    collection_name: "nullifiers",
    migrations: &["CREATE TABLE `nullifiers` (\
            `id` VARCHAR(64) NOT NULL PRIMARY KEY,\
            `created_at` INT NOT NULL,\
            `updated_at` INT NOT NULL,\
            `chain_id` INT NOT NULL,\
            `contract_address` VARCHAR(255) NOT NULL,\
            `serial_number` VARCHAR(255) NOT NULL,\
            `transaction_hash` VARCHAR(255) NOT NULL)"],
    field_names: &[
        "chain_id",
        "contract_address",
        "serial_number",
        "transaction_hash",
    ],
};

#[derive(Clone, PartialEq, Debug)]
pub struct Nullifier {
    pub chain_id: u32,
    pub contract_address: String,
    pub serial_number: String,
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
            "serial_number" => Some(self.serial_number.clone()),
            "transaction_hash" => Some(self.transaction_hash.clone()),
            _ => None,
        }
    }

    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self, Error> {
        Ok(Nullifier {
            chain_id: raw.field_integer_value::<u32>("chain_id")?.unwrap(),
            contract_address: raw.field_string_value("contract_address")?.unwrap(),
            serial_number: raw.field_string_value("serial_number")?.unwrap(),
            transaction_hash: raw.field_string_value("transaction_hash")?.unwrap(),
        })
    }
}
