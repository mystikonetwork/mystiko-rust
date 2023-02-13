use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use crate::common::{AssetType, validate_object};
use crate::raw::base::{RawConfig, RawConfigTrait};
use crate::raw::validator::{is_ethereum_address, array_unique, is_number_string};

#[derive(Validate, Serialize, Deserialize, Debug, Clone, Eq)]
pub struct RawAssetConfig {
    #[serde(default)]
    pub base: RawConfig,
    #[serde(rename = "assetType")]
    pub asset_type: AssetType,
    #[validate(length(min = 1))]
    #[serde(rename = "assetSymbol")]
    pub asset_symbol: String,
    #[validate(range(min = 1))]
    #[serde(rename = "assetDecimals")]
    pub asset_decimals: u32,
    #[validate(custom = "is_ethereum_address")]
    #[serde(rename = "assetAddress")]
    pub asset_address: String,
    #[validate(
    custom(function = "array_unique"),
    custom(function = "is_number_string::<true, true>")
    )]
    #[serde(rename = "recommendedAmounts")]
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

impl RawConfigTrait for RawAssetConfig {
    fn validate(&self) {
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

#[cfg(test)]
mod tests {
    use crate::common::AssetType;
    use crate::raw::asset::RawAssetConfig;
    use crate::raw::base::{RawConfig, RawConfigTrait};

    fn default_config() -> RawAssetConfig {
        RawAssetConfig::new(
            AssetType::erc20,
            String::from("MTT"),
            16,
            "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string(),
            vec![
                String::from("10000000000000000"),
                String::from("100000000000000000"),
            ],
        )
    }

    #[test]
    #[should_panic]
    fn test_invalid_asset_symbol() {
        let mut config = default_config();
        config.asset_symbol = "".to_string();
        config.validate();
    }

    #[test]
    #[should_panic]
    fn test_invalid_asset_address_00() {
        let mut config = default_config();
        config.asset_address = String::from("");
        config.validate();
    }

    #[test]
    #[should_panic]
    fn test_invalid_asset_address_01() {
        let mut config = default_config();
        config.asset_address = String::from("0xdeadbeef");
        config.validate();
    }

    #[test]
    #[should_panic]
    fn test_invalid_recommended_amounts_00() {
        let mut config = default_config();
        config.recommended_amounts = vec![String::from("")];
        config.validate();
    }

    #[test]
    #[should_panic]
    fn test_invalid_recommended_amounts_01() {
        let mut config = default_config();
        config.recommended_amounts = vec![String::from("abcd")];
        config.validate();
    }

    #[test]
    #[should_panic]
    fn test_invalid_recommended_amounts_02() {
        let mut config = default_config();
        config.recommended_amounts = vec![String::from("1"), String::from("1")];
        config.validate();
    }

    #[tokio::test]
    async fn test_import_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawAssetConfig>("src/tests/files/asset.valid.json").await;
        assert_eq!(file_config, default_config());
    }

    #[tokio::test]
    #[should_panic]
    async fn test_import_invalid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawAssetConfig>("src/tests/files/asset.invalid.json").await;
    }
}