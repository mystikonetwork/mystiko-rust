use crate::raw::asset::RawAssetConfig;
use crate::raw::contract::deposit::RawDepositContractConfig;
use crate::raw::contract::pool::RawPoolContractConfig;
use crate::raw::provider::RawProviderConfig;
use mystiko_types::ProviderType;
use mystiko_validator::validate::{array_unique, is_number_string_vec, validate_nested_vec};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use typed_builder::TypedBuilder;
use validator::Validate;

pub const EXPLORER_TX_PLACEHOLDER: &str = "%tx%";
pub const EXPLORER_DEFAULT_PREFIX: &str = "/tx/%tx%";

#[derive(TypedBuilder, Validate, Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RawChainConfig {
    #[validate(range(min = 1))]
    pub chain_id: u64,

    #[validate(length(min = 1))]
    pub name: String,

    #[validate(length(min = 1))]
    pub asset_symbol: String,

    #[validate(range(min = 1))]
    #[serde(default = "default_asset_decimals")]
    #[builder(default = default_asset_decimals())]
    pub asset_decimals: u32,

    #[validate(
        custom(function = "array_unique"),
        custom(function = "is_number_string_vec::<true>")
    )]
    #[serde(default)]
    #[builder(default = vec ! [])]
    pub recommended_amounts: Vec<String>,

    #[validate(url)]
    pub explorer_url: String,

    #[validate(contains = "%tx%")]
    #[serde(default = "default_explorer_prefix")]
    #[builder(default = default_explorer_prefix())]
    pub explorer_prefix: String,

    #[validate(length(min = 1))]
    #[validate(custom = "validate_nested_vec")]
    pub providers: Vec<Arc<RawProviderConfig>>,

    #[serde(default = "default_provider_type")]
    #[builder(default = default_provider_type())]
    pub provider_type: ProviderType,

    #[validate(range(min = 50, max = 100))]
    #[serde(default = "default_quorum_percentage")]
    #[builder(default = default_quorum_percentage())]
    pub provider_quorum_percentage: u8,

    #[validate(url)]
    pub signer_endpoint: String,

    #[validate(range(min = 0))]
    #[serde(default = "default_event_filter_block_backoff")]
    #[builder(default = default_event_filter_block_backoff())]
    pub event_filter_block_backoff: u64,

    #[validate(range(min = 1))]
    #[serde(default = "default_event_filter_size")]
    #[builder(default = default_event_filter_size())]
    pub event_filter_size: u64,

    #[validate(range(min = 1))]
    #[serde(default = "default_indexer_filter_size")]
    #[builder(default = default_indexer_filter_size())]
    pub indexer_filter_size: u64,

    #[validate(custom(function = "array_unique"))]
    #[validate(custom = "validate_nested_vec")]
    #[serde(default)]
    #[builder(default = vec ! [])]
    pub deposit_contracts: Vec<Arc<RawDepositContractConfig>>,

    #[validate(custom(function = "array_unique"))]
    #[validate(custom = "validate_nested_vec")]
    #[serde(default)]
    #[builder(default = vec ! [])]
    pub pool_contracts: Vec<Arc<RawPoolContractConfig>>,

    #[validate(custom(function = "array_unique"))]
    #[validate(custom = "validate_nested_vec")]
    #[serde(default)]
    #[builder(default = vec ! [])]
    pub assets: Vec<Arc<RawAssetConfig>>,
}

impl Hash for RawChainConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.chain_id.hash(state)
    }
}

fn default_event_filter_block_backoff() -> u64 {
    0
}

fn default_event_filter_size() -> u64 {
    200000
}

fn default_indexer_filter_size() -> u64 {
    500000
}

fn default_explorer_prefix() -> String {
    EXPLORER_DEFAULT_PREFIX.to_string()
}

fn default_provider_type() -> ProviderType {
    ProviderType::Failover
}

fn default_quorum_percentage() -> u8 {
    50
}

fn default_asset_decimals() -> u32 {
    18
}
