use serde::{Deserialize, Serialize};
use crate::common::AssetType;
use crate::raw::base::RawConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawAssetConfig {
    pub asset_type: AssetType,
    pub asset_symbol: String,
    pub asset_decimals: u32,
    pub asset_address: String,
    pub recommended_amounts: Vec<String>,
}

impl RawConfig for RawAssetConfig {}

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