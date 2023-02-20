use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use crate::common::{AssetType, validate_object};
use crate::raw::base::{RawConfig, Validator};
use crate::raw::validator::{is_ethereum_address, array_unique, is_number_string};

#[derive(Validate, Serialize, Deserialize, Debug, Clone, Eq)]
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

#[cfg(test)]
mod tests {
    use crate::common::AssetType;
    use crate::raw::asset::RawAssetConfig;
    use crate::raw::base::{RawConfig, Validator};

    async fn default_config() -> RawAssetConfig {
        RawConfig::create_from_object::<RawAssetConfig>(
            RawAssetConfig::new(
                AssetType::Erc20,
                String::from("MTT"),
                16,
                "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string(),
                vec![
                    String::from("10000000000000000"),
                    String::from("100000000000000000"),
                ],
            )
        ).await
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_asset_symbol() {
        let mut config = default_config().await;
        config.asset_symbol = "".to_string();
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_asset_address_0() {
        let mut config = default_config().await;
        config.asset_address = String::from("");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_asset_address_1() {
        let mut config = default_config().await;
        config.asset_address = String::from("0xdeadbeef");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_recommended_amounts_0() {
        let mut config = default_config().await;
        config.recommended_amounts = vec![String::from("")];
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_recommended_amounts_1() {
        let mut config = default_config().await;
        config.recommended_amounts = vec![String::from("abcd")];
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_recommended_amounts_2() {
        let mut config = default_config().await;
        config.recommended_amounts = vec![String::from("1"), String::from("1")];
        config.validation();
    }

    #[tokio::test]
    async fn test_import_valid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawAssetConfig>("src/tests/files/asset.valid.json").await;
        assert_eq!(file_config, default_config().await);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_import_invalid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawAssetConfig>("src/tests/files/asset.invalid.json").await;
    }
}