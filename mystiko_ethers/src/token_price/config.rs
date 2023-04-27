use crate::token_price::error::TokenPriceError;
use mystiko_fs::read_file_bytes;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TokenPriceConfig {
    #[serde(default = "default_base_url")]
    pub base_url: String,
    #[serde(default = "default_query_timeout_secs")]
    pub query_timeout_secs: u32,
    #[serde(default = "default_price_cache_ttl")]
    pub price_cache_ttl: u64,
    #[serde(default = "default_swap_precision")]
    pub swap_precision: u32,
    #[serde(default = "default_market_cap_ids")]
    pub coin_market_cap_ids: HashMap<String, u32>,
}

impl Default for TokenPriceConfig {
    fn default() -> Self {
        Self {
            base_url: default_base_url(),
            query_timeout_secs: default_query_timeout_secs(),
            price_cache_ttl: default_price_cache_ttl(),
            swap_precision: default_swap_precision(),
            coin_market_cap_ids: default_market_cap_ids(),
        }
    }
}

impl TokenPriceConfig {
    pub fn tokens(&self) -> Vec<String> {
        Vec::from_iter(self.coin_market_cap_ids.keys().cloned())
    }

    pub fn ids(&self) -> Vec<u32> {
        let set: HashSet<u32> = self.coin_market_cap_ids.values().cloned().collect();
        set.into_iter().collect()
    }
}

fn default_base_url() -> String {
    "https://pro-api.coinmarketcap.com".to_string()
}

fn default_query_timeout_secs() -> u32 {
    5
}

fn default_price_cache_ttl() -> u64 {
    1800
}

fn default_swap_precision() -> u32 {
    3
}

fn default_market_cap_ids() -> HashMap<String, u32> {
    let mut coin_market_cap_ids = HashMap::new();

    let tokens = vec![
        ("ETH", 1027),
        ("mETH", 1027),
        ("BNB", 1839),
        ("mBNB", 1839),
        ("USDT", 825),
        ("USDC", 3408),
        ("BUSD", 4687),
        ("MTT", 3408),
        ("mUSD", 3408),
        ("FTM", 3513),
        ("mFTM", 3513),
        ("MATIC", 3890),
        ("mMATIC", 3890),
        ("DEV", 5990),
        ("mDEV", 5990),
        ("AVAX", 5805),
        ("mAVAX", 5805),
    ];

    for (symbol, id) in tokens {
        coin_market_cap_ids.insert(symbol.to_string(), id);
    }

    coin_market_cap_ids
}

pub async fn read_config_from_file(
    file_path_str: &str,
) -> Result<TokenPriceConfig, TokenPriceError> {
    let file = read_file_bytes(file_path_str)
        .await
        .map_err(|why| TokenPriceError::FileError(why.to_string()))?;
    let config: TokenPriceConfig = serde_json::from_slice(&file)?;
    Ok(config)
}
