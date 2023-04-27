use crate::tx_manager::error::TxManagerError;
use ethers_core::types::{U256, U64};
use mystiko_fs::read_file_bytes;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder, Serialize, Deserialize, Eq, PartialEq)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct TxManagerConfig {
    #[serde(default = "default_max_gas_price")]
    pub max_gas_price: U256,
    #[serde(default = "default_max_priority_fee_per_gas")]
    pub max_priority_fee_per_gas: U256,
    #[serde(default = "default_max_confirm_count")]
    pub max_confirm_count: u32,
    #[serde(default = "default_confirm_interval_secs")]
    pub confirm_interval_secs: u64,
    #[serde(default = "default_force_gas_price_chains")]
    pub force_gas_price_chains: Vec<U64>,
}

impl Default for TxManagerConfig {
    fn default() -> Self {
        Self {
            max_gas_price: default_max_gas_price(),
            max_priority_fee_per_gas: default_max_priority_fee_per_gas(),
            max_confirm_count: default_max_confirm_count(),
            confirm_interval_secs: default_confirm_interval_secs(),
            force_gas_price_chains: default_force_gas_price_chains(),
        }
    }
}

fn default_max_gas_price() -> U256 {
    U256::from(100_000_000_000u64) // 100 Gwei
}

fn default_max_priority_fee_per_gas() -> U256 {
    U256::from(1_500_000_000u64) // 1.5 Gwei
}

fn default_max_confirm_count() -> u32 {
    100
}

fn default_confirm_interval_secs() -> u64 {
    5
}

fn default_force_gas_price_chains() -> Vec<U64> {
    vec![
        U64::from(250),
        U64::from(4002),
        U64::from(137),
        U64::from(80001),
    ]
}

pub async fn read_config_from_file(file_path_str: &str) -> Result<TxManagerConfig, TxManagerError> {
    let file = read_file_bytes(file_path_str)
        .await
        .map_err(|why| TxManagerError::FileError(why.to_string()))?;
    let config: TxManagerConfig = serde_json::from_slice(&file)?;
    Ok(config)
}
