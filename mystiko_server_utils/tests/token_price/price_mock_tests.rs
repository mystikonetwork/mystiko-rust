extern crate httptest;
extern crate mystiko_fs;

use ethers_core::types::U256;
use httptest::{matchers::*, responders::*, Expectation, Server};
use mystiko_fs::read_file_bytes;
use mystiko_server_utils::token_price::config::TokenPriceConfig;
use mystiko_server_utils::token_price::error::TokenPriceError;
use mystiko_server_utils::token_price::price::TokenPrice;
use mystiko_server_utils::token_price::query::{CurrencyMapResponse, CurrencyQuoteResponse};
use serde_json::json;

#[tokio::test]
async fn test_get_token_id() {
    let id_bytes = read_file_bytes("./tests/token_price/files/token_ids.json")
        .await
        .unwrap();
    let currency_map: CurrencyMapResponse = serde_json::from_slice(&id_bytes).unwrap();
    let resp_json = json_encoded(currency_map);
    let server = Server::run();
    server.expect(Expectation::matching(request::method_path("GET", "/v1/cryptocurrency/map")).respond_with(resp_json));

    let mut default_cfg = TokenPriceConfig::new("testnet", None).unwrap();
    let url = server.url("").to_string();
    default_cfg.base_url = url.split_at(url.len() - 1).0.to_string();

    let tp = TokenPrice::new(&default_cfg, "").unwrap();
    let id = tp.get_token_id("ETH").await.unwrap();
    assert_eq!(id, [1027]);
}

#[tokio::test]
async fn test_get_token_id_error() {
    let server = Server::run();
    server.expect(
        Expectation::matching(request::method_path("GET", "/v1/cryptocurrency/map")).respond_with(status_code(401)),
    );
    let mut default_cfg = TokenPriceConfig::new("testnet", None).unwrap();
    let url = server.url("").to_string();
    default_cfg.base_url = url.split_at(url.len() - 1).0.to_string();

    let tp = TokenPrice::new(&default_cfg, "").unwrap();
    let id = tp.get_token_id("ETH").await;
    assert!(matches!(id.err().unwrap(), TokenPriceError::ReqwestError(_)));

    let currency_map = json!({
        "date": 10,
    });
    let resp_json = json_encoded(currency_map);
    server.expect(Expectation::matching(request::method_path("GET", "/v1/cryptocurrency/map")).respond_with(resp_json));
    let id = tp.get_token_id("ETH").await;
    assert!(matches!(id.err().unwrap(), TokenPriceError::ReqwestError(_)));

    let id_bytes = read_file_bytes("./tests/token_price/files/token_ids_status_error.json")
        .await
        .unwrap();
    let currency_map: CurrencyMapResponse = serde_json::from_slice(&id_bytes).unwrap();
    let resp_json = json_encoded(currency_map);
    server.expect(Expectation::matching(request::method_path("GET", "/v1/cryptocurrency/map")).respond_with(resp_json));
    let id = tp.get_token_id("ETH").await;
    assert!(matches!(id.err().unwrap(), TokenPriceError::ResponseError(_)));
}

#[tokio::test]
async fn test_price() {
    let id_bytes = read_file_bytes("./tests/token_price/files/token_price.json")
        .await
        .unwrap();
    let currency_quote: CurrencyQuoteResponse = serde_json::from_slice(&id_bytes).unwrap();

    let resp_json = json_encoded(currency_quote);
    let server = Server::run();
    server.expect(
        Expectation::matching(request::method_path("GET", "/v2/cryptocurrency/quotes/latest")).respond_with(resp_json),
    );

    let mut default_cfg = TokenPriceConfig::new("testnet", None).unwrap();
    let url = server.url("").to_string();
    default_cfg.base_url = url.split_at(url.len() - 1).0.to_string();

    let mut tp = TokenPrice::new(&default_cfg, "").unwrap();
    let price = tp.price("ETH").await.unwrap();
    assert!(price > 100.0);
}

#[tokio::test]
async fn test_price_error() {
    let server = Server::run();
    server.expect(
        Expectation::matching(request::method_path("GET", "/v2/cryptocurrency/quotes/latest"))
            .respond_with(status_code(401)),
    );

    let mut default_cfg = TokenPriceConfig::new("testnet", None).unwrap();
    let url = server.url("").to_string();
    default_cfg.base_url = url.split_at(url.len() - 1).0.to_string();

    let mut tp = TokenPrice::new(&default_cfg, "").unwrap();
    let price = tp.price("ETH").await;
    assert!(matches!(price.err().unwrap(), TokenPriceError::ReqwestError(_)));

    let currency_map = json!({
        "date": 10,
    });
    let resp_json = json_encoded(currency_map);
    server.expect(
        Expectation::matching(request::method_path("GET", "/v2/cryptocurrency/quotes/latest")).respond_with(resp_json),
    );
    let price = tp.price("ETH").await;
    assert!(matches!(price.err().unwrap(), TokenPriceError::ReqwestError(_)));

    let id_bytes = read_file_bytes("./tests/token_price/files/token_price_status_error.json")
        .await
        .unwrap();
    let currency_quote: CurrencyQuoteResponse = serde_json::from_slice(&id_bytes).unwrap();
    let resp_json = json_encoded(currency_quote);
    server.expect(
        Expectation::matching(request::method_path("GET", "/v2/cryptocurrency/quotes/latest")).respond_with(resp_json),
    );
    let price = tp.price("ETH").await;
    assert!(matches!(price.err().unwrap(), TokenPriceError::ResponseError(_)));
}

#[tokio::test]
async fn test_swap() {
    let id_bytes = read_file_bytes("./tests/token_price/files/token_price.json")
        .await
        .unwrap();
    let currency_quote: CurrencyQuoteResponse = serde_json::from_slice(&id_bytes).unwrap();

    let resp_json = json_encoded(currency_quote);
    let server = Server::run();
    server.expect(
        Expectation::matching(request::method_path("GET", "/v2/cryptocurrency/quotes/latest")).respond_with(resp_json),
    );

    let mut default_cfg = TokenPriceConfig::new("testnet", None).unwrap();
    let url = server.url("").to_string();
    default_cfg.base_url = url.split_at(url.len() - 1).0.to_string();

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
}

#[tokio::test]
async fn test_internal_error() {
    let id_bytes = read_file_bytes("./tests/token_price/files/token_price.json")
        .await
        .unwrap();
    let currency_quote: CurrencyQuoteResponse = serde_json::from_slice(&id_bytes).unwrap();

    let resp_json = json_encoded(currency_quote);
    let server = Server::run();
    server.expect(
        Expectation::matching(request::method_path("GET", "/v2/cryptocurrency/quotes/latest")).respond_with(resp_json),
    );

    let mut default_cfg = TokenPriceConfig::new("testnet", None).unwrap();
    let url = server.url("").to_string();
    default_cfg.base_url = url.split_at(url.len() - 1).0.to_string();

    let mut tp = TokenPrice::new(&default_cfg, "").unwrap();
    let price = tp.price("BTC").await;
    assert_eq!(price.err().unwrap(), TokenPriceError::TokenNotSupport);
    let price = tp.price("mMATIC").await;
    assert_eq!(price.err().unwrap(), TokenPriceError::InternalError);
}
