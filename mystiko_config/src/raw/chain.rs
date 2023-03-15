use crate::errors::ValidationError;
use crate::raw::asset::RawAssetConfig;
use crate::raw::base::{RawConfig, Validator};
use crate::raw::contract::deposit::RawDepositContractConfig;
use crate::raw::contract::pool::RawPoolContractConfig;
use crate::raw::provider::RawProviderConfig;
use crate::raw::validator::{array_unique, is_number_string, validate_nested_vec};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use validator::Validate;

pub const EXPLORER_TX_PLACEHOLDER: &str = "%tx%";
pub const EXPLORER_DEFAULT_PREFIX: &str = "/tx/%tx%";

fn default_event_filter_size() -> u64 {
    200000
}

fn default_indexer_filter_size() -> u64 {
    500000
}

fn default_explorer_prefix() -> String {
    EXPLORER_DEFAULT_PREFIX.to_string()
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawChainConfig {
    #[serde(default)]
    pub base: RawConfig,

    #[validate(range(min = 1))]
    pub chain_id: u32,

    #[validate(length(min = 1))]
    pub name: String,

    #[validate(length(min = 1))]
    pub asset_symbol: String,

    #[validate(range(min = 1))]
    pub asset_decimals: u32,

    #[validate(
        custom(function = "array_unique"),
        custom(function = "is_number_string::<true, true>")
    )]
    #[serde(default)]
    pub recommended_amounts: Vec<String>,

    #[validate(url)]
    pub explorer_url: String,

    #[validate(contains = "%tx%")]
    #[serde(default = "default_explorer_prefix")]
    pub explorer_prefix: String,

    #[validate(length(min = 1))]
    #[validate(custom = "validate_nested_vec")]
    pub providers: Vec<RawProviderConfig>,

    #[validate(url)]
    pub signer_endpoint: String,

    #[validate(range(min = 1))]
    #[serde(default = "default_event_filter_size")]
    pub event_filter_size: u64,

    #[validate(range(min = 1))]
    #[serde(default = "default_indexer_filter_size")]
    pub indexer_filter_size: u64,

    #[validate(custom(function = "array_unique"))]
    #[validate(custom = "validate_nested_vec")]
    pub deposit_contracts: Vec<RawDepositContractConfig>,

    #[validate(custom(function = "array_unique"))]
    #[validate(custom = "validate_nested_vec")]
    pub pool_contracts: Vec<RawPoolContractConfig>,

    #[validate(custom(function = "array_unique"))]
    #[validate(custom = "validate_nested_vec")]
    pub assets: Vec<RawAssetConfig>,
}

impl PartialEq for RawChainConfig {
    fn eq(&self, other: &Self) -> bool {
        self.chain_id == other.chain_id
    }
}

impl Hash for RawChainConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.chain_id.hash(state)
    }
}

impl Validator for RawChainConfig {
    fn validation(&self) -> Result<(), ValidationError> {
        self.base.validate_object::<&RawChainConfig>(self)
    }
}

impl RawChainConfig {
    pub fn new(
        chain_id: u32,
        name: String,
        asset_symbol: String,
        asset_decimals: u32,
        recommended_amounts: Vec<String>,
        explorer_url: String,
        explorer_prefix: String,
        event_filter_size: Option<u64>,
        indexer_filter_size: Option<u64>,
        providers: Vec<RawProviderConfig>,
        signer_endpoint: String,
        deposit_contracts: Vec<RawDepositContractConfig>,
        pool_contracts: Vec<RawPoolContractConfig>,
        assets: Vec<RawAssetConfig>,
    ) -> RawChainConfig {
        let event_filter_size = match event_filter_size {
            None => default_event_filter_size(),
            Some(value) => value,
        };
        let indexer_filter_size = match indexer_filter_size {
            None => default_indexer_filter_size(),
            Some(value) => value,
        };
        Self {
            base: RawConfig::default(),
            chain_id,
            name,
            asset_symbol,
            asset_decimals,
            recommended_amounts,
            explorer_url,
            explorer_prefix,
            providers,
            signer_endpoint,
            event_filter_size,
            indexer_filter_size,
            deposit_contracts,
            pool_contracts,
            assets,
        }
    }
}
