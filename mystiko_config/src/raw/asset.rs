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
    pub fn new() -> RawAssetConfig {
        RawAssetConfig {
            asset_type: AssetType::ERC20,
            asset_symbol: "".to_string(),
            asset_decimals: 0,
            asset_address: "".to_string(),
            recommended_amounts: vec![],
        }
    }
}