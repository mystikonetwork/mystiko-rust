use crate::token_price::config::TokenPriceConfig;
use crate::token_price::error::TokenPriceError;
use crate::token_price::query::QueryApiInstance;
use crate::token_price::utils::{calc_token_precision, f64_to_u256, u256_to_f64};
use anyhow::Result;
use dotenv::dotenv;
use ethers_core::types::U256;
use std::collections::HashMap;
use std::env;
use std::ops::{Div, Mul};
use std::time::{Duration, Instant};

pub struct TokenPrice {
    config: TokenPriceConfig,
    instance: QueryApiInstance,
    record_time: Instant,
    ids: Vec<u32>,
    // key token id
    prices: HashMap<u32, f64>,
}

fn instant_init(duration: u64) -> Instant {
    Instant::now() - Duration::from_secs(duration + 1)

    // let now = Instant::now();
    // now.checked_sub(Duration::from_secs(duration * 2))
    //     .expect("overflow when subtracting duration from instant")
}

fn load_api_key_from_env() -> Result<String, TokenPriceError> {
    dotenv().ok();
    let key = "COIN_MARKET_CAP_API_KEY";
    match env::var(key) {
        Ok(value) => Ok(value),
        Err(_) => Err(TokenPriceError::ApiKeyNotConfigure),
    }
}

impl TokenPrice {
    pub fn new(cfg: TokenPriceConfig) -> Result<Self, TokenPriceError> {
        let api_key = load_api_key_from_env()?;
        let instance =
            QueryApiInstance::new(&api_key, cfg.base_url.clone(), cfg.query_timeout_secs)?;
        Ok(TokenPrice {
            ids: cfg.ids(),
            record_time: instant_init(cfg.price_cache_ttl),
            config: cfg,
            instance,
            prices: HashMap::new(),
        })
    }

    pub async fn price(&mut self, symbol: &str) -> Result<f64, TokenPriceError> {
        self.update_token_prices().await?;
        self.get_token_price(symbol)
    }

    pub async fn swap(
        &mut self,
        asset_a: &str,
        decimal_a: u32,
        amount_a: U256,
        asset_b: &str,
        decimal_b: u32,
    ) -> Result<U256, TokenPriceError> {
        self.update_token_prices().await?;

        let price_a = self.get_token_price(asset_a)?;
        let price_b = self.get_token_price(asset_b)?;
        let mut amount = u256_to_f64(amount_a, decimal_a);
        amount = amount.mul(price_a).div(price_b);
        let mut amount = f64_to_u256(amount, decimal_b);
        let token_precision = calc_token_precision(price_b, decimal_b, self.config.swap_precision);
        amount /= token_precision;
        amount *= token_precision;

        Ok(amount)
    }

    pub async fn get_token_id(&self, symbol: &str) -> Result<Vec<u32>, TokenPriceError> {
        self.instance.get_token_id(symbol).await
    }

    async fn update_token_prices(&mut self) -> Result<(), TokenPriceError> {
        let instant_now = Instant::now();
        let current = instant_now.duration_since(self.record_time).as_secs();
        if current < self.config.price_cache_ttl {
            return Ok(());
        }
        let token_prices = self.instance.get_latest_price(&self.ids).await?;
        for (key, price) in token_prices {
            self.prices.insert(key, price);
        }

        self.record_time = instant_now;

        Ok(())
    }

    fn get_token_price(&self, symbol: &str) -> Result<f64, TokenPriceError> {
        self.config
            .coin_market_cap_ids
            .get(symbol)
            .ok_or(TokenPriceError::TokenNotSupport)
            .and_then(|id| {
                self.prices
                    .get(id)
                    .copied()
                    .ok_or(TokenPriceError::InternalError)
            })
    }
}
