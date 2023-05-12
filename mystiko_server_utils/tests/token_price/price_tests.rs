extern crate httptest;
extern crate mystiko_fs;

use ethers_core::types::U256;
use mystiko_server_utils::token_price::config::TokenPriceConfig;
use mystiko_server_utils::token_price::price::TokenPrice;

const coin_market_api_key: &str = "";

#[tokio::test]
async fn test_get_token_id() {
    let default_cfg = TokenPriceConfig::default();
    let tp = TokenPrice::new(default_cfg, coin_market_api_key).unwrap();
    let id = tp.get_token_id("ETH").await.unwrap();
    assert_eq!(id, [1027]);
}

#[tokio::test]
async fn test_price() {
    let default_cfg = TokenPriceConfig::default();
    let mut tp = TokenPrice::new(default_cfg, coin_market_api_key).unwrap();
    let price = tp.price("ETH").await.unwrap();
    assert!(price > 100.0);
}

#[tokio::test]
async fn test_swap() {
    let default_cfg = TokenPriceConfig::default();
    let mut tp = TokenPrice::new(default_cfg, coin_market_api_key).unwrap();

    let amount = tp
        .swap("ETH", 18, U256::from("1000000000000000000"), "USDT", 6)
        .await
        .unwrap();
    assert!(amount > U256::from("100000000"));
}
