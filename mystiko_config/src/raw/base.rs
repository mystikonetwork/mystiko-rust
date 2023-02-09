use std::fmt::Debug;
use std::fs::{File, read_to_string};
use std::io::Read;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::{from_str, json};
use validator::Validate;
use crate::common::validate_object;

pub trait RawConfigTrait {
    fn validate(&self) -> Result<(), Vec<String>>;
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct RawConfig;

impl RawConfig {
    pub fn validate_object<T>(&self, object: T) -> Result<(), Vec<String>> where
        T: Validate + Debug
    {
        let result = validate_object(object);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        Ok(())
    }

    pub async fn create_from_object<T>(plain: T) -> T
        where T: DeserializeOwned + Serialize + RawConfigTrait
    {
        let result = plain.validate();
        if result.is_err() {
            panic!(
                "failed to validate config object:{:?}",
                result.unwrap_err()
            )
        }
        plain
    }

    pub async fn create_from_file<T>(json_file: &str) -> T
        where T: DeserializeOwned + Serialize + RawConfigTrait
    {
        let mut file = File::open(json_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let object: T = from_str(&contents).unwrap();
        RawConfig::create_from_object::<T>(object).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::AssetType;
    use crate::raw::asset::RawAssetConfig;

    #[tokio::test]
    async fn test_create_from_object() {
        let config = RawConfig::create_from_object::<RawAssetConfig>(RawAssetConfig {
            base: RawConfig,
            asset_type: AssetType::ERC20,
            asset_symbol: "MTT".to_string(),
            asset_decimals: 18,
            asset_address: "0x0000000000".to_string(),
            recommended_amounts: vec![],
        }).await;
    }

    #[tokio::test]
    async fn test_create_from_file() {
        let config = RawConfig::create_from_file
            ::<RawAssetConfig>("./src/raw/test/test.json").await;
    }
}
