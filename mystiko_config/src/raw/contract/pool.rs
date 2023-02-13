use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{BridgeType, ContractType, validate_object};
use crate::raw::base::RawConfigTrait;
use crate::raw::chain::RawChainConfig;
use crate::raw::contract::base::{RawContractConfig, RawContractConfigTrait};

#[derive(Validate, Serialize, Deserialize, Debug, Clone, Eq)]
pub struct RawPoolContractConfig {
    pub base: RawContractConfig,
    pub pool_name: String,
    pub bridge_type: BridgeType,
    pub contract_type: ContractType,
    pub asset_address: Option<String>,
    pub min_rollup_fee: String,
    pub circuits: Vec<String>,
}

impl Hash for RawPoolContractConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.address.hash(state)
    }
}

impl PartialEq for RawPoolContractConfig {
    fn eq(&self, other: &Self) -> bool {
        self.base.address == other.base.address
    }
}

impl RawConfigTrait for RawPoolContractConfig {
    fn validate(&self) {
        self.base.base.validate_object(self)
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