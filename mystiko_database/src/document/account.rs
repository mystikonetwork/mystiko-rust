#![forbid(unsafe_code)]
use anyhow::Result;
use mystiko_storage::document::{DocumentData, DocumentRawData, DocumentSchema};
use mystiko_types::AccountStatus;

pub const NAME_FIELD_NAME: &str = "name";
pub const PUBLIC_KEY_FIELD_NAME: &str = "public_key";
pub const ENCRYPTED_SECRET_KEY_FIELD_NAME: &str = "encrypted_secret_key";
pub const STATUS_FIELD_NAME: &str = "status";
pub const SHIELDED_ADDRESS_FIELD_NAME: &str = "shielded_address";
pub const SCAN_SIZE_FIELD_NAME: &str = "scan_size";
pub const WALLET_ID_FIELD_NAME: &str = "wallet_id";

pub static ACCOUNT_SCHEMA: DocumentSchema = DocumentSchema {
    collection_name: "accounts",
    migrations: &[
        "CREATE TABLE `accounts` (\
            `id` VARCHAR(64) NOT NULL PRIMARY KEY,\
            `created_at` INT NOT NULL,\
            `updated_at` INT NOT NULL,\
            `name` VARCHAR(64) NOT NULL,\
            `shielded_address` VARCHAR(255) NOT NULL,\
            `public_key` VARCHAR(255) NOT NULL,\
            `encrypted_secret_key` TEXT NOT NULL,\
            `status` VARCHAR(32) NOT NULL,\
            `scan_size` INT NOT NULL,\
            `wallet_id` VARCHAR(64) NOT NULL)",
        "CREATE INDEX `accounts_shielded_address_index` ON `accounts` (`shielded_address`);",
        "CREATE INDEX `accounts_public_key_index` ON `accounts` (`public_key`);",
    ],
    field_names: &[
        NAME_FIELD_NAME,
        SHIELDED_ADDRESS_FIELD_NAME,
        PUBLIC_KEY_FIELD_NAME,
        ENCRYPTED_SECRET_KEY_FIELD_NAME,
        STATUS_FIELD_NAME,
        SCAN_SIZE_FIELD_NAME,
        WALLET_ID_FIELD_NAME,
    ],
};

#[derive(Clone, PartialEq, Debug)]
pub struct Account {
    pub name: String,
    pub shielded_address: String,
    pub public_key: String,
    pub encrypted_secret_key: String,
    pub status: AccountStatus,
    pub scan_size: u32,
    pub wallet_id: String,
}

impl DocumentData for Account {
    fn schema() -> &'static DocumentSchema {
        &ACCOUNT_SCHEMA
    }

    fn field_value_string(&self, field: &str) -> Option<String> {
        match field {
            NAME_FIELD_NAME => Some(self.name.clone()),
            SHIELDED_ADDRESS_FIELD_NAME => Some(self.shielded_address.clone()),
            PUBLIC_KEY_FIELD_NAME => Some(self.public_key.clone()),
            ENCRYPTED_SECRET_KEY_FIELD_NAME => Some(self.encrypted_secret_key.clone()),
            STATUS_FIELD_NAME => Some(serde_json::to_string(&self.status).unwrap()),
            SCAN_SIZE_FIELD_NAME => Some(self.scan_size.to_string()),
            WALLET_ID_FIELD_NAME => Some(self.wallet_id.to_string()),
            _ => None,
        }
    }

    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self> {
        Ok(Account {
            name: raw.field_string_value(NAME_FIELD_NAME)?.unwrap(),
            shielded_address: raw.field_string_value(SHIELDED_ADDRESS_FIELD_NAME)?.unwrap(),
            public_key: raw.field_string_value(PUBLIC_KEY_FIELD_NAME)?.unwrap(),
            encrypted_secret_key: raw.field_string_value(ENCRYPTED_SECRET_KEY_FIELD_NAME)?.unwrap(),
            status: serde_json::from_str(&raw.field_string_value(STATUS_FIELD_NAME)?.unwrap())?,
            scan_size: raw.field_integer_value::<u32>(SCAN_SIZE_FIELD_NAME)?.unwrap(),
            wallet_id: raw.field_string_value(WALLET_ID_FIELD_NAME)?.unwrap(),
        })
    }
}
