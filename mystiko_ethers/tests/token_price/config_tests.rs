use mystiko_ethers::token_price::config::{read_config_from_file, TokenPriceConfig};
use mystiko_ethers::token_price::error::TokenPriceError;

#[tokio::test]
async fn test_read_config() {
    let cfg = read_config_from_file("./tests/token_price/files/xxx.json").await;
    assert_eq!(cfg.err().unwrap(), TokenPriceError::FileError("".into()));

    let cfg = read_config_from_file("./tests/token_price/files/token_price_wrong.json").await;
    assert!(matches!(cfg.err().unwrap(), TokenPriceError::SerdeJsonError(_)));

    let cfg = read_config_from_file("./src/token_price/config/config.json").await;
    assert!(cfg.is_ok());
    let cfg = cfg.unwrap();
    let default_cfg = TokenPriceConfig::default();
    assert_eq!(cfg, default_cfg);

    let ser = serde_json::to_value(cfg.clone()).unwrap();
    let cfg_des: TokenPriceConfig = serde_json::from_value(ser).unwrap();
    assert_eq!(cfg, cfg_des);

    let tokens = cfg.tokens();
    assert_eq!(tokens.len(), 17);
    let ids = cfg.ids();
    assert_eq!(ids.len(), 9);
}
