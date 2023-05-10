#![forbid(unsafe_code)]

use anyhow::Result;
use mystiko_storage::document::{DocumentData, DocumentRawData, DocumentSchema};
use mystiko_storage::error::StorageError;
use mystiko_types::{TransactionStatus, TransactionType};
use num_bigint::BigInt;

pub const CHAIN_ID_FIELD_NAME: &str = "chain_id";
pub const CONTRACT_ADDRESS_FIELD_NAME: &str = "contract_address";
pub const ASSET_SYMBOL_FIELD_NAME: &str = "asset_symbol";
pub const ASSET_DECIMALS_FIELD_NAME: &str = "asset_decimals";
pub const ASSET_ADDRESS_FIELD_NAME: &str = "asset_address";
pub const PROOF_FIELD_NAME: &str = "proof";
pub const ROOT_HASH_FIELD_NAME: &str = "root_hash";
pub const INPUT_COMMITMENTS_FIELD_NAME: &str = "input_commitments";
pub const OUTPUT_COMMITMENTS_FIELD_NAME: &str = "output_commitments";
pub const NULLIFIERS_FIELD_NAME: &str = "nullifiers";
pub const SIGNATURE_PUBLIC_KEY_FIELD_NAME: &str = "signature_public_key";
pub const SIGNATURE_PUBLIC_KEY_HASHES_FIELD_NAME: &str = "signature_public_key_hashes";
pub const AMOUNT_FIELD_NAME: &str = "amount";
pub const PUBLIC_AMOUNT_FIELD_NAME: &str = "public_amount";
pub const ROLLUP_FEE_AMOUNT_FIELD_NAME: &str = "rollup_fee_amount";
pub const GAS_RELAYER_FEE_AMOUNT_FIELD_NAME: &str = "gas_relayer_fee_amount";
pub const SHIELDED_ADDRESS_FIELD_NAME: &str = "shielded_address";
pub const PUBLIC_ADDRESS_FIELD_NAME: &str = "public_address";
pub const GAS_RELAYER_ADDRESS_FIELD_NAME: &str = "gas_relayer_address";
pub const SIGNATURE_FIELD_NAME: &str = "signature";
pub const RANDOM_AUDITING_PUBLIC_KEY_FIELD_NAME: &str = "random_auditing_public_key";
pub const ENCRYPTED_AUDITOR_NOTES_FIELD_NAME: &str = "encrypted_auditor_notes";
pub const TRANSACTION_TYPE_FIELD_NAME: &str = "transaction_type";
pub const STATUS_FIELD_NAME: &str = "status";
pub const ERROR_MESSAGE_FIELD_NAME: &str = "error_message";
pub const TRANSACTION_HASH_FIELD_NAME: &str = "transaction_hash";
pub const WALLET_ID_FIELD_NAME: &str = "wallet_id";

pub static TRANSACTION_SCHEMA: DocumentSchema = DocumentSchema {
    collection_name: "transactions",
    migrations: &[
        "CREATE TABLE `transactions` (\
            `id` VARCHAR(64) NOT NULL PRIMARY KEY,\
            `created_at` INT NOT NULL,\
            `updated_at` INT NOT NULL,\
            `chain_id` BIGINT NOT NULL,\
            `contract_address` VARCHAR(64) NOT NULL,\
            `asset_symbol` VARCHAR(16) NOT NULL,\
            `asset_decimals` INT NOT NULL,\
            `asset_address` VARCHAR(64),\
            `proof` TEXT,\
            `root_hash` VARCHAR(128) NOT NULL,\
            `input_commitments` TEXT NOT NULL,\
            `output_commitments` TEXT,\
            `nullifiers` TEXT,\
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
        "CREATE INDEX `transactions_wallet_id_index` ON `transactions` (`wallet_id`)",
    ],
    field_names: &[
        CHAIN_ID_FIELD_NAME,
        CONTRACT_ADDRESS_FIELD_NAME,
        ASSET_SYMBOL_FIELD_NAME,
        ASSET_DECIMALS_FIELD_NAME,
        ASSET_ADDRESS_FIELD_NAME,
        PROOF_FIELD_NAME,
        ROOT_HASH_FIELD_NAME,
        INPUT_COMMITMENTS_FIELD_NAME,
        OUTPUT_COMMITMENTS_FIELD_NAME,
        NULLIFIERS_FIELD_NAME,
        SIGNATURE_PUBLIC_KEY_FIELD_NAME,
        SIGNATURE_PUBLIC_KEY_HASHES_FIELD_NAME,
        AMOUNT_FIELD_NAME,
        PUBLIC_AMOUNT_FIELD_NAME,
        ROLLUP_FEE_AMOUNT_FIELD_NAME,
        GAS_RELAYER_FEE_AMOUNT_FIELD_NAME,
        SHIELDED_ADDRESS_FIELD_NAME,
        PUBLIC_ADDRESS_FIELD_NAME,
        GAS_RELAYER_ADDRESS_FIELD_NAME,
        SIGNATURE_FIELD_NAME,
        RANDOM_AUDITING_PUBLIC_KEY_FIELD_NAME,
        ENCRYPTED_AUDITOR_NOTES_FIELD_NAME,
        TRANSACTION_TYPE_FIELD_NAME,
        STATUS_FIELD_NAME,
        ERROR_MESSAGE_FIELD_NAME,
        TRANSACTION_HASH_FIELD_NAME,
        WALLET_ID_FIELD_NAME,
    ],
};

#[derive(Clone, PartialEq, Debug)]
pub struct Transaction {
    pub chain_id: u64,
    pub contract_address: String,
    pub asset_symbol: String,
    pub asset_decimals: u32,
    pub asset_address: Option<String>,
    pub proof: Option<String>,
    pub root_hash: BigInt,
    pub input_commitments: Vec<BigInt>,
    pub output_commitments: Option<Vec<BigInt>>,
    pub nullifiers: Option<Vec<BigInt>>,
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
    pub random_auditing_public_key: Option<BigInt>,
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
            CHAIN_ID_FIELD_NAME => Some(self.chain_id.to_string()),
            CONTRACT_ADDRESS_FIELD_NAME => Some(self.contract_address.clone()),
            ASSET_SYMBOL_FIELD_NAME => Some(self.asset_symbol.clone()),
            ASSET_DECIMALS_FIELD_NAME => Some(self.asset_decimals.to_string()),
            ASSET_ADDRESS_FIELD_NAME => self.asset_address.clone(),
            PROOF_FIELD_NAME => self.proof.clone(),
            ROOT_HASH_FIELD_NAME => Some(self.root_hash.to_string()),
            INPUT_COMMITMENTS_FIELD_NAME => Some(serde_json::to_string(&self.input_commitments).unwrap()),
            OUTPUT_COMMITMENTS_FIELD_NAME => Some(serde_json::to_string(&self.output_commitments).unwrap()),
            NULLIFIERS_FIELD_NAME => Some(serde_json::to_string(&self.nullifiers).unwrap()),
            SIGNATURE_PUBLIC_KEY_FIELD_NAME => self.signature_public_key.clone(),
            SIGNATURE_PUBLIC_KEY_HASHES_FIELD_NAME => {
                Some(serde_json::to_string(&self.signature_public_key_hashes.clone()).unwrap())
            }
            AMOUNT_FIELD_NAME => Some(self.amount.to_string()),
            PUBLIC_AMOUNT_FIELD_NAME => Some(self.public_amount.to_string()),
            ROLLUP_FEE_AMOUNT_FIELD_NAME => Some(self.rollup_fee_amount.to_string()),
            GAS_RELAYER_FEE_AMOUNT_FIELD_NAME => Some(self.gas_relayer_fee_amount.to_string()),
            SHIELDED_ADDRESS_FIELD_NAME => self.shielded_address.clone(),
            PUBLIC_ADDRESS_FIELD_NAME => self.public_address.clone(),
            GAS_RELAYER_ADDRESS_FIELD_NAME => self.gas_relayer_address.clone(),
            SIGNATURE_FIELD_NAME => self.signature.clone(),
            RANDOM_AUDITING_PUBLIC_KEY_FIELD_NAME => self.random_auditing_public_key.as_ref().map(|pk| pk.to_string()),
            ENCRYPTED_AUDITOR_NOTES_FIELD_NAME => Some(serde_json::to_string(&self.encrypted_auditor_notes).unwrap()),
            TRANSACTION_TYPE_FIELD_NAME => Some(serde_json::to_string(&self.transaction_type).unwrap()),
            STATUS_FIELD_NAME => Some(serde_json::to_string(&self.status).unwrap()),
            ERROR_MESSAGE_FIELD_NAME => self.error_message.clone(),
            TRANSACTION_HASH_FIELD_NAME => self.transaction_hash.clone(),
            WALLET_ID_FIELD_NAME => Some(self.wallet_id.clone()),
            _ => None,
        }
    }

    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self, StorageError> {
        Ok(Transaction {
            chain_id: raw.required_field_integer_value::<u64>(CHAIN_ID_FIELD_NAME)?,
            contract_address: raw.required_field_string_value(CONTRACT_ADDRESS_FIELD_NAME)?,
            asset_symbol: raw.required_field_string_value(ASSET_SYMBOL_FIELD_NAME)?,
            asset_decimals: raw.required_field_integer_value::<u32>(ASSET_DECIMALS_FIELD_NAME)?,
            asset_address: raw.field_string_value(ASSET_ADDRESS_FIELD_NAME)?,
            proof: raw.field_string_value(PROOF_FIELD_NAME)?,
            root_hash: raw.required_bigint_value(ROOT_HASH_FIELD_NAME)?,
            input_commitments: serde_json::from_str(&raw.required_field_string_value(INPUT_COMMITMENTS_FIELD_NAME)?)?,
            output_commitments: serde_json::from_str(&raw.required_field_string_value(OUTPUT_COMMITMENTS_FIELD_NAME)?)?,
            nullifiers: serde_json::from_str(&raw.required_field_string_value(NULLIFIERS_FIELD_NAME)?)?,
            signature_public_key: raw.field_string_value(SIGNATURE_PUBLIC_KEY_FIELD_NAME)?,
            signature_public_key_hashes: serde_json::from_str(
                &raw.required_field_string_value(SIGNATURE_PUBLIC_KEY_HASHES_FIELD_NAME)?,
            )?,
            amount: raw.required_bigint_value(AMOUNT_FIELD_NAME)?,
            public_amount: raw.required_bigint_value(PUBLIC_AMOUNT_FIELD_NAME)?,
            rollup_fee_amount: raw.required_bigint_value(ROLLUP_FEE_AMOUNT_FIELD_NAME)?,
            gas_relayer_fee_amount: raw.required_bigint_value(GAS_RELAYER_FEE_AMOUNT_FIELD_NAME)?,
            shielded_address: raw.field_string_value(SHIELDED_ADDRESS_FIELD_NAME)?,
            public_address: raw.field_string_value(PUBLIC_ADDRESS_FIELD_NAME)?,
            gas_relayer_address: raw.field_string_value(GAS_RELAYER_ADDRESS_FIELD_NAME)?,
            signature: raw.field_string_value(SIGNATURE_FIELD_NAME)?,
            random_auditing_public_key: raw.field_bigint_value(RANDOM_AUDITING_PUBLIC_KEY_FIELD_NAME)?,
            encrypted_auditor_notes: serde_json::from_str(
                &raw.required_field_string_value(ENCRYPTED_AUDITOR_NOTES_FIELD_NAME)?,
            )?,
            transaction_type: serde_json::from_str(&raw.required_field_string_value(TRANSACTION_TYPE_FIELD_NAME)?)?,
            status: serde_json::from_str(&raw.required_field_string_value(STATUS_FIELD_NAME)?)?,
            error_message: Some(raw.required_field_string_value(ERROR_MESSAGE_FIELD_NAME)?),
            transaction_hash: Some(raw.required_field_string_value(TRANSACTION_HASH_FIELD_NAME)?),
            wallet_id: raw.required_field_string_value(WALLET_ID_FIELD_NAME)?,
        })
    }
}
