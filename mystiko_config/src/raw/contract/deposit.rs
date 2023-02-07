use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{BridgeType, ContractType};
use crate::raw::base::RawConfig;
use crate::raw::contract::base::{RawContractConfig, RawContractConfigTrait};

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawDepositContractConfig {
    pub base: RawContractConfig,
    pub contract_type: ContractType,
    pub bridge_type: BridgeType,
    pub pool_address: String,
    pub disabled: bool,
    pub peer_chain_id: Option<u32>,
    pub peer_contract_address: Option<String>,
    pub min_amount: String,
    pub max_amount: String,
    pub min_bridge_fee: String,
    pub min_executor_fee: String,
    pub bridge_fee_asset_address: String,
    pub executor_fee_asset_address: String,
    pub service_fee: u32,
    pub service_fee_divider: u32,
}

impl RawConfig for RawDepositContractConfig {}

impl RawContractConfigTrait for RawDepositContractConfig {
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