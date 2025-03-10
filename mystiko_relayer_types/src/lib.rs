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
use mystiko_protos::core::v1::SpendType;
use mystiko_types::{BridgeType, CircuitType};
use mystiko_validator::validate::is_ethereum_address;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use typed_builder::TypedBuilder;
use validator::{Validate, ValidationError};

#[derive(Validate, TypedBuilder, Serialize, Deserialize, Debug)]
#[builder(field_defaults(setter(into)))]
pub struct RelayTransactRequest {
    #[validate(url)]
    pub relayer_url: String,
    #[validate]
    pub data: TransactRequestData,
}

#[derive(Validate, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[builder(field_defaults(setter(into)))]
pub struct TransactRequestData {
    pub contract_param: TransactRequest,
    pub spend_type: SpendType,
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
#[builder(field_defaults(setter(into)))]
pub struct RelayTransactResponse {
    pub uuid: String,
}

#[derive(Validate, TypedBuilder, Serialize, Deserialize, Debug)]
#[builder(field_defaults(setter(into)))]
pub struct RelayTransactStatusRequest {
    #[validate(url)]
    pub relayer_url: String,
    #[validate(length(min = 1))]
    pub uuid: String,
}

#[derive(TypedBuilder, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[builder(field_defaults(setter(into)))]
pub struct RelayTransactStatusResponse {
    pub uuid: String,
    pub chain_id: u64,
    pub spend_type: SpendType,
    pub status: TransactStatus,
    pub transaction_hash: Option<String>,
    pub error_msg: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum TransactStatus {
    Queued,
    Pending,
    #[default]
    Succeeded,
    Failed,
}

#[derive(Validate, TypedBuilder, Serialize, Deserialize, Debug)]
#[builder(field_defaults(setter(into)))]
pub struct WaitingTransactionRequest {
    #[validate(url)]
    pub relayer_url: String,
    #[validate(length(min = 1))]
    pub uuid: String,
    #[builder(default)]
    pub waiting_status: TransactStatus,
    #[builder(default)]
    pub timeout: Option<Duration>,
    #[builder(default)]
    pub interval: Option<Duration>,
}

#[derive(Validate, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[builder(field_defaults(setter(into)))]
pub struct RegisterInfoRequest {
    #[validate(range(min = 1))]
    pub chain_id: u64,
    #[serde(default)]
    #[builder(default)]
    pub name: Option<String>,
    #[validate]
    #[serde(default)]
    #[builder(default)]
    pub options: Option<RegisterOptions>,
}

#[derive(Validate, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[builder(field_defaults(setter(into)))]
pub struct RegisterOptions {
    #[validate(length(min = 1))]
    pub asset_symbol: String,
    #[validate(custom = "is_valid_circuit_type")]
    pub circuit_type: CircuitType,
    pub show_unavailable: bool,
}

#[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[builder(field_defaults(setter(into)))]
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
#[builder(field_defaults(setter(into)))]
pub struct HandshakeResponse {
    pub package_version: String,
    pub api_version: Vec<String>,
}

#[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[builder(field_defaults(setter(into)))]
pub struct ContractInfo {
    pub asset_symbol: String,
    pub relayer_fee_of_ten_thousandth: u32,
    pub minimum_gas_fee: Option<String>,
}

fn is_valid_circuit_type(value: &CircuitType) -> Result<(), ValidationError> {
    let allowed = [
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
