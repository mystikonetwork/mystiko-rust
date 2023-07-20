use mystiko_server_utils::tx_manager::config::TxManagerConfig;
use std::env;

#[tokio::test]
async fn test_read_config() {
    // test invalid config
    let cfg = TxManagerConfig::new(Some("tests/tx_manager/files/invalid"));
    assert!(cfg.is_err());

    let cfg = TxManagerConfig::new(None).unwrap();
    assert_eq!(cfg.max_confirm_count, 100);
    assert_eq!(cfg.gas_limit_reserve_percentage, 10);

    let cfg = TxManagerConfig::new(Some("tests/tx_manager/files")).unwrap();
    assert_eq!(cfg.max_confirm_count, 123456);

    env::set_var("MYSTIKO_TX_MANAGER.MAX_CONFIRM_COUNT", "112");
    env::set_var("MYSTIKO_TX_MANAGER.GAS_LIMIT_RESERVE_PERCENTAGE", "24");
    let cfg = TxManagerConfig::new(None).unwrap();
    assert_eq!(cfg.max_confirm_count, 112);
    assert_eq!(cfg.gas_limit_reserve_percentage, 24);
    let min_priority_fee_per_gas = cfg.get_min_priority_fee_per_gas(137);
    assert_eq!(min_priority_fee_per_gas, 30000000000);
    let min_priority_fee_per_gas = cfg.get_min_priority_fee_per_gas(1);
    assert_eq!(min_priority_fee_per_gas, 1000000000);
}
