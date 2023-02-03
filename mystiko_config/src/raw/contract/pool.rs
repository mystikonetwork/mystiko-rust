use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{BridgeType, ContractType};
use crate::raw::base::RawConfig;
use crate::raw::chain::RawChainConfig;
use crate::raw::contract::base::RawContractConfig;

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawPoolContractConfig {
    pub base: RawContractConfig,
    pub pool_name: String,
    pub bridge_type: BridgeType,
    pub contract_type: ContractType,
    pub asset_address: Option<String>,
    pub min_rollup_fee: String,
    pub circuits: Vec<String>,
}

impl RawConfig for RawChainConfig {}