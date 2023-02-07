use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{AssetType, validate_object};
use crate::raw::base::RawConfig;

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawAssetConfig {
    pub asset_type: AssetType,
    pub asset_symbol: String,
    pub asset_decimals: u32,
    pub asset_address: String,
    pub recommended_amounts: Vec<String>,
}

impl RawConfig for RawAssetConfig {
    fn validate(&self) -> Result<(), Vec<String>> {
        let result = validate_object(self);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        Ok(())
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
            asset_type,
            asset_symbol,
            asset_decimals,
            asset_address,
            recommended_amounts,
        }
    }
}