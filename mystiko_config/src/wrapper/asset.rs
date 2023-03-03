use std::str::FromStr;
use num_bigint::BigInt;
use mystiko_utils::check::check;
use mystiko_utils::convert::from_decimals;
use crate::common::AssetType;
use crate::raw::asset::RawAssetConfig;
use crate::wrapper::base::BaseConfig;

pub const MAIN_ASSET_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

#[derive(Clone, Debug, PartialEq)]
pub struct AssetConfig {
    base: BaseConfig<RawAssetConfig>,
}

impl AssetConfig {
    pub fn new(raw: RawAssetConfig) -> Self {
        let config = Self { base: BaseConfig::new(raw, None) };
        config.validate();
        config
    }

    pub fn asset_address(&self) -> &str {
        &self.base.data.asset_address
    }

    pub fn asset_type(&self) -> &AssetType {
        &self.base.data.asset_type
    }

    pub fn asset_symbol(&self) -> String {
        self.base.data.asset_symbol.clone()
    }

    pub fn asset_decimals(&self) -> u32 {
        self.base.data.asset_decimals
    }

    pub fn recommended_amounts(&self) -> Vec<BigInt> {
        let mut amounts: Vec<BigInt> = vec![];
        for amount in &self.base.data.recommended_amounts {
            let bn = BigInt::from_str(amount).unwrap();
            amounts.push(bn);
        }
        amounts
    }

    pub fn recommended_amounts_number(&self) -> Vec<f64> {
        self.base.data.recommended_amounts.iter().map(
            |amount| from_decimals(amount, Some(self.asset_decimals()))
        ).collect()
    }

    pub fn mutate(&self, data: Option<RawAssetConfig>) -> Self {
        match data {
            None => {
                AssetConfig::new(self.base.data.clone())
            }
            Some(value) => {
                AssetConfig::new(value)
            }
        }
    }

    fn validate(&self) {
        check(
            (self.asset_type().clone() != AssetType::Main && self.asset_address() != MAIN_ASSET_ADDRESS) ||
                (self.asset_type().clone() == AssetType::Main && self.asset_address() == MAIN_ASSET_ADDRESS),
            format!(
                "wrong asset address={} and type={:?}", self.asset_address(), self.asset_type()
            ).as_str(),
        );
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::common::AssetType;
    use crate::raw::asset::RawAssetConfig;
    use crate::raw::base::RawConfig;
    use crate::wrapper::asset;
    use crate::wrapper::asset::{AssetConfig, MAIN_ASSET_ADDRESS};

    async fn default_config() -> (RawAssetConfig, AssetConfig) {
        let raw_config =
            RawConfig::create_from_file::<RawAssetConfig>("src/tests/files/asset.valid.json").await;
        let config = AssetConfig::new(raw_config.clone());
        (raw_config, config)
    }

    #[tokio::test]
    async fn test_equality() {
        let (raw_config, config) = default_config().await;
        assert_eq!(config.asset_address(), raw_config.asset_address);
        assert_eq!(config.asset_type(), &raw_config.asset_type);
        assert_eq!(config.asset_decimals(), raw_config.asset_decimals);
        assert_eq!(config.asset_symbol(), raw_config.asset_symbol);
        let recommended_amounts: Vec<String> =
            config.recommended_amounts().iter().map(|a| a.to_string()).collect();
        assert_eq!(recommended_amounts, raw_config.recommended_amounts);
        assert_eq!(config.recommended_amounts_number(), vec![1f64, 10f64]);
    }

    #[tokio::test]
    #[should_panic(expected = "wrong asset address=0x0000000000000000000000000000000000000000 and type=Erc20")]
    async fn test_wrong_address_or_type_0() {
        let (mut raw_config, _) = default_config().await;
        raw_config.asset_address = MAIN_ASSET_ADDRESS.to_string();
        raw_config.asset_type = AssetType::Erc20;
        AssetConfig::new(raw_config);
    }

    #[tokio::test]
    #[should_panic(expected = "wrong asset address=0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a and type=Main")]
    async fn test_wrong_address_or_type_1() {
        let (mut raw_config, _) = default_config().await;
        raw_config.asset_address = String::from("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a");
        raw_config.asset_type = AssetType::Main;
        AssetConfig::new(raw_config);
    }

    #[tokio::test]
    async fn test_copy() {
        let (_, config) = default_config().await;
        assert_eq!(
            AssetConfig::new(config.base.copy_data()),
            config
        );
    }

    #[tokio::test]
    async fn test_mutate() {
        let (mut raw_config, config) = default_config().await;
        assert_eq!(config.mutate(None), config);
        raw_config.asset_decimals = 6;
        let new_config = config.mutate(Some(raw_config));
        assert_eq!(new_config.asset_decimals(), 6);
    }

    #[tokio::test]
    async fn test_to_json_string() {
        let (raw_config, config) = default_config().await;
        let json_string = config.base.to_json_string();
        let loaded_raw_config =
            RawConfig::create_from_json_string::<RawAssetConfig>(json_string.as_str()).await;
        assert_eq!(loaded_raw_config, raw_config);
    }
}