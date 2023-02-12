use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use crate::common::{AssetType, validate_object};
use crate::raw::base::{RawConfig, RawConfigTrait};
use crate::raw::validator::{is_ethereum_address, array_unique, is_number_string};

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawAssetConfig {
    #[serde(default)]
    pub base: RawConfig,
    pub asset_type: AssetType,
    #[validate(length(min = 1))]
    pub asset_symbol: String,
    #[validate(range(min = 1))]
    pub asset_decimals: u32,
    #[validate(custom = "is_ethereum_address")]
    pub asset_address: String,
    #[validate(
    custom(function = "array_unique"),
    custom(function = "is_number_string::<true, true>")
    )]
    pub recommended_amounts: Vec<String>,
}

impl RawConfigTrait for RawAssetConfig {
    fn validate(&self) -> Result<(), Vec<String>> {
        self.base.validate_object::<&RawAssetConfig>(self)
    }
}

impl RawAssetConfig {
    pub fn new(
        asset_type: AssetType,
        asset_symbol: String,
        asset_decimals: u32,
        asset_address: String,
        recommended_amounts: Vec<String>,
    ) -> RawAssetConfig {
        Self {
            base: RawConfig::default(),
            asset_type,
            asset_symbol,
            asset_decimals,
            asset_address,
            recommended_amounts,
        }
    }
}