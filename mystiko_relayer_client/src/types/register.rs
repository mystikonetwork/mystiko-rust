use mystiko_relayer_types::ContractInfo;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Serialize, Deserialize, Debug)]
pub struct RegisterInfo {
    pub support: bool,
    pub available: bool,
    pub chain_id: u64,
    pub url: String,
    pub name: String,
    pub relayer_address: String,
    pub relayer_contract_address: String,
    pub contracts: Vec<ContractInfo>,
}
