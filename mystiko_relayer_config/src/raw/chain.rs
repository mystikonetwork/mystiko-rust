use crate::raw::asset::RawAssetConfig;
use crate::raw::transaction_info::RawTransactionInfoConfig;
use mystiko_validator::validate::is_ethereum_address;
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
    pub chain_id: u64,

    #[validate(length(min = 1))]
    pub asset_symbol: String,

    #[validate(range(min = 1))]
    #[serde(default = "default_asset_decimals")]
    #[builder(default = default_asset_decimals())]
    pub asset_decimals: u32,

    #[validate(custom = "is_ethereum_address")]
    #[validate(length(min = 1))]
    pub relayer_contract_address: String,

    #[validate]
    #[serde(default = "default_assets")]
    #[builder(default = default_assets())]
    pub assets: Vec<Arc<RawAssetConfig>>,

    #[validate]
    pub transaction_info: Arc<RawTransactionInfoConfig>,
}

fn default_assets() -> Vec<Arc<RawAssetConfig>> {
    Vec::new()
}

fn default_asset_decimals() -> u32 {
    18
}
