use crate::RawAssetConfig;
use anyhow::Result;
use mystiko_types::AssetType;
use mystiko_utils::convert::decimal_to_number;
use num_bigint::BigUint;
use num_traits::{NumCast, Zero};
use std::str::FromStr;
use std::sync::Arc;
use validator::Validate;

pub const MAIN_ASSET_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

#[derive(Clone, Debug)]
pub struct AssetConfig {
    raw: Arc<RawAssetConfig>,
}

impl AssetConfig {
    pub fn new(raw: Arc<RawAssetConfig>) -> Self {
        AssetConfig { raw }
    }

    pub fn asset_address(&self) -> &str {
        &self.raw.asset_address
    }

    pub fn asset_type(&self) -> &AssetType {
        &self.raw.asset_type
    }

    pub fn asset_symbol(&self) -> &str {
        &self.raw.asset_symbol
    }

    pub fn asset_decimals(&self) -> u32 {
        self.raw.asset_decimals
    }

    pub fn recommended_amounts(&self) -> Result<Vec<BigUint>> {
        let mut amounts: Vec<BigUint> = Vec::new();
        for amount_str in &self.raw.recommended_amounts {
            amounts.push(BigUint::from_str(amount_str)?);
        }
        Ok(amounts)
    }

    pub fn recommended_amounts_number<T>(&self) -> Result<Vec<T>>
    where
        T: NumCast + Zero,
    {
        let mut amounts: Vec<T> = Vec::new();
        for amount_str in &self.raw.recommended_amounts {
            amounts.push(decimal_to_number::<T, String>(amount_str, Some(self.asset_decimals()))?);
        }
        Ok(amounts)
    }

    #[cfg(feature = "proto")]
    pub fn to_proto(&self) -> Result<mystiko_protos::config::v1::AssetConfig> {
        let config = mystiko_protos::config::v1::AssetConfig::builder()
            .asset_type(Into::<mystiko_protos::common::v1::AssetType>::into(self.asset_type()))
            .asset_symbol(self.asset_symbol().to_string())
            .asset_decimals(self.asset_decimals())
            .asset_address(Some(self.asset_address().to_string()))
            .recommended_amounts(
                self.recommended_amounts()?
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>(),
            )
            .build();
        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        Ok(self.raw.validate()?)
    }
}
