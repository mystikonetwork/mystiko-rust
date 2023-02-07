use std::str::FromStr;
use num_bigint::BigInt;
use mystiko_utils::check::check;
use crate::common::AssetType;
use crate::raw::asset::RawAssetConfig;
use crate::wrapper::base::BaseConfig;

const MAIN_ASSET_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

#[derive(Clone, Debug)]
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

    pub fn asset_symbol(&self) -> &str {
        &self.base.data.asset_symbol
    }

    pub fn asset_decimals(&self) -> &u32 {
        &self.base.data.asset_decimals
    }

    // TODO check this method
    pub fn recommended_amounts(&self) -> Vec<BigInt> {
        let mut amounts: Vec<BigInt> = vec![];
        for amount in &self.base.data.recommended_amounts {
            let bn = BigInt::from_str(amount).unwrap();
            amounts.push(bn);
        }
        amounts
    }

    // TODO Complete this method
    pub fn recommended_amounts_number(&self) -> Vec<u32> {
        let mut amounts_number: Vec<u32> = vec![];
        amounts_number
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
                "wrong asset address={:?} and type={:?}", self.asset_address(), self.asset_type()
            ).as_str(),
        );
    }
}