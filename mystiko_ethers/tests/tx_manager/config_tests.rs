use mystiko_ethers::tx_manager::config::{read_config_from_file, TxManagerConfig};
use mystiko_ethers::tx_manager::error::TxManagerError;

#[tokio::test]
async fn test_read_config() {
    let cfg = read_config_from_file("./tests/tx_manager/files/xxx.json").await;
    assert_eq!(cfg.err().unwrap(), TxManagerError::FileError("".into()));

    let cfg = read_config_from_file("./tests/tx_manager/files/tx_manager_wrong.json").await;
    assert!(matches!(
        cfg.err().unwrap(),
        TxManagerError::SerdeJsonError(_)
    ));

    let cfg = read_config_from_file("./src/tx_manager/config/config.json").await;
    assert!(cfg.is_ok());
    let cfg = cfg.unwrap();
    let default_cfg = TxManagerConfig::default();
    assert_eq!(cfg, default_cfg);

    let ser = serde_json::to_value(cfg.clone()).unwrap();
    let cfg_des: TxManagerConfig = serde_json::from_value(ser).unwrap();
    assert_eq!(cfg, cfg_des)
}
