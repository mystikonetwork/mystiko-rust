extern crate mystiko_roller;

use mystiko_roller::common::error::RollerError;
use mystiko_roller::config::config::RollerConfig;

#[tokio::test]
async fn test_read_config() {
    let cfg = RollerConfig::from_json_file("./tests/files/xxx.json").await;
    assert_eq!(cfg.err().unwrap(), RollerError::FileError("".into()));

    let cfg = RollerConfig::from_json_file("./tests/files/roller_config_wrong.json").await;
    assert!(matches!(cfg.err().unwrap(), RollerError::SerdeJsonError(_)));

    let cfg = RollerConfig::from_json_file("./src/config/roller.json").await;
    assert!(cfg.is_ok());
    let cfg = cfg.unwrap();
    let default_cfg = RollerConfig::default();
    assert_eq!(cfg, default_cfg);

    let ser = serde_json::to_value(cfg.clone()).unwrap();
    let cfg_des: RollerConfig = serde_json::from_value(ser).unwrap();
    assert_eq!(cfg, cfg_des);
}
