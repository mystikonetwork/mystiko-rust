#![forbid(unsafe_code)]

use mystiko_storage::document::{DocumentData, DocumentRawData, DocumentSchema};
use num_bigint::BigInt;
use serde_json;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

pub static TRANSACTION_SCHEMA: DocumentSchema = DocumentSchema {
    collection_name: "transactions",
    migrations: &["CREATE TABLE `transactions` (\
            `id` VARCHAR(64) NOT NULL PRIMARY KEY,\
            `created_at` INT NOT NULL,\
            `updated_at` INT NOT NULL,\
            `chain_id` INT NOT NULL,\
            `contract_address` VARCHAR(64) NOT NULL,\
            `asset_symbol` VARCHAR(16) NOT NULL,\
            `asset_decimals` INT NOT NULL,\
            `asset_address` VARCHAR(64),\
            `proof` TEXT,\
            `root_hash` VARCHAR(128) NOT NULL,\
            `input_commitments` TEXT NOT NULL,\
            `output_commitments` TEXT,\
            `serial_numbers` TEXT,\
            `signature_public_key` VARCHAR(255),\
            `signature_public_key_hashes` TEXT,\
            `amount` VARCHAR(128) NOT NULL,\
            `public_amount` VARCHAR(128) NOT NULL,\
            `rollup_fee_amount` VARCHAR(128) NOT NULL,\
            `gas_relayer_fee_amount` VARCHAR(128) NOT NULL,\
            `shielded_address` VARCHAR(128),\
            `public_address` VARCHAR(64),\
            `gas_relayer_address` VARCHAR(64),\
            `signature` VARCHAR(255),\
            `random_auditing_public_key` VARCHAR(255),\
            `encrypted_auditor_notes` TEXT,\
            `transaction_type` VARCHAR(32) NOT NULL,\
            `status` VARCHAR(32) NOT NULL,\
            `error_message` TEXT,\
            `transaction_hash` VARCHAR(128),\
            `wallet_id` VARCHAR(64) NOT NULL)",
            "CREATE INDEX `transactions_created_at_index` ON `transactions` (`created_at`)",
            "CREATE INDEX `transactions_updated_at_index` ON `transactions` (`updated_at`)",
            "CREATE INDEX `transactions_chain_id_index` ON `transactions` (`chain_id`)",
            "CREATE INDEX `transactions_contract_address_index` ON `transactions` (`contract_address`)",
            "CREATE INDEX `transactions_signature_public_key_index` ON `transactions` (`signature_public_key`)",
            "CREATE INDEX `transactions_root_hash_index` ON `transactions` (`root_hash`)",
            "CREATE INDEX `transactions_shielded_address_index` ON `transactions` (`shielded_address`)",
            "CREATE INDEX `transactions_public_address_index` ON `transactions` (`public_address`)",
            "CREATE INDEX `transactions_transaction_hash_index` ON `transactions` (`transaction_hash`)",
            "CREATE INDEX `transactions_transaction_type_index` ON `transactions` (`transaction_type`)",
            "CREATE INDEX `transactions_wallet_id_index` ON `transactions` (`wallet_id`)"],
    field_names: &[
        "chain_id",
        "contract_address",
        "asset_symbol",
        "asset_decimals",
        "asset_address",
        "proof",
        "root_hash",
        "input_commitments",
        "output_commitments",
        "serial_numbers",
        "signature_public_key",
        "signature_public_key_hashes",
        "amount",
        "public_amount",
        "rollup_fee_amount",
        "gas_relayer_fee_amount",
        "shielded_address",
        "public_address",
        "gas_relayer_address",
        "signature",
        "random_auditing_public_key",
        "encrypted_auditor_notes",
        "transaction_type",
        "status",
        "error_message",
        "transaction_hash",
        "wallet_id",
    ],
};

#[derive(Clone, PartialEq, Debug)]
pub enum TransactionStatus {
    Init,
    ProofGenerating,
    ProofGenerated,
    Pending,
    Succeeded,
    Failed,
}

impl ToString for TransactionStatus {
    fn to_string(&self) -> String {
        match self {
            TransactionStatus::Init => String::from("Init"),
            TransactionStatus::ProofGenerating => String::from("ProofGenerating"),
            TransactionStatus::ProofGenerated => String::from("ProofGenerated"),
            TransactionStatus::Pending => String::from("Pending"),
            TransactionStatus::Succeeded => String::from("Succeeded"),
            TransactionStatus::Failed => String::from("Failed"),
        }
    }
}

impl FromStr for TransactionStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Init" => Ok(TransactionStatus::Init),
            "ProofGenerating" => Ok(TransactionStatus::ProofGenerating),
            "ProofGenerated" => Ok(TransactionStatus::ProofGenerated),
            "Pending" => Ok(TransactionStatus::Pending),
            "Succeeded" => Ok(TransactionStatus::Succeeded),
            "Failed" => Ok(TransactionStatus::Failed),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("invalid transaction status string {}", s),
            )),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum TransactionType {
    Transfer,
    Withdraw,
}

impl ToString for TransactionType {
    fn to_string(&self) -> String {
        match self {
            TransactionType::Transfer => String::from("Transfer"),
            TransactionType::Withdraw => String::from("Withdraw"),
        }
    }
}

impl FromStr for TransactionType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Transfer" => Ok(TransactionType::Transfer),
            "Withdraw" => Ok(TransactionType::Withdraw),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("invalid transaction type string {}", s),
            )),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Transaction {
    pub chain_id: u32,
    pub contract_address: String,
    pub asset_symbol: String,
    pub asset_decimals: u32,
    pub asset_address: Option<String>,
    pub proof: Option<String>,
    pub root_hash: String,
    pub input_commitments: Vec<String>,
    pub output_commitments: Option<Vec<String>>,
    pub serial_numbers: Option<Vec<String>>,
    pub signature_public_key: Option<String>,
    pub signature_public_key_hashes: Option<Vec<String>>,
    pub amount: BigInt,
    pub public_amount: BigInt,
    pub rollup_fee_amount: BigInt,
    pub gas_relayer_fee_amount: BigInt,
    pub shielded_address: Option<String>,
    pub public_address: Option<String>,
    pub gas_relayer_address: Option<String>,
    pub signature: Option<String>,
    pub random_auditing_public_key: Option<String>,
    pub encrypted_auditor_notes: Option<Vec<String>>,
    pub transaction_type: TransactionType,
    pub status: TransactionStatus,
    pub error_message: Option<String>,
    pub transaction_hash: Option<String>,
    pub wallet_id: String,
}

impl DocumentData for Transaction {
    fn schema() -> &'static DocumentSchema {
        &TRANSACTION_SCHEMA
    }

    fn field_value_string(&self, field: &str) -> Option<String> {
        match field {
            "chain_id" => Some(self.chain_id.to_string()),
            "contract_address" => Some(self.contract_address.clone()),
            "asset_symbol" => Some(self.asset_symbol.clone()),
            "asset_decimals" => Some(self.asset_decimals.to_string()),
            "asset_address" => self.asset_address.clone(),
            "proof" => self.proof.clone(),
            "root_hash" => Some(self.root_hash.clone()),
            "input_commitments" => {
                Some(serde_json::to_string(&self.input_commitments.clone()).unwrap())
            }
            "output_commitments" => {
                Some(serde_json::to_string(&self.output_commitments.clone()).unwrap())
            }
            "serial_numbers" => Some(serde_json::to_string(&self.serial_numbers.clone()).unwrap()),
            "signature_public_key" => self.signature_public_key.clone(),
            "signature_public_key_hashes" => {
                Some(serde_json::to_string(&self.signature_public_key_hashes.clone()).unwrap())
            }
            "amount" => Some(self.amount.to_string()),
            "public_amount" => Some(self.public_amount.to_string()),
            "rollup_fee_amount" => Some(self.rollup_fee_amount.to_string()),
            "gas_relayer_fee_amount" => Some(self.gas_relayer_fee_amount.to_string()),
            "shielded_address" => self.shielded_address.clone(),
            "public_address" => self.public_address.clone(),
            "gas_relayer_address" => self.gas_relayer_address.clone(),
            "signature" => self.signature.clone(),
            "random_auditing_public_key" => self.random_auditing_public_key.clone(),
            "encrypted_auditor_notes" => {
                Some(serde_json::to_string(&self.encrypted_auditor_notes.clone()).unwrap())
            }
            "transaction_type" => Some(self.transaction_type.to_string()),
            "status" => Some(self.status.to_string()),
            "error_message" => self.error_message.clone(),
            "transaction_hash" => self.transaction_hash.clone(),
            "wallet_id" => Some(self.wallet_id.clone()),
            _ => None,
        }
    }

    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self, Error> {
        Ok(Transaction {
            chain_id: raw.field_integer_value::<u32>("chain_id")?.unwrap(),
            contract_address: raw.field_string_value("contract_address")?.unwrap(),
            asset_symbol: raw.field_string_value("asset_symbol")?.unwrap(),
            asset_decimals: raw.field_integer_value::<u32>("asset_decimals")?.unwrap(),
            asset_address: raw.field_string_value("asset_address")?,
            proof: raw.field_string_value("proof")?,
            root_hash: raw.field_string_value("root_hash")?.unwrap(),
            input_commitments: serde_json::from_str(
                &raw.field_string_value("input_commitments")?.unwrap(),
            )?,
            output_commitments: serde_json::from_str(
                &raw.field_string_value("output_commitments")?.unwrap(),
            )?,
            serial_numbers: serde_json::from_str(
                &raw.field_string_value("serial_numbers")?.unwrap(),
            )?,
            signature_public_key: raw.field_string_value("signature_public_key")?,
            signature_public_key_hashes: serde_json::from_str(
                &raw.field_string_value("signature_public_key_hashes")?
                    .unwrap(),
            )?,
            amount: BigInt::parse_bytes(raw.field_string_value("amount")?.unwrap().as_bytes(), 10)
                .unwrap(),
            public_amount: BigInt::parse_bytes(
                raw.field_string_value("public_amount")?.unwrap().as_bytes(),
                10,
            )
            .unwrap(),
            rollup_fee_amount: BigInt::parse_bytes(
                raw.field_string_value("rollup_fee_amount")?
                    .unwrap()
                    .as_bytes(),
                10,
            )
            .unwrap(),
            gas_relayer_fee_amount: BigInt::parse_bytes(
                raw.field_string_value("gas_relayer_fee_amount")?
                    .unwrap()
                    .as_bytes(),
                10,
            )
            .unwrap(),
            shielded_address: raw.field_string_value("shielded_address")?,
            public_address: raw.field_string_value("public_address")?,
            gas_relayer_address: raw.field_string_value("gas_relayer_address")?,
            signature: raw.field_string_value("signature")?,
            random_auditing_public_key: raw.field_string_value("random_auditing_public_key")?,
            encrypted_auditor_notes: serde_json::from_str(
                &raw.field_string_value("encrypted_auditor_notes")?.unwrap(),
            )?,
            transaction_type: TransactionType::from_str(
                &raw.field_string_value("transaction_type")?.unwrap(),
            )?,
            status: TransactionStatus::from_str(&raw.field_string_value("status")?.unwrap())?,
            error_message: Some(raw.field_string_value("error_message")?.unwrap()),
            transaction_hash: Some(raw.field_string_value("transaction_hash")?.unwrap()),
            wallet_id: raw.field_string_value("wallet_id")?.unwrap(),
        })
    }
}
