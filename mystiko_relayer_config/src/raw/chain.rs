use crate::raw::contract::RawContractConfig;
use crate::raw::transaction_info::RawTransactionInfoConfig;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(TypedBuilder, Validate, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawChainConfig {
    #[validate(length(min = 1))]
    pub name: String,

    #[validate(range(min = 1))]
    pub chain_id: u32,

    #[validate(length(min = 1))]
    pub asset_symbol: String,

    // TODO validate ethereum address
    #[validate(length(min = 1))]
    pub relayer_contract_address: String,

    #[validate]
    #[serde(default = "default_contracts")]
    #[builder(default = default_contracts())]
    pub contracts: Vec<Arc<RawContractConfig>>,

    #[validate]
    pub transaction_info: Arc<RawTransactionInfoConfig>,
}

fn default_contracts() -> Vec<Arc<RawContractConfig>> {
    Vec::new()
}
