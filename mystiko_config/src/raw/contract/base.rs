use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{ContractType, validate_object};
use crate::raw::base::RawConfig;

pub trait RawContractConfigTrait: RawConfig {
    fn version(&self) -> &u32;
    fn name(&self) -> &str;
    fn address(&self) -> &str;
    fn contract_type(&self) -> &ContractType;
    fn start_block(&self) -> &u32;
    fn event_filter_size(&self) -> &Option<u32>;
    fn indexer_filter_size(&self) -> &Option<u32>;
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawContractConfig {
    pub version: u32,
    pub name: String,
    pub address: String,
    pub contract_type: ContractType,
    pub start_block: u32,
    pub event_filter_size: Option<u32>,
    pub indexer_filter_size: Option<u32>,
}

impl RawConfig for RawContractConfig {
    fn validate(&self) -> Result<(), Vec<String>> {
        let result = validate_object(self);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        Ok(())
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

    fn event_filter_size(&self) -> &Option<u32> {
        &self.event_filter_size
    }

    fn indexer_filter_size(&self) -> &Option<u32> {
        &self.indexer_filter_size
    }
}