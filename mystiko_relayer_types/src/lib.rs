extern crate actix_web;
extern crate mystiko_abi;
extern crate mystiko_types;
extern crate mystiko_validator;
extern crate num_bigint;
extern crate serde;
extern crate serde_json;
extern crate typed_builder;
extern crate validator;

pub mod response;

use mystiko_abi::commitment_pool::TransactRequest;
use mystiko_types::{BridgeType, CircuitType, TransactionType};
use mystiko_validator::validate::is_ethereum_address;
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use typed_builder::TypedBuilder;
use validator::{Validate, ValidationError};

#[derive(Validate, TypedBuilder, Serialize, Deserialize, Debug)]
pub struct RelayTransactRequest {
    #[validate(url)]
    pub relayer_url: String,
    #[validate]
    pub data: TransactRequestData,
}

#[derive(Validate, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
pub struct TransactRequestData {
    pub contract_param: TransactRequest,
    pub transaction_type: TransactionType,
    pub bridge_type: BridgeType,
    #[validate(range(min = 1))]
    pub chain_id: u64,
    pub asset_symbol: String,
    pub asset_decimals: u32,
    #[validate(custom = "is_ethereum_address")]
    pub pool_address: String,
    #[validate(custom = "is_valid_circuit_type")]
    pub circuit_type: CircuitType,
    pub signature: String,
}

#[derive(TypedBuilder, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RelayTransactResponse {
    pub uuid: String,
}

#[derive(Validate, TypedBuilder, Serialize, Deserialize, Debug)]
pub struct RelayTransactStatusRequest {
    #[validate(url)]
    pub relayer_url: String,
    #[validate(length(min = 1))]
    pub uuid: String,
}

#[derive(TypedBuilder, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RelayTransactStatusResponse {
    pub uuid: String,
    pub chain_id: u64,
    pub transaction_type: TransactionType,
    pub status: TransactStatus,
    pub transaction_hash: Option<String>,
    pub error_msg: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TransactStatus {
    Queued,
    Pending,
    Succeeded,
    Failed,
}

#[derive(Validate, TypedBuilder, Serialize, Deserialize, Debug)]
pub struct WaitingTransactionRequest {
    #[validate(url)]
    pub relayer_url: String,
    #[validate(length(min = 1))]
    pub uuid: String,
    pub waiting_status: TransactStatus,
    pub timeout: Duration,
    pub interval: Option<Duration>,
}

#[derive(Validate, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
pub struct RegisterInfoRequest {
    #[validate(range(min = 1))]
    pub chain_id: u64,
    #[validate]
    #[serde(default)]
    #[builder(default)]
    pub options: Option<RegisterOptions>,
}

#[derive(Validate, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
pub struct RegisterOptions {
    #[validate(length(min = 1))]
    pub asset_symbol: String,
    #[validate(custom = "is_valid_circuit_type")]
    pub circuit_type: CircuitType,
    pub show_unavailable: bool,
}

#[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RegisterInfoResponse {
    pub chain_id: u64,
    pub support: bool,
    #[builder(default, setter(strip_option))]
    pub available: Option<bool>,
    #[builder(default, setter(strip_option))]
    pub relayer_contract_address: Option<String>,
    #[builder(default, setter(strip_option))]
    pub contracts: Option<Vec<ContractInfo>>,
}

#[derive(TypedBuilder, Serialize, Deserialize, Debug)]
pub struct PingResponse {
    pub api_version: Vec<String>,
}

#[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ContractInfo {
    pub asset_symbol: String,
    pub relayer_fee_of_ten_thousandth: u32,
    pub minimum_gas_fee: Option<BigInt>,
}

fn is_valid_circuit_type(value: &CircuitType) -> Result<(), ValidationError> {
    let allowed = vec![
        CircuitType::Transaction1x0,
        CircuitType::Transaction1x1,
        CircuitType::Transaction1x2,
        CircuitType::Transaction2x0,
        CircuitType::Transaction2x1,
        CircuitType::Transaction2x2,
    ];
    if allowed.contains(value) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid circuit type"))
    }
}
