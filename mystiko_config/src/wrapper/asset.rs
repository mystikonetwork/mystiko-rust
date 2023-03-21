use crate::common::AssetType;
use crate::raw::asset::RawAssetConfig;
use crate::wrapper::base::BaseConfig;
use anyhow::ensure;
use mystiko_utils::convert::from_decimals;
use num_bigint::BigInt;
use std::str::FromStr;

pub const MAIN_ASSET_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AssetConfig {
    base: BaseConfig<RawAssetConfig>,
}

impl AssetConfig {
    pub fn new(raw: RawAssetConfig) -> Result<Self, anyhow::Error> {
        let config = Self {
            base: BaseConfig::builder().data(raw).build(),
        };
        config.validate()?;
        Ok(config)
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
        self.base
            .data
            .recommended_amounts
            .iter()
            .map(|amount| from_decimals(amount, Some(self.asset_decimals())))
            .collect()
    }

    pub fn mutate(&self, data: Option<RawAssetConfig>) -> Result<Self, anyhow::Error> {
        match data {
            None => AssetConfig::new(self.data().clone()),
            Some(value) => AssetConfig::new(value),
        }
    }

    pub fn data(&self) -> &RawAssetConfig {
        &self.base.data
    }

    pub fn copy_data(&self) -> RawAssetConfig {
        self.base.copy_data()
    }

    pub fn to_json_string(&self) -> anyhow::Result<String> {
        self.base.to_json_string()
    }

    fn validate(&self) -> Result<(), anyhow::Error> {
        ensure!(
            (self.asset_type().clone() != AssetType::Main
                && self.asset_address() != MAIN_ASSET_ADDRESS)
                || (self.asset_type().clone() == AssetType::Main
                    && self.asset_address() == MAIN_ASSET_ADDRESS),
            "wrong asset address={} and type={:?}",
            self.asset_address(),
            self.asset_type()
        );
        Ok(())
    }
}
