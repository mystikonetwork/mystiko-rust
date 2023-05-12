use mystiko_server_utils::tx_manager::config::TxManagerConfig;
use mystiko_server_utils::tx_manager::error::TxManagerError;

#[tokio::test]
async fn test_read_config() {
    let cfg = TxManagerConfig::from_json_file("./tests/tx_manager/files/xxx.json").await;
    assert_eq!(cfg.err().unwrap(), TxManagerError::FileError("".into()));

    let cfg = TxManagerConfig::from_json_file("./tests/tx_manager/files/tx_manager_wrong.json").await;
    assert!(matches!(cfg.err().unwrap(), TxManagerError::SerdeJsonError(_)));

    let cfg = TxManagerConfig::from_json_file("./src/tx_manager/config/tx_manager.json").await;
    assert!(cfg.is_ok());
    let cfg = cfg.unwrap();
    let default_cfg = TxManagerConfig::default();
    assert_eq!(cfg, default_cfg);

    let ser = serde_json::to_value(cfg.clone()).unwrap();
    let cfg_des: TxManagerConfig = serde_json::from_value(ser).unwrap();
    assert_eq!(cfg, cfg_des)
}
