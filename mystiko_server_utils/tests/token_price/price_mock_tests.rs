extern crate mystiko_fs;

use ethers_core::types::U256;
use mockito::{Matcher, Mock, Server, ServerGuard};
use mystiko_fs::read_file_bytes;
use mystiko_server_utils::token_price::config::TokenPriceConfig;
use mystiko_server_utils::token_price::error::TokenPriceError;
use mystiko_server_utils::token_price::price::TokenPrice;
use mystiko_server_utils::token_price::query::{CurrencyMapResponse, CurrencyQuoteResponse};
use serde_json::json;

#[tokio::test]
async fn test_get_token_id() {
    let (server, mocks) = create_mock_token_price_server(Some(true), None).await;
    let mut default_cfg = TokenPriceConfig::new("testnet", None).unwrap();
    default_cfg.base_url = server.url().to_string();

    let tp = TokenPrice::new(&default_cfg, "").unwrap();
    let id = tp.get_token_id("ETH").await.unwrap();
    for mock in mocks {
        mock.assert_async().await;
    }
    assert_eq!(id, [1027]);
}

#[tokio::test]
async fn test_get_token_id_error() {
    let mut server = Server::new_async().await;

    let mut default_cfg = TokenPriceConfig::new("testnet", None).unwrap();
    default_cfg.base_url = server.url().to_string();

    let tp = TokenPrice::new(&default_cfg, "").unwrap();
    let mock = server
        .mock("GET", "/v1/cryptocurrency/map")
        .match_query(Matcher::Regex("symbol".into()))
        .with_status(401)
        .with_body("")
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let id = tp.get_token_id("ETH").await;
    mock.create_async().await;
    assert!(matches!(id.err().unwrap(), TokenPriceError::ReqwestError(_)));

    let currency_map = json!({
        "date": 10,
    });
    let resp_json = serde_json::to_string(&currency_map).unwrap();
    let mock = server
        .mock("GET", "/v1/cryptocurrency/map")
        .match_query(Matcher::Regex("symbol".into()))
        .with_status(200)
        .with_body(resp_json)
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let id = tp.get_token_id("ETH").await;
    mock.create_async().await;
    assert!(matches!(id.err().unwrap(), TokenPriceError::ReqwestError(_)));
}

#[tokio::test]
async fn test_get_token_id_error2() {
    let mut server = Server::new_async().await;
    let mut default_cfg = TokenPriceConfig::new("testnet", None).unwrap();
    default_cfg.base_url = server.url().to_string();
    let tp = TokenPrice::new(&default_cfg, "").unwrap();
    let id_bytes = read_file_bytes("./tests/token_price/files/token_ids_status_error.json")
        .await
        .unwrap();
    let currency_map: CurrencyMapResponse = serde_json::from_slice(&id_bytes).unwrap();
    let resp_json = serde_json::to_string(&currency_map).unwrap();
    let mock = server
        .mock("GET", "/v1/cryptocurrency/map")
        .match_query(Matcher::Regex("symbol".into()))
        .with_status(200)
        .with_body(resp_json)
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let id = tp.get_token_id("ETH").await;
    mock.create_async().await;
    assert!(matches!(id.err().unwrap(), TokenPriceError::ResponseError(_)));
}

#[tokio::test]
async fn test_price() {
    let (server, mocks) = create_mock_token_price_server(None, Some(true)).await;
    let mut default_cfg = TokenPriceConfig::new("testnet", None).unwrap();
    default_cfg.base_url = server.url().to_string();

    let mut tp = TokenPrice::new(&default_cfg, "").unwrap();
    let price = tp.price("ETH").await.unwrap();
    for mock in mocks {
        mock.assert_async().await;
    }
    assert!(price > 100.0);
}

#[tokio::test]
async fn test_price_error() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v2/cryptocurrency/quotes/latest")
        .match_query(Matcher::Regex("id".into()))
        .with_status(401)
        .with_body("")
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let mut default_cfg = TokenPriceConfig::new("testnet", None).unwrap();
    default_cfg.base_url = server.url().to_string();

    let mut tp = TokenPrice::new(&default_cfg, "").unwrap();
    let price = tp.price("ETH").await;
    mock.create_async().await;
    assert!(matches!(price.err().unwrap(), TokenPriceError::ReqwestError(_)));

    let currency_map = json!({
        "date": 10,
    });
    let resp_json = serde_json::to_string(&currency_map).unwrap();
    let mock = server
        .mock("GET", "/v2/cryptocurrency/quotes/latest")
        .match_query(Matcher::Regex("id".into()))
        .with_body(resp_json)
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let price = tp.price("ETH").await;
    mock.create_async().await;
    assert!(matches!(price.err().unwrap(), TokenPriceError::ReqwestError(_)));
}

#[tokio::test]
async fn test_price_error2() {
    let mut server = Server::new_async().await;
    let mut default_cfg = TokenPriceConfig::new("testnet", None).unwrap();
    default_cfg.base_url = server.url().to_string();
    let mut tp = TokenPrice::new(&default_cfg, "").unwrap();

    let id_bytes = read_file_bytes("./tests/token_price/files/token_price_status_error.json")
        .await
        .unwrap();
    let currency_quote: CurrencyQuoteResponse = serde_json::from_slice(&id_bytes).unwrap();
    let resp_json = serde_json::to_string(&currency_quote).unwrap();
    let mock = server
        .mock("GET", "/v2/cryptocurrency/quotes/latest")
        .match_query(Matcher::Regex("id".into()))
        .with_body(resp_json)
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let price = tp.price("ETH").await;
    mock.create_async().await;
    assert!(matches!(price.err().unwrap(), TokenPriceError::ResponseError(_)));
}

#[tokio::test]
async fn test_tokne_price_not_init_err() {
    let mut default_cfg = TokenPriceConfig::new("testnet", None).unwrap();
    default_cfg.base_url = "http://error.com".to_string();
    let mut tp = TokenPrice::new(&default_cfg, "").unwrap();
    let price = tp.price("ETH").await;
    assert!(matches!(price.err().unwrap(), TokenPriceError::ReqwestError(_)));

    let amount_a = U256::from(10000000);
    let amount_b = tp.swap("USDT", 6, amount_a, "USDT", 6).await;
    assert!(matches!(amount_b.err().unwrap(), TokenPriceError::ReqwestError(_)));
}

#[tokio::test]
async fn test_swap() {
    let (server, mocks) = create_mock_token_price_server(None, Some(true)).await;
    let mut default_cfg = TokenPriceConfig::new("testnet", None).unwrap();
    default_cfg.base_url = server.url().to_string();
    default_cfg.swap_precision = 8;

    let mut tp = TokenPrice::new(&default_cfg, "").unwrap();
    for i in 0..6 {
        let amount_a = U256::from(10000000) / U256::from(10_u64.pow(i));
        let amount_b = tp.swap("USDT", 6, amount_a, "USDT", 6).await.unwrap();
        assert_eq!(amount_b, amount_a);
    }

    let amount_a = U256::from(10);
    let amount_b = tp.swap("USDT", 6, amount_a, "USDT", 6).await.unwrap();
    assert_eq!(amount_b, U256::from(10));

    let amount = ethers_core::utils::parse_ether(10).unwrap();
    for i in 0..12 {
        let amount_a = amount / (U256::from(10_u64.pow(i)));
        let amount_b = tp.swap("ETH", 18, amount_a, "USDT", 6).await.unwrap();
        assert!(amount_b >= U256::from(0));
    }

    for mock in mocks {
        mock.assert_async().await;
    }
}

#[tokio::test]
async fn test_internal_error() {
    let (server, mocks) = create_mock_token_price_server(None, Some(true)).await;
    let mut default_cfg = TokenPriceConfig::new("testnet", None).unwrap();
    default_cfg.base_url = server.url().to_string();
    let mut tp = TokenPrice::new(&default_cfg, "test").unwrap();
    let price = tp.price("BTC").await;
    for mock in mocks {
        mock.assert_async().await;
    }
    assert!(matches!(price.err().unwrap(), TokenPriceError::TokenNotSupport));
    let price = tp.price("mMATIC").await;
    assert!(matches!(price.err().unwrap(), TokenPriceError::InternalError));
}

pub async fn create_mock_token_price_server(id: Option<bool>, price: Option<bool>) -> (ServerGuard, Vec<Mock>) {
    let mut server = Server::new_async().await;
    let mut mocks = vec![];
    if id.is_some() {
        let id_bytes = read_file_bytes("./tests/token_price/files/token_ids.json")
            .await
            .unwrap();
        let currency_map: CurrencyMapResponse = serde_json::from_slice(&id_bytes).unwrap();
        let resp_json = serde_json::to_string(&currency_map).unwrap();
        let mock = server
            .mock("GET", "/v1/cryptocurrency/map")
            .match_query(Matcher::Regex("symbol".into()))
            .with_status(200)
            .with_body(resp_json)
            .with_header("content-type", "application/json")
            .create_async()
            .await;
        mocks.push(mock);
    }

    if price.is_some() {
        let price_bytes = read_file_bytes("./../mystiko_server_utils/tests/token_price/files/token_price.json")
            .await
            .unwrap();
        let currency_quote: CurrencyQuoteResponse = serde_json::from_slice(&price_bytes).unwrap();
        let resp_json = serde_json::to_string(&currency_quote).unwrap();
        let mock = server
            .mock("GET", "/v2/cryptocurrency/quotes/latest")
            .match_query(Matcher::Regex("id".into()))
            .with_status(200)
            .with_body(resp_json)
            .with_header("content-type", "application/json")
            .create_async()
            .await;
        mocks.push(mock);
    }

    (server, mocks)
}
