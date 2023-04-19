#![forbid(unsafe_code)]
use anyhow::Result;
use mystiko_storage::document::{DocumentData, DocumentRawData, DocumentSchema};

pub const ENCRYPTED_ENTROPY_FIELD_NAME: &str = "encrypted_entropy";
pub const HASHED_PASSWORD_FIELD_NAME: &str = "hashed_password";
pub const ACCOUNT_NONCE_FIELD_NAME: &str = "account_nonce";

pub static WALLET_SCHEMA: DocumentSchema = DocumentSchema {
    collection_name: "wallets",
    migrations: &["CREATE TABLE `wallets` (\
            `id` VARCHAR(64) NOT NULL PRIMARY KEY,\
            `created_at` INT NOT NULL,\
            `updated_at` INT NOT NULL,\
            `encrypted_entropy` TEXT NOT NULL,\
            `hashed_password` TEXT NOT NULL,\
            `account_nonce` INT NOT NULL)"],
    field_names: &["encrypted_entropy", "hashed_password", "account_nonce"],
};

#[derive(Clone, PartialEq, Debug)]
pub struct Wallet {
    pub encrypted_entropy: String,
    pub hashed_password: String,
    pub account_nonce: u32,
}

impl DocumentData for Wallet {
    fn schema() -> &'static DocumentSchema {
        &WALLET_SCHEMA
    }

    fn field_value_string(&self, field: &str) -> Option<String> {
        match field {
            ENCRYPTED_ENTROPY_FIELD_NAME => Some(self.encrypted_entropy.clone()),
            HASHED_PASSWORD_FIELD_NAME => Some(self.hashed_password.clone()),
            ACCOUNT_NONCE_FIELD_NAME => Some(self.account_nonce.to_string()),
            _ => None,
        }
    }

    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self> {
        Ok(Wallet {
            encrypted_entropy: raw
                .field_string_value(ENCRYPTED_ENTROPY_FIELD_NAME)?
                .unwrap(),
            hashed_password: raw.field_string_value(HASHED_PASSWORD_FIELD_NAME)?.unwrap(),
            account_nonce: raw.field_integer_value(ACCOUNT_NONCE_FIELD_NAME)?.unwrap(),
        })
    }
}
