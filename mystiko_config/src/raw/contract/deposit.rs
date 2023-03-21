use crate::common::{BridgeType, ContractType};
use crate::raw::base::Validator;
use crate::raw::contract::base::{RawContractConfig, RawContractConfigTrait};
use crate::raw::validator::{is_ethereum_address, is_number_string};
use serde::{Deserialize, Deserializer, Serialize};
use std::hash::{Hash, Hasher};
use typed_builder::TypedBuilder;
use validator::{Validate, ValidationError};

#[derive(Validate, Serialize, Debug, Clone, Eq, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RawDepositContractConfig {
    #[serde(flatten)]
    pub base: RawContractConfig,

    pub bridge_type: BridgeType,

    #[serde(rename = "type")]
    #[serde(skip_serializing)]
    #[validate(custom = "validate_contract_type")]
    #[builder(default = ContractType::Deposit)]
    pub contract_type: ContractType,

    #[validate(custom = "is_ethereum_address")]
    pub pool_address: String,

    pub disabled: bool,

    #[validate(range(min = 1))]
    #[builder(default = None)]
    pub peer_chain_id: Option<u32>,

    #[validate(custom = "is_ethereum_address")]
    #[builder(default = None)]
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
    #[builder(default = None)]
    pub bridge_fee_asset_address: Option<String>,

    #[validate(custom = "is_ethereum_address")]
    #[builder(default = None)]
    pub executor_fee_asset_address: Option<String>,

    #[validate(range(min = 0))]
    #[builder(default = 0)]
    pub service_fee: u32,

    #[validate(range(min = 1))]
    #[builder(default = 1000000)]
    pub service_fee_divider: u32,
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

impl Validator for RawDepositContractConfig {
    fn validation(&self) -> Result<(), anyhow::Error> {
        self.base.base.validate_object(self)
    }
}

impl<'de> Deserialize<'de> for RawDepositContractConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Inner {
            version: u32,
            name: String,
            address: String,
            #[serde(rename = "type")]
            contract_type: Option<ContractType>,
            start_block: u32,
            even_filter_size: Option<u64>,
            indexer_filter_size: Option<u64>,
            bridge_type: BridgeType,
            pool_address: String,
            disabled: Option<bool>,
            peer_chain_id: Option<u32>,
            peer_contract_address: Option<String>,
            min_amount: String,
            max_amount: String,
            min_bridge_fee: Option<String>,
            min_executor_fee: Option<String>,
            bridge_fee_asset_address: Option<String>,
            executor_fee_asset_address: Option<String>,
            service_fee: Option<u32>,
            service_fee_divider: Option<u32>,
        }
        let inner = Inner::deserialize(deserializer)?;
        let contract_type = inner.contract_type.unwrap_or(ContractType::Deposit);
        let base_contract_type = contract_type.clone();
        let service_fee = inner.service_fee.unwrap_or(0);
        let service_fee_divider = inner.service_fee_divider.unwrap_or(1000000);
        let disabled = inner.disabled.unwrap_or(false);
        let min_bridge_fee = inner.min_bridge_fee.unwrap_or("0".to_string());
        let min_executor_fee = inner.min_executor_fee.unwrap_or("0".to_string());
        Ok(Self {
            base: RawContractConfig {
                base: Default::default(),
                version: inner.version,
                name: inner.name,
                address: inner.address,
                contract_type: base_contract_type,
                start_block: inner.start_block,
                event_filter_size: inner.even_filter_size,
                indexer_filter_size: inner.indexer_filter_size,
            },
            bridge_type: inner.bridge_type,
            contract_type,
            pool_address: inner.pool_address,
            disabled,
            peer_chain_id: inner.peer_chain_id,
            peer_contract_address: inner.peer_contract_address,
            min_amount: inner.min_amount,
            max_amount: inner.max_amount,
            min_bridge_fee,
            min_executor_fee,
            bridge_fee_asset_address: inner.bridge_fee_asset_address,
            executor_fee_asset_address: inner.executor_fee_asset_address,
            service_fee,
            service_fee_divider,
        })
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
        self.base.address()
    }

    fn contract_type(&self) -> &ContractType {
        &self.contract_type
    }

    fn start_block(&self) -> &u32 {
        &self.base.start_block
    }

    fn event_filter_size(&self) -> &Option<u64> {
        &self.base.event_filter_size
    }

    fn indexer_filter_size(&self) -> &Option<u64> {
        &self.base.indexer_filter_size
    }
}

fn validate_contract_type(t: &ContractType) -> Result<(), ValidationError> {
    if *t == ContractType::Deposit {
        return Ok(());
    }
    Err(ValidationError::new("contract type error"))
}
