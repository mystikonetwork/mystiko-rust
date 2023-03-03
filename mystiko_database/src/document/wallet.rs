#![forbid(unsafe_code)]

use mystiko_storage::document::{DocumentData, DocumentRawData, DocumentSchema};
use std::io::Error;

pub static WALLET_SCHEMA: DocumentSchema = DocumentSchema {
    collection_name: "wallets",
    migrations: &["CREATE TABLE `wallets` (\
            `id` VARCHAR(64) NOT NULL PRIMARY KEY,\
            `created_at` INT NOT NULL,\
            `updated_at` INT NOT NULL,\
            `encrypted_master_seed` TEXT NOT NULL,\
            `hashed_password` TEXT NOT NULL,\
            `account_nonce` INT NOT NULL)"],
    field_names: &["encrypted_master_seed", "hashed_password", "account_nonce"],
};

#[derive(Clone, PartialEq, Debug)]
pub struct Wallet {
    pub encrypted_master_seed: String,
    pub hashed_password: String,
    pub account_nonce: i32,
}

impl DocumentData for Wallet {
    fn schema() -> &'static DocumentSchema {
        &WALLET_SCHEMA
    }

    fn field_value_string(&self, field: &str) -> Option<String> {
        match field {
            "encrypted_master_seed" => Some(self.encrypted_master_seed.clone()),
            "hashed_password" => Some(self.hashed_password.clone()),
            "account_nonce" => Some(self.account_nonce.to_string()),
            _ => None,
        }
    }

    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self, Error> {
        Ok(Wallet {
            encrypted_master_seed: raw.field_string_value("encrypted_master_seed")?.unwrap(),
            hashed_password: raw.field_string_value("hashed_password")?.unwrap(),
            account_nonce: raw.field_integer_value("account_nonce")?.unwrap(),
        })
    }
}
