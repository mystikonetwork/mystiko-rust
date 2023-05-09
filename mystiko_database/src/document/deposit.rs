#![forbid(unsafe_code)]

use anyhow::Result;
use mystiko_storage::document::{DocumentData, DocumentRawData, DocumentSchema};
use mystiko_types::{BridgeType, DepositStatus};
use num_bigint::BigInt;
use std::str::FromStr;

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
        "chain_id",
        "contract_address",
        "pool_address",
        "commitment_hash",
        "hash_k",
        "random_s",
        "encrypted_note",
        "asset_symbol",
        "asset_decimals",
        "asset_address",
        "bridge_type",
        "amount",
        "rollup_fee_amount",
        "bridge_fee_amount",
        "bridge_fee_asset_address",
        "executor_fee_amount",
        "executor_fee_asset_address",
        "service_fee_amount",
        "shielded_recipient_address",
        "status",
        "error_message",
        "wallet_id",
        "dst_chain_id",
        "dst_chain_contract_address",
        "dst_pool_address",
        "asset_approve_transaction_hash",
        "transaction_hash",
        "relay_transaction_hash",
        "rollup_transaction_hash",
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
            "chain_id" => Some(self.chain_id.to_string()),
            "contract_address" => Some(self.contract_address.clone()),
            "pool_address" => Some(self.pool_address.clone()),
            "commitment_hash" => Some(self.commitment_hash.to_string()),
            "hash_k" => Some(self.hash_k.to_string()),
            "random_s" => Some(self.random_s.to_string()),
            "encrypted_note" => Some(self.encrypted_note.clone()),
            "asset_symbol" => Some(self.asset_symbol.clone()),
            "asset_decimals" => Some(self.asset_decimals.to_string()),
            "asset_address" => self.asset_address.clone(),
            "bridge_type" => Some(serde_json::to_string(&self.bridge_type).unwrap()),
            "amount" => Some(self.amount.to_string()),
            "rollup_fee_amount" => Some(self.rollup_fee_amount.to_string()),
            "bridge_fee_amount" => Some(self.bridge_fee_amount.to_string()),
            "bridge_fee_asset_address" => self.bridge_fee_asset_address.clone(),
            "executor_fee_amount" => Some(self.executor_fee_amount.to_string()),
            "executor_fee_asset_address" => self.executor_fee_asset_address.clone(),
            "service_fee_amount" => Some(self.service_fee_amount.to_string()),
            "shielded_recipient_address" => Some(self.shielded_recipient_address.clone()),
            "status" => Some(serde_json::to_string(&self.status).unwrap()),
            "error_message" => self.error_message.clone(),
            "wallet_id" => Some(self.wallet_id.to_string()),
            "dst_chain_id" => Some(self.dst_chain_id.to_string()),
            "dst_chain_contract_address" => Some(self.dst_chain_contract_address.clone()),
            "dst_pool_address" => Some(self.dst_pool_address.clone()),
            "asset_approve_transaction_hash" => self.asset_approve_transaction_hash.clone(),
            "transaction_hash" => self.transaction_hash.clone(),
            "relay_transaction_hash" => self.relay_transaction_hash.clone(),
            "rollup_transaction_hash" => self.rollup_transaction_hash.clone(),
            _ => None,
        }
    }

    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self> {
        Ok(Deposit {
            chain_id: raw.field_integer_value::<u64>("chain_id")?.unwrap(),
            contract_address: raw.field_string_value("contract_address")?.unwrap(),
            pool_address: raw.field_string_value("pool_address")?.unwrap(),
            commitment_hash: BigInt::from_str(&raw.field_string_value("commitment_hash")?.unwrap())?,
            hash_k: BigInt::from_str(&raw.field_string_value("hash_k")?.unwrap())?,
            random_s: BigInt::from_str(&raw.field_string_value("random_s")?.unwrap())?,
            encrypted_note: raw.field_string_value("encrypted_note")?.unwrap(),
            asset_symbol: raw.field_string_value("asset_symbol")?.unwrap(),
            asset_decimals: raw.field_integer_value::<u32>("asset_decimals")?.unwrap(),
            asset_address: raw.field_string_value("asset_address")?,
            bridge_type: serde_json::from_str(&raw.field_string_value("bridge_type")?.unwrap())?,
            amount: BigInt::from_str(&raw.field_string_value("amount")?.unwrap()).unwrap(),
            rollup_fee_amount: BigInt::from_str(&raw.field_string_value("rollup_fee_amount")?.unwrap())?,
            bridge_fee_amount: BigInt::from_str(&raw.field_string_value("bridge_fee_amount")?.unwrap())?,
            bridge_fee_asset_address: raw.field_string_value("bridge_fee_asset_address")?,
            executor_fee_amount: BigInt::from_str(&raw.field_string_value("executor_fee_amount")?.unwrap())?,
            executor_fee_asset_address: raw.field_string_value("executor_fee_asset_address")?,
            service_fee_amount: BigInt::from_str(&raw.field_string_value("service_fee_amount")?.unwrap())?,
            shielded_recipient_address: raw.field_string_value("shielded_recipient_address")?.unwrap(),
            status: serde_json::from_str(&raw.field_string_value("status")?.unwrap())?,
            error_message: raw.field_string_value("error_message")?,
            wallet_id: raw.field_string_value("wallet_id")?.unwrap(),
            dst_chain_id: raw.field_integer_value::<u64>("dst_chain_id")?.unwrap(),
            dst_chain_contract_address: raw.field_string_value("dst_chain_contract_address")?.unwrap(),
            dst_pool_address: raw.field_string_value("dst_pool_address")?.unwrap(),
            asset_approve_transaction_hash: raw.field_string_value("asset_approve_transaction_hash")?,
            transaction_hash: raw.field_string_value("transaction_hash")?,
            relay_transaction_hash: raw.field_string_value("relay_transaction_hash")?,
            rollup_transaction_hash: raw.field_string_value("rollup_transaction_hash")?,
        })
    }
}
