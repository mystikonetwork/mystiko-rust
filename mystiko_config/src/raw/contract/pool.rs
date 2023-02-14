use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use crate::common::{BridgeType, ContractType, validate_object};
use crate::raw::base::RawConfigTrait;
use crate::raw::chain::RawChainConfig;
use crate::raw::contract::base::{RawContractConfig, RawContractConfigTrait};
use crate::raw::validator::{is_ethereum_address, is_number_string};

fn default_contract_type() -> ContractType {
    ContractType::Pool
}

fn validate_contract_type(t: &ContractType) -> Result<(), ValidationError> {
    if *t == ContractType::Pool {
        return Ok(());
    }
    Err(ValidationError::new("contract type error"))
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawPoolContractConfig {
    #[serde(flatten)]
    pub base: RawContractConfig,

    #[serde(default = "default_contract_type")]
    #[validate(custom = "validate_contract_type")]
    pub contract_type: ContractType,

    #[validate(length(min = 1))]
    pub pool_name: String,

    pub bridge_type: BridgeType,

    #[validate(custom = "is_ethereum_address")]
    pub asset_address: Option<String>,

    #[validate(custom = "is_number_string::<true,false>")]
    pub min_rollup_fee: String,

    #[validate(length(min = 1))]
    pub circuits: Vec<String>,
}

impl RawPoolContractConfig {
    pub fn new(
        base: RawContractConfig,
        pool_name: String,
        bridge_type: BridgeType,
        asset_address: Option<String>,
        min_rollup_fee: String,
        circuits: Vec<String>,
    ) -> Self {
        Self {
            base,
            pool_name,
            bridge_type,
            asset_address,
            min_rollup_fee,
            circuits,
            contract_type: default_contract_type(),
        }
    }
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
        &self.contract_type
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