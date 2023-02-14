use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use crate::common::{BridgeType, ContractType, validate_object};
use crate::raw::base::RawConfigTrait;
use crate::raw::contract::base::{RawContractConfig, RawContractConfigTrait};
use crate::raw::validator::{is_ethereum_address, is_number_string};

fn default_contract_type() -> ContractType {
    ContractType::Deposit
}

fn validate_contract_type(t: &ContractType) -> Result<(), ValidationError> {
    if *t == ContractType::Deposit {
        return Ok(());
    }
    Err(ValidationError::new("contract type error"))
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawDepositContractConfig {
    #[serde(flatten)]
    pub base: RawContractConfig,

    pub bridge_type: BridgeType,

    #[serde(default = "default_contract_type")]
    #[serde(rename = "type")]
    #[validate(custom = "validate_contract_type")]
    pub contract_type: ContractType,

    #[validate(custom = "is_ethereum_address")]
    pub pool_address: String,

    pub disabled: bool,

    #[validate(range(min = 1))]
    pub peer_chain_id: Option<u32>,

    #[validate(custom = "is_ethereum_address")]
    pub peer_contract_address: Option<String>,

    #[validate(custom = "is_number_string::<true,false>")]
    pub min_amount: String,

    #[validate(custom = "is_number_string::<true,false>")]
    pub max_amount: String,

    #[validate(custom = "is_number_string::<true,false>")]
    pub min_bridge_fee: String,

    #[validate(custom = "is_number_string::<true,false>")]
    pub min_executor_fee: String,

    #[validate(custom = "is_ethereum_address")]
    pub bridge_fee_asset_address: Option<String>,

    #[validate(custom = "is_ethereum_address")]
    pub executor_fee_asset_address: Option<String>,

    #[validate(range(min = 0))]
    pub service_fee: Option<u32>,

    #[validate(range(min = 1))]
    pub service_fee_divider: Option<u32>,
}

impl RawDepositContractConfig {
    pub fn new(
        base: RawContractConfig,
        bridge_type: BridgeType,
        pool_address: String,
        disabled: bool,
        peer_chain_id: Option<u32>,
        peer_contract_address: Option<String>,
        min_amount: String, max_amount: String,
        min_bridge_fee: String,
        min_executor_fee: String,
        bridge_fee_asset_address: Option<String>,
        executor_fee_asset_address: Option<String>,
        mut service_fee: Option<u32>,
        mut service_fee_divider: Option<u32>,
    ) -> Self {
        if service_fee.is_none() {
            service_fee = Some(0)
        }
        if service_fee_divider.is_none() {
            service_fee_divider = Some(1000000)
        }
        Self {
            base,
            bridge_type,
            pool_address,
            disabled,
            peer_chain_id,
            peer_contract_address,
            min_amount,
            max_amount,
            min_bridge_fee,
            min_executor_fee,
            bridge_fee_asset_address,
            executor_fee_asset_address,
            service_fee,
            service_fee_divider,
            contract_type: default_contract_type(),
        }
    }
}

impl Hash for RawDepositContractConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.address.hash(state)
    }
}

impl PartialEq for RawDepositContractConfig {
    fn eq(&self, other: &Self) -> bool {
        self.base.address == other.base.address
    }
}

impl RawConfigTrait for RawDepositContractConfig {
    fn validate(&self) {
        self.base.base.validate_object(self)
    }
}

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