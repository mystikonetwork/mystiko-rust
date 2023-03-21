use crate::common::{BridgeType, ContractType};
use crate::raw::base::Validator;
use crate::raw::contract::base::{RawContractConfig, RawContractConfigTrait};
use crate::raw::validator::{is_ethereum_address, is_number_string, string_vec_each_not_empty};
use serde::{Deserialize, Deserializer, Serialize};
use std::hash::{Hash, Hasher};
use typed_builder::TypedBuilder;
use validator::{Validate, ValidationError};

#[derive(Validate, Serialize, Debug, Clone, Eq, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RawPoolContractConfig {
    #[serde(flatten)]
    pub base: RawContractConfig,

    #[serde(rename = "type")]
    #[serde(skip_serializing)]
    #[validate(custom = "validate_contract_type")]
    #[builder(default = ContractType::Pool)]
    pub contract_type: ContractType,

    #[validate(length(min = 1))]
    pub pool_name: String,

    pub bridge_type: BridgeType,

    #[validate(custom = "is_ethereum_address")]
    #[builder(default = None)]
    pub asset_address: Option<String>,

    #[validate(custom = "is_number_string::<true,false>")]
    #[serde(default = "default_min_rollup_fee")]
    pub min_rollup_fee: String,

    #[validate(custom = "string_vec_each_not_empty")]
    #[serde(default)]
    #[builder(default = vec ! [])]
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

impl Validator for RawPoolContractConfig {
    fn validation(&self) -> Result<(), anyhow::Error> {
        self.base.base.validate_object(self)
    }
}

impl<'de> Deserialize<'de> for RawPoolContractConfig {
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
            pool_name: String,
            bridge_type: BridgeType,
            asset_address: Option<String>,
            min_rollup_fee: Option<String>,
            circuits: Option<Vec<String>>,
        }
        let inner = Inner::deserialize(deserializer)?;
        let contract_type = inner.contract_type.unwrap_or(ContractType::Pool);
        let base_contract_type = contract_type.clone();
        let min_rollup_fee = inner.min_rollup_fee.unwrap_or_else(default_min_rollup_fee);
        let circuits = inner.circuits.unwrap_or(Vec::new());
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
            contract_type,
            pool_name: inner.pool_name,
            bridge_type: inner.bridge_type,
            asset_address: inner.asset_address,
            min_rollup_fee,
            circuits,
        })
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

    fn event_filter_size(&self) -> &Option<u64> {
        &self.base.event_filter_size
    }

    fn indexer_filter_size(&self) -> &Option<u64> {
        &self.base.indexer_filter_size
    }
}

fn validate_contract_type(t: &ContractType) -> Result<(), ValidationError> {
    if *t == ContractType::Pool {
        return Ok(());
    }
    Err(ValidationError::new("contract type error"))
}

fn default_min_rollup_fee() -> String {
    String::from("0")
}
