#![forbid(unsafe_code)]

use mystiko_storage::document::{DocumentData, DocumentRawData, DocumentSchema};
use num_bigint::BigInt;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

pub static COMMITMENT_SCHEMA: DocumentSchema = DocumentSchema {
    collection_name: "commitments",
    migrations: &[
        "CREATE TABLE `commitments` (\
            `id`                        VARCHAR(64) NOT NULL PRIMARY KEY,\
            `created_at`                INT          NOT NULL,\
            `updated_at`                INT          NOT NULL,\
            `chain_id`                  INT          NOT NULL,\
            `contract_address`          VARCHAR(64)  NOT NULL,\
            `commitment_hash`           VARCHAR(128) NOT NULL,\
            `asset_symbol`              VARCHAR(16) NOT NULL,\
            `asset_decimals`            INT          NOT NULL,\
            `asset_address`             VARCHAR(64),\
            `status`                    VARCHAR(32) NOT NULL,\
            `rollup_fee_amount`         VARCHAR(128),\
            `encrypted_note`            TEXT,\
            `leaf_index`                VARCHAR(64),\
            `amount`                    VARCHAR(128),\
            `serial_number`             VARCHAR(128),\
            `shielded_address`          VARCHAR(128),\
            `creation_transaction_hash` VARCHAR(128),\
            `spending_transaction_hash` VARCHAR(128),\
            `rollup_transaction_hash`   VARCHAR(128))",
        "CREATE INDEX `commitments_chain_id_index` ON `commitments` (`chain_id`);",
        "CREATE INDEX `commitments_contract_address_index` ON `commitments` (`contract_address`);",
        "CREATE INDEX `commitments_commitment_hash_index` ON `commitments` (`commitment_hash`);",
        "CREATE INDEX `commitments_shielded_address_index` ON `commitments` (`shielded_address`);",
        "CREATE INDEX `commitments_serial_number_index` ON `commitments` (`serial_number`);",
        "CREATE INDEX `commitments_creation_transaction_hash_index` ON `commitments` (`creation_transaction_hash`);",
        "CREATE INDEX `commitments_spending_transaction_hash_index` ON `commitments` (`spending_transaction_hash`);",
        "CREATE INDEX `commitments_rollup_transaction_hash_index` ON `commitments` (`rollup_transaction_hash`);",
    ],
    field_names: &[
        "chain_id",
        "contract_address",
        "commitment_hash",
        "asset_symbol",
        "asset_decimals",
        "asset_address",
        "status",
        "rollup_fee_amount",
        "encrypted_note",
        "leaf_index",
        "amount",
        "serial_number",
        "shielded_address",
        "creation_transaction_hash",
        "spending_transaction_hash",
        "rollup_transaction_hash",
    ],
};

#[derive(Clone, PartialEq, Debug)]
pub enum CommitmentStatus {
    Init,
    SrcSucceeded,
    Queued,
    Included,
    Spent,
    Failed,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Commitment {
    pub chain_id: u32,
    pub contract_address: String,
    pub commitment_hash: String,
    pub asset_symbol: String,
    pub asset_decimals: u32,
    pub asset_address: Option<String>,
    pub status: CommitmentStatus,
    pub rollup_fee_amount: Option<BigInt>,
    pub encrypted_note: Option<String>,
    pub leaf_index: Option<String>,
    pub amount: Option<BigInt>,
    pub serial_number: Option<String>,
    pub shielded_address: Option<String>,
    pub creation_transaction_hash: Option<String>,
    pub spending_transaction_hash: Option<String>,
    pub rollup_transaction_hash: Option<String>,
}

impl DocumentData for Commitment {
    fn schema() -> &'static DocumentSchema {
        &COMMITMENT_SCHEMA
    }

    fn field_value_string(&self, field: &str) -> Option<String> {
        match field {
            "chain_id" => Some(self.chain_id.to_string()),
            "contract_address" => Some(self.contract_address.clone()),
            "commitment_hash" => Some(self.commitment_hash.clone()),
            "asset_symbol" => Some(self.asset_symbol.clone()),
            "asset_decimals" => Some(self.asset_decimals.to_string()),
            "asset_address" => self.asset_address.clone(),
            "status" => Some(self.status.to_string()),
            "rollup_fee_amount" => Some(self.rollup_fee_amount.clone()?.to_string()),
            "encrypted_note" => self.encrypted_note.clone(),
            "leaf_index" => self.leaf_index.clone(),
            "amount" => Some(self.amount.clone()?.to_string()),
            "serial_number" => self.serial_number.clone(),
            "shielded_address" => self.shielded_address.clone(),
            "creation_transaction_hash" => self.creation_transaction_hash.clone(),
            "spending_transaction_hash" => self.spending_transaction_hash.clone(),
            "rollup_transaction_hash" => self.rollup_transaction_hash.clone(),
            _ => None,
        }
    }

    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self, Error> {
        Ok(Commitment {
            chain_id: raw.field_integer_value("chain_id")?.unwrap(),
            contract_address: raw.field_string_value("contract_address")?.unwrap(),
            commitment_hash: raw.field_string_value("commitment_hash")?.unwrap(),
            asset_symbol: raw.field_string_value("asset_symbol")?.unwrap(),
            asset_decimals: raw.field_integer_value("asset_decimals")?.unwrap(),
            asset_address: raw.field_string_value("asset_address")?,
            status: CommitmentStatus::from_str(&raw.field_string_value("status")?.unwrap())?,
            rollup_fee_amount: BigInt::parse_bytes(
                raw.field_string_value("rollup_fee_amount")?
                    .unwrap()
                    .as_bytes(),
                10,
            ),
            encrypted_note: raw.field_string_value("encrypted_note")?,
            leaf_index: raw.field_string_value("leaf_index")?,
            amount: BigInt::parse_bytes(raw.field_string_value("amount")?.unwrap().as_bytes(), 10),
            serial_number: raw.field_string_value("serial_number")?,
            shielded_address: raw.field_string_value("shielded_address")?,
            creation_transaction_hash: raw.field_string_value("creation_transaction_hash")?,
            spending_transaction_hash: raw.field_string_value("spending_transaction_hash")?,
            rollup_transaction_hash: raw.field_string_value("rollup_transaction_hash")?,
        })
    }
}

impl ToString for CommitmentStatus {
    fn to_string(&self) -> String {
        match self {
            CommitmentStatus::Init => String::from("Init"),
            CommitmentStatus::SrcSucceeded => String::from("SrcSucceeded"),
            CommitmentStatus::Queued => String::from("Queued"),
            CommitmentStatus::Included => String::from("Included"),
            CommitmentStatus::Spent => String::from("Spent"),
            CommitmentStatus::Failed => String::from("Failed"),
        }
    }
}

impl FromStr for CommitmentStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Init" => Ok(CommitmentStatus::Init),
            "SrcSucceeded" => Ok(CommitmentStatus::SrcSucceeded),
            "Queued" => Ok(CommitmentStatus::Queued),
            "Included" => Ok(CommitmentStatus::Included),
            "Spent" => Ok(CommitmentStatus::Spent),
            "Failed" => Ok(CommitmentStatus::Failed),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("invalid commitment status string {}", s),
            )),
        }
    }
}
