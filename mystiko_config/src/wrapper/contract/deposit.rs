use crate::raw::contract::deposit::RawDepositContractConfig;
use crate::types::{AssetType, BridgeType, ContractType};
use crate::wrapper::asset::AssetConfig;
use crate::wrapper::circuit::CircuitConfig;
use crate::wrapper::contract::pool::PoolContractConfig;
use anyhow::Result;
use mystiko_utils::convert::decimal_to_number;
use num_bigint::BigInt;
use num_traits::{NumCast, Zero};
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, TypedBuilder)]
pub struct DepositContractConfig {
    raw: Arc<RawDepositContractConfig>,
    bridge_fee_asset_config: Arc<AssetConfig>,
    executor_fee_asset_config: Arc<AssetConfig>,
    pool_contract_config: Arc<PoolContractConfig>,
}

impl DepositContractConfig {
    pub fn new(
        raw: Arc<RawDepositContractConfig>,
        bridge_fee_asset_config: Arc<AssetConfig>,
        executor_fee_asset_config: Arc<AssetConfig>,
        pool_contract_config: Arc<PoolContractConfig>,
    ) -> Self {
        DepositContractConfig {
            raw,
            bridge_fee_asset_config,
            executor_fee_asset_config,
            pool_contract_config,
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

    pub fn bridge_type(&self) -> &BridgeType {
        &self.raw.bridge_type
    }

    pub fn pool_contract_address(&self) -> &str {
        &self.raw.pool_address
    }

    pub fn pool_contract(&self) -> &PoolContractConfig {
        &self.pool_contract_config
    }

    pub fn disabled(&self) -> bool {
        self.raw.disabled
    }

    pub fn min_amount(&self) -> Result<BigInt> {
        Ok(BigInt::from_str(&self.raw.min_amount)?)
    }

    pub fn min_amount_number<T>(&self) -> Result<T>
    where
        T: NumCast + Zero,
    {
        let asset_decimals = self.asset().asset_decimals();
        decimal_to_number::<T, String>(&self.raw.min_amount, Some(asset_decimals))
    }

    pub fn max_amount(&self) -> Result<BigInt> {
        Ok(BigInt::from_str(&self.raw.max_amount)?)
    }

    pub fn max_amount_number<T>(&self) -> Result<T>
    where
        T: NumCast + Zero,
    {
        let asset_decimals = self.asset().asset_decimals();
        decimal_to_number::<T, String>(&self.raw.max_amount, Some(asset_decimals))
    }

    pub fn min_bridge_fee(&self) -> Result<BigInt> {
        Ok(BigInt::from_str(&self.raw.min_bridge_fee)?)
    }

    pub fn min_bridge_fee_number<T>(&self) -> Result<T>
    where
        T: NumCast + Zero,
    {
        let asset_decimals = self.bridge_fee_asset().asset_decimals();
        decimal_to_number::<T, String>(&self.raw.min_bridge_fee, Some(asset_decimals))
    }

    pub fn min_executor_fee(&self) -> Result<BigInt> {
        Ok(BigInt::from_str(&self.raw.min_executor_fee)?)
    }

    pub fn min_executor_fee_number<T>(&self) -> Result<T>
    where
        T: NumCast + Zero,
    {
        let asset_decimals = self.executor_fee_asset().asset_decimals();
        decimal_to_number::<T, String>(&self.raw.min_executor_fee, Some(asset_decimals))
    }

    pub fn min_rollup_fee(&self) -> Result<BigInt> {
        self.pool_contract().min_rollup_fee()
    }

    pub fn min_rollup_fee_number<T>(&self) -> Result<T>
    where
        T: NumCast + Zero,
    {
        self.pool_contract().min_rollup_fee_number()
    }

    pub fn service_fee(&self) -> u32 {
        self.raw.service_fee
    }

    pub fn service_fee_divider(&self) -> u32 {
        self.raw.service_fee_divider
    }

    pub fn asset(&self) -> &AssetConfig {
        self.pool_contract().asset()
    }

    pub fn asset_address(&self) -> &Option<String> {
        self.pool_contract().asset_address()
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

    pub fn bridge_fee_asset(&self) -> &AssetConfig {
        &self.bridge_fee_asset_config
    }

    pub fn bridge_fee_asset_address(&self) -> &Option<String> {
        &self.raw.bridge_fee_asset_address
    }

    pub fn executor_fee_asset(&self) -> &AssetConfig {
        &self.executor_fee_asset_config
    }

    pub fn executor_fee_asset_address(&self) -> &Option<String> {
        &self.raw.executor_fee_asset_address
    }

    pub fn circuits(&self) -> Vec<&CircuitConfig> {
        self.pool_contract().circuits()
    }

    pub fn peer_chain_id(&self) -> &Option<u32> {
        &self.raw.peer_chain_id
    }

    pub fn peer_contract_address(&self) -> &Option<String> {
        &self.raw.peer_contract_address
    }
}
