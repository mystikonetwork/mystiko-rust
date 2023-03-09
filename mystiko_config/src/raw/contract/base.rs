use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{ContractType};
use crate::errors;
use crate::raw::base::{RawConfig, Validator};
use crate::raw::validator::{is_ethereum_address};

pub trait RawContractConfigTrait: Validator {
    fn version(&self) -> &u32;
    fn name(&self) -> &str;
    fn address(&self) -> &str;
    fn contract_type(&self) -> &ContractType;
    fn start_block(&self) -> &u32;
    fn event_filter_size(&self) -> &Option<u64>;
    fn indexer_filter_size(&self) -> &Option<u64>;
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawContractConfig {
    #[serde(default)]
    pub base: RawConfig,

    #[validate(range(min = 1))]
    pub version: u32,

    #[validate(length(min = 1))]
    pub name: String,

    #[validate(custom = "is_ethereum_address")]
    pub address: String,

    #[serde(rename = "type")]
    pub contract_type: ContractType,

    #[validate(range(min = 1))]
    pub start_block: u32,

    #[validate(range(min = 1))]
    pub event_filter_size: Option<u64>,

    #[validate(range(min = 1))]
    pub indexer_filter_size: Option<u64>,
}

impl RawContractConfig {
    pub fn new(
        version: u32,
        name: String,
        address: String,
        contract_type: ContractType,
        start_block: u32,
        event_filter_size: Option<u64>,
        indexer_filter_size: Option<u64>,
    ) -> Self {
        Self {
            base: RawConfig::default(),
            version,
            name,
            address,
            contract_type,
            start_block,
            event_filter_size,
            indexer_filter_size,
        }
    }
}

impl Validator for RawContractConfig {
    fn validation(&self) -> Result<(), errors::ValidationError> {
        self.base.validate_object(self)
    }
}

impl RawContractConfigTrait for RawContractConfig {
    fn version(&self) -> &u32 {
        &self.version
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn address(&self) -> &str {
        &self.address
    }

    fn contract_type(&self) -> &ContractType {
        &self.contract_type
    }

    fn start_block(&self) -> &u32 {
        &self.start_block
    }

    fn event_filter_size(&self) -> &Option<u64> {
        &self.event_filter_size
    }

    fn indexer_filter_size(&self) -> &Option<u64> {
        &self.indexer_filter_size
    }
}
