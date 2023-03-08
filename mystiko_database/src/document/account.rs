#![forbid(unsafe_code)]
use mystiko_storage::document::{DocumentData, DocumentRawData, DocumentSchema};
use std::io::{Error, ErrorKind};
use std::str::FromStr;

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
        "name",
        "shielded_address",
        "public_key",
        "encrypted_secret_key",
        "status",
        "scan_size",
        "wallet_id",
    ],
};

#[derive(Clone, PartialEq, Debug)]
pub enum AccountStatus {
    Created,
    Scanning,
    Scanned,
}

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
            "name" => Some(self.name.clone()),
            "shielded_address" => Some(self.shielded_address.clone()),
            "public_key" => Some(self.public_key.clone()),
            "encrypted_secret_key" => Some(self.encrypted_secret_key.clone()),
            "status" => Some(self.status.to_string()),
            "scan_size" => Some(self.scan_size.to_string()),
            "wallet_id" => Some(self.wallet_id.to_string()),
            _ => None,
        }
    }

    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self, Error> {
        Ok(Account {
            name: raw.field_string_value("name")?.unwrap(),
            shielded_address: raw.field_string_value("shielded_address")?.unwrap(),
            public_key: raw.field_string_value("public_key")?.unwrap(),
            encrypted_secret_key: raw.field_string_value("encrypted_secret_key")?.unwrap(),
            status: AccountStatus::from_str(&raw.field_string_value("status")?.unwrap())?,
            scan_size: raw.field_integer_value::<u32>("scan_size")?.unwrap(),
            wallet_id: raw.field_string_value("wallet_id")?.unwrap(),
        })
    }
}

impl ToString for AccountStatus {
    fn to_string(&self) -> String {
        match self {
            AccountStatus::Created => String::from("Created"),
            AccountStatus::Scanning => String::from("Scanning"),
            AccountStatus::Scanned => String::from("Scanned"),
        }
    }
}

impl FromStr for AccountStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Created" => Ok(AccountStatus::Created),
            "Scanning" => Ok(AccountStatus::Scanning),
            "Scanned" => Ok(AccountStatus::Scanned),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("invalid account status string {}", s),
            )),
        }
    }
}
