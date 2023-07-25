use mehcode_config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[allow(unused)]
pub struct TokenPriceConfig {
    pub base_url: String,
    pub query_timeout_secs: u32,
    pub price_cache_ttl: u64,
    pub swap_precision: u32,
    pub coin_market_cap_ids: HashMap<String, u32>,
}

impl TokenPriceConfig {
    pub fn new(run_mod: &str, config_path: Option<&str>) -> anyhow::Result<Self> {
        let mut ids = HashMap::new();
        ids.insert("ETH".to_string(), 1027);
        ids.insert("mETH".to_string(), 1027);
        ids.insert("WETH".to_string(), 1027);
        ids.insert("BNB".to_string(), 1839);
        ids.insert("mBNB".to_string(), 1839);
        ids.insert("FTM".to_string(), 3513);
        ids.insert("mFTM".to_string(), 3513);
        ids.insert("MATIC".to_string(), 3890);
        ids.insert("mMATIC".to_string(), 3890);
        ids.insert("DEV".to_string(), 5990);
        ids.insert("mDEV".to_string(), 5990);
        ids.insert("AVAX".to_string(), 5805);
        ids.insert("mAVAX".to_string(), 5805);
        ids.insert("USDT".to_string(), 825);
        ids.insert("USDC".to_string(), 3408);
        ids.insert("BUSD".to_string(), 4687);
        ids.insert("MTT".to_string(), 1839);
        ids.insert("mUSD".to_string(), 1839);

        let mut s = Config::builder()
            .set_default("base_url", "https://pro-api.coinmarketcap.com")?
            .set_default("query_timeout_secs", 10)?
            .set_default("swap_precision", 3)?
            .set_default("coin_market_cap_ids", ids)?;

        if run_mod == "testnet" {
            s = s.set_default("price_cache_ttl", 72000)?;
        } else {
            s = s.set_default("price_cache_ttl", 1800)?;
        }

        if let Some(path) = config_path {
            let run_config_path = format!("{}/token_price.json", path);
            if Path::exists(Path::new(&run_config_path)) {
                s = s.add_source(File::with_name(&run_config_path));
            }
        }

        let cfg = s
            .add_source(Environment::with_prefix("MYSTIKO_TOKEN_PRICE").separator("."))
            .build()?;
        Ok(cfg.try_deserialize()?)
    }

    pub fn tokens(&self) -> Vec<String> {
        Vec::from_iter(self.coin_market_cap_ids.keys().cloned())
    }

    pub fn ids(&self) -> Vec<u32> {
        let set: HashSet<u32> = self.coin_market_cap_ids.values().cloned().collect();
        set.into_iter().collect()
    }
}
