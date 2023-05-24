use mystiko_server_utils::tx_manager::config::TxManagerConfig;
use std::env;

#[tokio::test]
async fn test_read_config() {
    let cfg = TxManagerConfig::new("testnet", None).unwrap();
    assert_eq!(cfg.max_gas_price.to_string(), "200000000000");
    assert_eq!(cfg.max_confirm_count, 100);
    assert_eq!(cfg.gas_limit_reserve_percentage, 10);

    let cfg = TxManagerConfig::new("mainnet", None).unwrap();
    assert_eq!(cfg.max_gas_price.to_string(), "100000000000");
    assert_eq!(cfg.max_confirm_count, 100);
    assert_eq!(cfg.gas_limit_reserve_percentage, 10);

    let cfg = TxManagerConfig::new("testnet", Some("tests/tx_manager/files")).unwrap();
    assert_eq!(cfg.max_gas_price.to_string(), "500000000000");

    env::set_var("MYSTIKO_TX_MANAGER.MAX_GAS_PRICE", "0x45d964b800");
    env::set_var("MYSTIKO_TX_MANAGER.MAX_CONFIRM_COUNT", "10");
    env::set_var("MYSTIKO_TX_MANAGER.GAS_LIMIT_RESERVE_PERCENTAGE", "20");
    let cfg = TxManagerConfig::new("mainnet", None).unwrap();
    assert_eq!(cfg.max_gas_price.to_string(), "300000000000");
    assert_eq!(cfg.max_confirm_count, 10);
    assert_eq!(cfg.gas_limit_reserve_percentage, 20);

    let cfg = TxManagerConfig::new("testnet", Some("tests/tx_manager/files")).unwrap();
    assert_eq!(cfg.max_gas_price.to_string(), "300000000000");
}
