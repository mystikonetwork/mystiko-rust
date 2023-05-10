#![forbid(unsafe_code)]

use anyhow::Result;
use mystiko_storage::document::{DocumentData, DocumentRawData, DocumentSchema};
use mystiko_storage::error::StorageError;
use mystiko_types::{BridgeType, DepositStatus};
use num_bigint::BigInt;

pub const CHAIN_ID_FIELD_NAME: &str = "chain_id";
pub const CONTRACT_ADDRESS_FIELD_NAME: &str = "contract_address";
pub const POOL_ADDRESS_FIELD_NAME: &str = "pool_address";
pub const COMMITMENT_HASH_FIELD_NAME: &str = "commitment_hash";
pub const HASH_K_FIELD_NAME: &str = "hash_k";
pub const RANDOM_S_FIELD_NAME: &str = "random_s";
pub const ENCRYPTED_NOTE_FIELD_NAME: &str = "encrypted_note";
pub const ASSET_SYMBOL_FIELD_NAME: &str = "asset_symbol";
pub const ASSET_DECIMALS_FIELD_NAME: &str = "asset_decimals";
pub const ASSET_ADDRESS_FIELD_NAME: &str = "asset_address";
pub const BRIDGE_TYPE_FIELD_NAME: &str = "bridge_type";
pub const AMOUNT_FIELD_NAME: &str = "amount";
pub const ROLLUP_FEE_AMOUNT_FIELD_NAME: &str = "rollup_fee_amount";
pub const BRIDGE_FEE_AMOUNT_FIELD_NAME: &str = "bridge_fee_amount";
pub const BRIDGE_FEE_ASSET_ADDRESS_FIELD_NAME: &str = "bridge_fee_asset_address";
pub const EXECUTOR_FEE_AMOUNT_FIELD_NAME: &str = "executor_fee_amount";
pub const EXECUTOR_FEE_ASSET_ADDRESS_FIELD_NAME: &str = "executor_fee_asset_address";
pub const SERVICE_FEE_AMOUNT_FIELD_NAME: &str = "service_fee_amount";
pub const SHIELDED_RECIPIENT_ADDRESS_FIELD_NAME: &str = "shielded_recipient_address";
pub const STATUS_FIELD_NAME: &str = "status";
pub const ERROR_MESSAGE_FIELD_NAME: &str = "error_message";
pub const WALLET_ID_FIELD_NAME: &str = "wallet_id";
pub const DST_CHAIN_ID_FIELD_NAME: &str = "dst_chain_id";
pub const DST_CHAIN_CONTRACT_ADDRESS_FIELD_NAME: &str = "dst_chain_contract_address";
pub const DST_POOL_ADDRESS_FIELD_NAME: &str = "dst_pool_address";
pub const ASSET_APPROVE_TRANSACTION_HASH_FIELD_NAME: &str = "asset_approve_transaction_hash";
pub const TRANSACTION_HASH_FIELD_NAME: &str = "transaction_hash";
pub const RELAY_TRANSACTION_HASH_FIELD_NAME: &str = "relay_transaction_hash";
pub const ROLLUP_TRANSACTION_HASH_FIELD_NAME: &str = "rollup_transaction_hash";

pub static DEPOSIT_SCHEMA: DocumentSchema = DocumentSchema {
    collection_name: "deposits",
    migrations: &[
        "CREATE TABLE `deposits` (\
            `id` VARCHAR(64) NOT NULL PRIMARY KEY,\
            `created_at` INT    NOT NULL,\
            `updated_at` INT    NOT NULL,\
            `chain_id`   BIGINT NOT NULL,\
            `contract_address` VARCHAR(64) NOT NULL,\
            `pool_address` VARCHAR(64) NOT NULL,\
            `commitment_hash` VARCHAR(128) NOT NULL,\
            `hash_k` VARCHAR(128) NOT NULL,\
            `random_s` VARCHAR(128) NOT NULL,\
            `encrypted_note` TEXT NOT NULL,\
            `asset_symbol` VARCHAR(16) NOT NULL,\
            `asset_decimals` INT NOT NULL,\
            `asset_address` VARCHAR(64),\
            `bridge_type` VARCHAR(64) NOT NULL,\
            `amount` VARCHAR(128) NOT NULL,\
            `rollup_fee_amount` VARCHAR(128) NOT NULL,\
            `bridge_fee_amount` VARCHAR(128) NOT NULL,\
            `bridge_fee_asset_address` VARCHAR(64),\
            `executor_fee_amount` VARCHAR(128) NOT NULL,\
            `executor_fee_asset_address` VARCHAR(64),\
            `service_fee_amount` VARCHAR(128) NOT NULL,\
            `shielded_recipient_address` VARCHAR(128) NOT NULL,\
            `status` VARCHAR(32) NOT NULL,\
            `error_message` TEXT,\
            `wallet_id` VARCHAR(64) NOT NULL,\
            `dst_chain_id` BIGINT NOT NULL,\
            `dst_chain_contract_address` VARCHAR(64) NOT NULL,\
            `dst_pool_address` VARCHAR(64) NOT NULL,\
            `asset_approve_transaction_hash` VARCHAR(128),\
            `transaction_hash` VARCHAR(128),\
            `relay_transaction_hash` VARCHAR(128),\
            `rollup_transaction_hash` VARCHAR(128),\
            CONSTRAINT `deposits_commitment_hash_unique` UNIQUE (`chain_id`, `contract_address`, `commitment_hash`))",
        "CREATE INDEX `deposits_created_at_index` ON `deposits` (`created_at`)",
        "CREATE INDEX `deposits_updated_at_index` ON `deposits` (`updated_at`)",
        "CREATE INDEX `deposits_chain_id_index` ON `deposits` (`chain_id`)",
        "CREATE INDEX `deposits_contract_address_index` ON `deposits` (`contract_address`)",
        "CREATE INDEX `deposits_commitment_hash_index` ON `deposits` (`commitment_hash`)",
        "CREATE INDEX `deposits_dst_chain_id_index` ON `deposits` (`dst_chain_id`)",
        "CREATE INDEX `deposits_dst_chain_contract_address_index` ON `deposits` (`dst_chain_contract_address`)",
        "CREATE INDEX `deposits_shielded_recipient_address_index` ON `deposits` (`shielded_recipient_address`)",
        "CREATE INDEX `deposits_asset_approve_transaction_hash_index` ON `deposits` (`asset_approve_transaction_hash`)",
        "CREATE INDEX `deposits_transaction_hash_index` ON `deposits` (`transaction_hash`)",
        "CREATE INDEX `deposits_relay_transaction_hash_index` ON `deposits` (`relay_transaction_hash`)",
        "CREATE INDEX `deposits_rollup_transaction_hash_index` ON `deposits` (`rollup_transaction_hash`)",
    ],
    field_names: &[
        CHAIN_ID_FIELD_NAME,
        CONTRACT_ADDRESS_FIELD_NAME,
        POOL_ADDRESS_FIELD_NAME,
        COMMITMENT_HASH_FIELD_NAME,
        HASH_K_FIELD_NAME,
        RANDOM_S_FIELD_NAME,
        ENCRYPTED_NOTE_FIELD_NAME,
        ASSET_SYMBOL_FIELD_NAME,
        ASSET_DECIMALS_FIELD_NAME,
        ASSET_ADDRESS_FIELD_NAME,
        BRIDGE_TYPE_FIELD_NAME,
        AMOUNT_FIELD_NAME,
        ROLLUP_FEE_AMOUNT_FIELD_NAME,
        BRIDGE_FEE_AMOUNT_FIELD_NAME,
        BRIDGE_FEE_ASSET_ADDRESS_FIELD_NAME,
        EXECUTOR_FEE_AMOUNT_FIELD_NAME,
        EXECUTOR_FEE_ASSET_ADDRESS_FIELD_NAME,
        SERVICE_FEE_AMOUNT_FIELD_NAME,
        SHIELDED_RECIPIENT_ADDRESS_FIELD_NAME,
        STATUS_FIELD_NAME,
        ERROR_MESSAGE_FIELD_NAME,
        WALLET_ID_FIELD_NAME,
        DST_CHAIN_ID_FIELD_NAME,
        DST_CHAIN_CONTRACT_ADDRESS_FIELD_NAME,
        DST_POOL_ADDRESS_FIELD_NAME,
        ASSET_APPROVE_TRANSACTION_HASH_FIELD_NAME,
        TRANSACTION_HASH_FIELD_NAME,
        RELAY_TRANSACTION_HASH_FIELD_NAME,
        ROLLUP_TRANSACTION_HASH_FIELD_NAME,
    ],
};

#[derive(Clone, PartialEq, Debug)]
pub struct Deposit {
    pub chain_id: u64,
    pub contract_address: String,
    pub pool_address: String,
    pub commitment_hash: BigInt,
    pub hash_k: BigInt,
    pub random_s: BigInt,
    pub encrypted_note: String,
    pub asset_symbol: String,
    pub asset_decimals: u32,
    pub asset_address: Option<String>,
    pub bridge_type: BridgeType,
    pub amount: BigInt,
    pub rollup_fee_amount: BigInt,
    pub bridge_fee_amount: BigInt,
    pub bridge_fee_asset_address: Option<String>,
    pub executor_fee_amount: BigInt,
    pub executor_fee_asset_address: Option<String>,
    pub service_fee_amount: BigInt,
    pub shielded_recipient_address: String,
    pub status: DepositStatus,
    pub error_message: Option<String>,
    pub wallet_id: String,
    pub dst_chain_id: u64,
    pub dst_chain_contract_address: String,
    pub dst_pool_address: String,
    pub asset_approve_transaction_hash: Option<String>,
    pub transaction_hash: Option<String>,
    pub relay_transaction_hash: Option<String>,
    pub rollup_transaction_hash: Option<String>,
}

impl DocumentData for Deposit {
    fn schema() -> &'static DocumentSchema {
        &DEPOSIT_SCHEMA
    }

    fn field_value_string(&self, field: &str) -> Option<String> {
        match field {
            CHAIN_ID_FIELD_NAME => Some(self.chain_id.to_string()),
            CONTRACT_ADDRESS_FIELD_NAME => Some(self.contract_address.clone()),
            POOL_ADDRESS_FIELD_NAME => Some(self.pool_address.clone()),
            COMMITMENT_HASH_FIELD_NAME => Some(self.commitment_hash.to_string()),
            HASH_K_FIELD_NAME => Some(self.hash_k.to_string()),
            RANDOM_S_FIELD_NAME => Some(self.random_s.to_string()),
            ENCRYPTED_NOTE_FIELD_NAME => Some(self.encrypted_note.clone()),
            ASSET_SYMBOL_FIELD_NAME => Some(self.asset_symbol.clone()),
            ASSET_DECIMALS_FIELD_NAME => Some(self.asset_decimals.to_string()),
            ASSET_ADDRESS_FIELD_NAME => self.asset_address.clone(),
            BRIDGE_TYPE_FIELD_NAME => Some(serde_json::to_string(&self.bridge_type).unwrap()),
            AMOUNT_FIELD_NAME => Some(self.amount.to_string()),
            ROLLUP_FEE_AMOUNT_FIELD_NAME => Some(self.rollup_fee_amount.to_string()),
            BRIDGE_FEE_AMOUNT_FIELD_NAME => Some(self.bridge_fee_amount.to_string()),
            BRIDGE_FEE_ASSET_ADDRESS_FIELD_NAME => self.bridge_fee_asset_address.clone(),
            EXECUTOR_FEE_AMOUNT_FIELD_NAME => Some(self.executor_fee_amount.to_string()),
            EXECUTOR_FEE_ASSET_ADDRESS_FIELD_NAME => self.executor_fee_asset_address.clone(),
            SERVICE_FEE_AMOUNT_FIELD_NAME => Some(self.service_fee_amount.to_string()),
            SHIELDED_RECIPIENT_ADDRESS_FIELD_NAME => Some(self.shielded_recipient_address.clone()),
            STATUS_FIELD_NAME => Some(serde_json::to_string(&self.status).unwrap()),
            ERROR_MESSAGE_FIELD_NAME => self.error_message.clone(),
            WALLET_ID_FIELD_NAME => Some(self.wallet_id.to_string()),
            DST_CHAIN_ID_FIELD_NAME => Some(self.dst_chain_id.to_string()),
            DST_CHAIN_CONTRACT_ADDRESS_FIELD_NAME => Some(self.dst_chain_contract_address.clone()),
            DST_POOL_ADDRESS_FIELD_NAME => Some(self.dst_pool_address.clone()),
            ASSET_APPROVE_TRANSACTION_HASH_FIELD_NAME => self.asset_approve_transaction_hash.clone(),
            TRANSACTION_HASH_FIELD_NAME => self.transaction_hash.clone(),
            RELAY_TRANSACTION_HASH_FIELD_NAME => self.relay_transaction_hash.clone(),
            ROLLUP_TRANSACTION_HASH_FIELD_NAME => self.rollup_transaction_hash.clone(),
            _ => None,
        }
    }

    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self, StorageError> {
        Ok(Deposit {
            chain_id: raw.required_field_integer_value::<u64>(CHAIN_ID_FIELD_NAME)?,
            contract_address: raw.required_field_string_value(CONTRACT_ADDRESS_FIELD_NAME)?,
            pool_address: raw.required_field_string_value(POOL_ADDRESS_FIELD_NAME)?,
            commitment_hash: raw.required_bigint_value(COMMITMENT_HASH_FIELD_NAME)?,
            hash_k: raw.required_bigint_value(HASH_K_FIELD_NAME)?,
            random_s: raw.required_bigint_value(RANDOM_S_FIELD_NAME)?,
            encrypted_note: raw.required_field_string_value(ENCRYPTED_NOTE_FIELD_NAME)?,
            asset_symbol: raw.required_field_string_value(ASSET_SYMBOL_FIELD_NAME)?,
            asset_decimals: raw.required_field_integer_value::<u32>(ASSET_DECIMALS_FIELD_NAME)?,
            asset_address: raw.field_string_value(ASSET_ADDRESS_FIELD_NAME)?,
            bridge_type: serde_json::from_str(&raw.required_field_string_value(BRIDGE_TYPE_FIELD_NAME)?)?,
            amount: raw.required_bigint_value(AMOUNT_FIELD_NAME)?,
            rollup_fee_amount: raw.required_bigint_value(ROLLUP_FEE_AMOUNT_FIELD_NAME)?,
            bridge_fee_amount: raw.required_bigint_value(BRIDGE_FEE_AMOUNT_FIELD_NAME)?,
            bridge_fee_asset_address: raw.field_string_value(BRIDGE_FEE_ASSET_ADDRESS_FIELD_NAME)?,
            executor_fee_amount: raw.required_bigint_value(EXECUTOR_FEE_AMOUNT_FIELD_NAME)?,
            executor_fee_asset_address: raw.field_string_value(EXECUTOR_FEE_ASSET_ADDRESS_FIELD_NAME)?,
            service_fee_amount: raw.required_bigint_value(SERVICE_FEE_AMOUNT_FIELD_NAME)?,
            shielded_recipient_address: raw.required_field_string_value(SHIELDED_RECIPIENT_ADDRESS_FIELD_NAME)?,
            status: serde_json::from_str(&raw.required_field_string_value(STATUS_FIELD_NAME)?)?,
            error_message: raw.field_string_value(ERROR_MESSAGE_FIELD_NAME)?,
            wallet_id: raw.required_field_string_value(WALLET_ID_FIELD_NAME)?,
            dst_chain_id: raw.required_field_integer_value::<u64>(DST_CHAIN_ID_FIELD_NAME)?,
            dst_chain_contract_address: raw.required_field_string_value(DST_CHAIN_CONTRACT_ADDRESS_FIELD_NAME)?,
            dst_pool_address: raw.required_field_string_value(DST_POOL_ADDRESS_FIELD_NAME)?,
            asset_approve_transaction_hash: raw.field_string_value(ASSET_APPROVE_TRANSACTION_HASH_FIELD_NAME)?,
            transaction_hash: raw.field_string_value(TRANSACTION_HASH_FIELD_NAME)?,
            relay_transaction_hash: raw.field_string_value(RELAY_TRANSACTION_HASH_FIELD_NAME)?,
            rollup_transaction_hash: raw.field_string_value(ROLLUP_TRANSACTION_HASH_FIELD_NAME)?,
        })
    }
}
