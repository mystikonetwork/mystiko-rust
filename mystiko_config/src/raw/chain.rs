use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::validate_object;
use crate::raw::asset::RawAssetConfig;
use crate::raw::base::RawConfigTrait;
use crate::raw::contract::deposit::RawDepositContractConfig;
use crate::raw::contract::pool::RawPoolContractConfig;
use crate::raw::provider::RawProviderConfig;
use crate::raw::validator::{array_unique, is_number_string};

pub const EXPLORER_TX_PLACEHOLDER: &str = "%tx%";

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawChainConfig {
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
    #[validate(contains= "EXPLORER_TX_PLACEHOLDER")]
    pub explorer_prefix: String,
    pub providers: Vec<RawProviderConfig>,
    pub signer_endpoint: String,
    pub event_filter_size: u32,
    pub indexer_filter_size: u32,
    pub deposit_contracts: Vec<RawDepositContractConfig>,
    pub pool_contracts: Vec<RawPoolContractConfig>,
    pub assets: Vec<RawAssetConfig>,
}

impl RawConfigTrait for RawChainConfig {
    fn validate(&self) -> Result<(), Vec<String>> {
        let result = validate_object(self);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        Ok(())
    }
}