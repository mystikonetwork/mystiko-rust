use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::validate_object;
use crate::raw::asset::RawAssetConfig;
use crate::raw::base::{RawConfig, RawConfigTrait};
use crate::raw::contract::deposit::RawDepositContractConfig;
use crate::raw::contract::pool::RawPoolContractConfig;
use crate::raw::provider::RawProviderConfig;
use crate::raw::validator::{array_unique, is_number_string};

pub const EXPLORER_TX_PLACEHOLDER: &str = "%tx%";

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
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
    pub recommended_amounts: Vec<String>,
    #[validate(url)]
    pub explorer_url: String,
    // TODO test this validate
    #[validate(contains = "EXPLORER_TX_PLACEHOLDER")]
    pub explorer_prefix: String,
    #[validate(length(min = 1))]
    pub providers: Vec<RawProviderConfig>,
    #[validate(url)]
    pub signer_endpoint: String,
    #[validate(range(min = 1))]
    pub event_filter_size: u32,
    #[validate(range(min = 1))]
    pub indexer_filter_size: u32,
    #[validate(custom(function = "array_unique"))]
    pub deposit_contracts: Vec<RawDepositContractConfig>,
    #[validate(custom(function = "array_unique"))]
    pub pool_contracts: Vec<RawPoolContractConfig>,
    #[validate(custom(function = "array_unique"))]
    pub assets: Vec<RawAssetConfig>,
}

impl RawConfigTrait for RawChainConfig {
    fn validate(&self) {
        self.base.validate_object::<&RawChainConfig>(self)
    }
}