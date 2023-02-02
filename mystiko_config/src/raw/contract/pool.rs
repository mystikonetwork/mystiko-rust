use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{BridgeType, ContractType};
use crate::raw::contract::base::RawContractConfig;

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct RawPoolContractConfig {
    contract: RawContractConfig,
    pool_name: String,
    bridge_type: BridgeType,
    contract_type: ContractType,
    asset_address: String,
    min_rollup_fee: String,
    circuits: Vec<String>,
}