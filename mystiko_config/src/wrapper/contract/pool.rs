use crate::raw::contract::pool::RawPoolContractConfig;
use crate::types::{AssetType, BridgeType, CircuitType, ContractType};
use crate::wrapper::asset::AssetConfig;
use crate::wrapper::circuit::CircuitConfig;
use anyhow::{Error, Result};
use mystiko_utils::convert::decimal_to_number;
use num_bigint::BigInt;
use num_traits::{NumCast, Zero};
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(Clone, Debug, TypedBuilder)]
pub struct PoolContractConfig {
    raw: Arc<RawPoolContractConfig>,
    main_asset_config: Arc<AssetConfig>,
    asset_config: Arc<AssetConfig>,
    circuit_configs: Vec<Arc<CircuitConfig>>,
}

impl PoolContractConfig {
    pub fn new(
        raw: Arc<RawPoolContractConfig>,
        main_asset_config: Arc<AssetConfig>,
        asset_config: Arc<AssetConfig>,
        circuit_configs: Vec<Arc<CircuitConfig>>,
    ) -> Self {
        PoolContractConfig {
            raw,
            main_asset_config,
            asset_config,
            circuit_configs,
        }
    }

    pub fn version(&self) -> u32 {
        self.raw.version
    }

    pub fn name(&self) -> &str {
        &self.raw.name
    }

    pub fn address(&self) -> &str {
        &self.raw.address
    }

    pub fn contract_type(&self) -> &ContractType {
        &self.raw.contract_type
    }

    pub fn start_block(&self) -> u64 {
        self.raw.start_block
    }

    pub fn event_filter_size(&self) -> &Option<u64> {
        &self.raw.event_filter_size
    }

    pub fn indexer_filter_size(&self) -> &Option<u64> {
        &self.raw.indexer_filter_size
    }

    pub fn pool_name(&self) -> &str {
        &self.raw.pool_name
    }

    pub fn bridge_type(&self) -> &BridgeType {
        &self.raw.bridge_type
    }

    pub fn asset(&self) -> &AssetConfig {
        if self.asset_address().is_some() {
            &self.asset_config
        } else {
            &self.main_asset_config
        }
    }

    pub fn asset_address(&self) -> &Option<String> {
        &self.raw.asset_address
    }

    pub fn asset_type(&self) -> &AssetType {
        self.asset().asset_type()
    }

    pub fn asset_symbol(&self) -> &str {
        self.asset().asset_symbol()
    }

    pub fn asset_decimals(&self) -> u32 {
        self.asset().asset_decimals()
    }

    pub fn recommended_amounts(&self) -> Result<Vec<BigInt>> {
        self.asset().recommended_amounts()
    }

    pub fn recommended_amounts_number<T>(&self) -> Result<Vec<T>>
    where
        T: NumCast + Zero,
    {
        self.asset().recommended_amounts_number()
    }

    pub fn min_rollup_fee(&self) -> Result<BigInt> {
        Ok(BigInt::from_str(&self.raw.min_rollup_fee)?)
    }

    pub fn min_rollup_fee_number<T>(&self) -> Result<T>
    where
        T: NumCast + Zero,
    {
        let decimals = self.asset().asset_decimals();
        decimal_to_number::<T, String>(&self.raw.min_rollup_fee, Some(decimals))
    }

    pub fn circuits_names(&self) -> &Vec<String> {
        &self.raw.circuits
    }

    pub fn circuits(&self) -> Vec<&CircuitConfig> {
        self.circuit_configs.iter().map(|c| c.as_ref()).collect()
    }

    pub fn circuit_by_type(&self, circuit_type: &CircuitType) -> Option<&CircuitConfig> {
        self.circuits()
            .into_iter()
            .find(|c| c.circuit_type() == circuit_type)
    }

    pub fn validate(&self) -> Result<()> {
        self.raw.validate()?;
        if self.asset_address().is_some() && self.asset_type() == &AssetType::Main {
            return Err(Error::msg(format!(
                "pool contract {} asset_address should be None when asset_type is MAIN",
                self.address()
            )));
        }
        if self.asset_address().is_none() && self.asset_type() != &AssetType::Main {
            return Err(Error::msg(format!(
                "pool contract {} asset_address should NOT be None when asset_type is not MAIN",
                self.address()
            )));
        }
        Ok(())
    }
}
