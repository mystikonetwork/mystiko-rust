use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{BridgeType, ContractType, validate_object};
use crate::raw::base::RawConfig;
use crate::raw::chain::RawChainConfig;
use crate::raw::contract::base::{RawContractConfig, RawContractConfigTrait};

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

impl RawConfig for RawPoolContractConfig {
    fn validate(&self) -> Result<(), Vec<String>> {
        let result = validate_object(self);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        Ok(())
    }
}

impl RawContractConfigTrait for RawPoolContractConfig {
    fn version(&self) -> &u32 {
        &self.base.version
    }

    fn name(&self) -> &str {
        &self.base.name
    }

    fn address(&self) -> &str {
        &self.base.address
    }

    fn contract_type(&self) -> &ContractType {
        &self.base.contract_type
    }

    fn start_block(&self) -> &u32 {
        &self.base.start_block
    }

    fn event_filter_size(&self) -> &Option<u32> {
        &self.base.event_filter_size
    }

    fn indexer_filter_size(&self) -> &Option<u32> {
        &self.base.indexer_filter_size
    }
}