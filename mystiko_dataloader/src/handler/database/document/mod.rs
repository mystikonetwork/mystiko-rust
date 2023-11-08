mod commitment;
mod contract;
mod nullifier;

pub use commitment::*;
pub use contract::*;
pub use nullifier::*;

use anyhow::Result;
use mystiko_config::MystikoConfig;
use mystiko_protos::data::v1::{Commitment as ProtoCommitment, Nullifier as ProtoNullifier};
use mystiko_storage::DocumentData;
use num_bigint::BigUint;
use std::sync::Arc;

pub trait DatabaseContract: DocumentData {
    fn column_chain_id() -> String;

    fn column_contract_address() -> String;

    fn column_loaded_block() -> String;

    fn from_config(config: Arc<MystikoConfig>, chain_id: u64, address: &str) -> Result<Self>;

    fn get_chain_id(&self) -> u64;

    fn get_contract_address(&self) -> &String;

    fn get_loaded_block(&self) -> u64;

    fn update_loaded_block(&mut self, block: u64);
}

pub trait DatabaseCommitment: DocumentData {
    fn column_chain_id() -> String;

    fn column_contract_address() -> String;

    fn column_bridge_type() -> String;

    fn column_commitment_hash() -> String;

    fn column_status() -> String;

    fn column_block_number() -> String;

    fn column_src_block_number() -> String;

    fn column_included_block_number() -> String;

    fn column_leaf_index() -> String;

    fn column_rollup_fee() -> String;

    fn column_encrypted_note() -> String;

    fn column_queued_transaction_hash() -> String;

    fn column_included_transaction_hash() -> String;

    fn column_src_transaction_hash() -> String;

    fn from_proto(
        config: Arc<MystikoConfig>,
        chain_id: u64,
        address: &str,
        bridge_type: i32,
        proto: ProtoCommitment,
    ) -> Result<Self>;

    fn get_chain_id(&self) -> u64;

    fn get_contract_address(&self) -> &String;

    fn get_bridge_type(&self) -> i32;

    fn get_commitment_hash(&self) -> &BigUint;

    fn get_status(&self) -> i32;

    fn get_block_number(&self) -> u64;

    fn get_src_block_number(&self) -> Option<u64>;

    fn get_included_block_number(&self) -> Option<u64>;

    fn get_leaf_index(&self) -> Option<u64>;

    fn get_rollup_fee(&self) -> Option<&BigUint>;

    fn get_encrypted_note(&self) -> Option<&String>;

    fn get_queued_transaction_hash(&self) -> Option<&String>;

    fn get_included_transaction_hash(&self) -> Option<&String>;

    fn get_src_transaction_hash(&self) -> Option<&String>;

    fn update_by_proto(&mut self, config: Arc<MystikoConfig>, proto: &ProtoCommitment) -> Result<()>;

    fn into_proto(self) -> Result<ProtoCommitment>;
}

pub trait DatabaseNullifier: DocumentData {
    fn column_chain_id() -> String;

    fn column_contract_address() -> String;

    fn column_nullifier() -> String;

    fn column_block_number() -> String;

    fn column_transaction_hash() -> String;

    fn from_proto(config: Arc<MystikoConfig>, chain_id: u64, address: &str, proto: ProtoNullifier) -> Result<Self>;

    fn get_chain_id(&self) -> u64;

    fn get_contract_address(&self) -> &String;

    fn get_nullifier(&self) -> &BigUint;

    fn get_block_number(&self) -> u64;

    fn get_transaction_hash(&self) -> &String;

    fn update_by_proto(&mut self, config: Arc<MystikoConfig>, proto: &ProtoNullifier) -> Result<()>;

    fn into_proto(self) -> Result<ProtoNullifier>;
}
