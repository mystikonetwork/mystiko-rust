use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use validator::{Validate};
use crate::common::{AssetType};
use crate::raw::base::{RawConfig, Validator};
use crate::raw::validator::{is_ethereum_address, array_unique, is_number_string};

#[derive(Validate, Serialize, Deserialize, Debug, Clone, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RawAssetConfig {
    #[serde(default)]
    pub base: RawConfig,

    #[serde(rename = "assetType")]
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
    #[serde(default)]
    pub recommended_amounts: Vec<String>,
}

impl Hash for RawAssetConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.asset_address.hash(state)
    }
}

impl PartialEq for RawAssetConfig {
    fn eq(&self, other: &Self) -> bool {
        self.asset_address == other.asset_address
    }
}

impl Validator for RawAssetConfig {
    fn validation(&self) {
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