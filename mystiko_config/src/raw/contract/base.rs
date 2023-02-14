use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{ContractType, validate_object};
use crate::raw::base::{RawConfig, RawConfigTrait};
use crate::raw::validator::{is_ethereum_address};

pub trait RawContractConfigTrait: RawConfigTrait {
    fn version(&self) -> &u32;
    fn name(&self) -> &str;
    fn address(&self) -> &str;
    fn contract_type(&self) -> &ContractType;
    fn start_block(&self) -> &u32;
    fn event_filter_size(&self) -> &Option<u32>;
    fn indexer_filter_size(&self) -> &Option<u32>;
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

    #[validate(range(min = 1))]
    pub start_block: u32,

    #[validate(range(min = 1))]
    pub event_filter_size: Option<u32>,

    #[validate(range(min = 1))]
    pub indexer_filter_size: Option<u32>,
}

impl RawContractConfig {
    pub fn new(
        version: u32,
        name: String,
        address: String,
        start_block: u32,
        event_filter_size: Option<u32>,
        indexer_filter_size: Option<u32>,
    ) -> Self {
        Self {
            base: RawConfig::default(),
            version,
            name,
            address,
            start_block,
            event_filter_size,
            indexer_filter_size,
        }
    }
}

impl RawConfigTrait for RawContractConfig {
    fn validate(&self) {
        self.base.validate_object(self)
    }
}
