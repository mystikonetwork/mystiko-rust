use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::raw::asset::RawAssetConfig;
use crate::raw::base::RawConfig;
use crate::raw::contract::deposit::RawDepositContractConfig;
use crate::raw::contract::pool::RawPoolContractConfig;
use crate::raw::provider::RawProviderConfig;

pub const EXPLORER_TX_PLACEHOLDER: &str = "%tx%";

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawChainConfig {
    pub chain_id: u32,
    pub name: String,
    pub asset_symbol: String,
    pub asset_decimals: u32,
    pub recommended_amounts: Vec<String>,
    pub explorer_url: String,
    pub explorer_prefix: String,
    pub providers: Vec<RawProviderConfig>,
    pub signer_endpoint: String,
    pub event_filter_size: u32,
    pub indexer_filter_size: u32,
    pub deposit_contracts: Vec<RawDepositContractConfig>,
    pub pool_contracts: Vec<RawPoolContractConfig>,
    pub assets: Vec<RawAssetConfig>,
}

impl RawConfig for RawChainConfig {}