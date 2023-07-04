use mystiko_server_utils::token_price::config::TokenPriceConfig;
use std::env;

#[tokio::test]
async fn test_read_config() {
    let cfg = TokenPriceConfig::new("testnet", None).unwrap();
    assert_eq!(cfg.price_cache_ttl, 72000);

    let cfg = TokenPriceConfig::new("mainnet", None).unwrap();
    assert_eq!(cfg.price_cache_ttl, 1800);

    let cfg = TokenPriceConfig::new("mainnet", Some("tests/token_price/files/config")).unwrap();
    assert_eq!(cfg.price_cache_ttl, 90000);

    env::set_var("MYSTIKO_TOKEN_PRICE.PRICE_CACHE_TTL", "800");
    let cfg = TokenPriceConfig::new("mainnet", None).unwrap();
    assert_eq!(cfg.price_cache_ttl, 800);

    let cfg = TokenPriceConfig::new("mainnet", Some("tests/token_price/files/config")).unwrap();
    assert_eq!(cfg.price_cache_ttl, 800);
    let tokens = cfg.tokens();
    assert!(tokens.len() > 1);
}
