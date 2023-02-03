use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::ContractType;

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawContractConfig {
    version: u32,
    name: String,
    address: String,
    contract_type: ContractType,
    start_block: u32,
    event_filter_size: Option<u32>,
    indexer_filter_size: Option<u32>,
}