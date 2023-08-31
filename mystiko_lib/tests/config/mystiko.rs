use mystiko_lib::config::mystiko::config_version;
use mystiko_lib::core::{mystiko_initialize, mystiko_is_initialized};
use mystiko_protos::core::v1::MystikoOptions;
use prost::Message;

#[test]
fn test_config_version() {
    let options = MystikoOptions::builder()
        .is_testnet(true)
        .config_file_path(String::from("tests/files/config.json"))
        .build();
    let result = mystiko_initialize(&options.encode_to_vec());
    assert!(result.is_ok());
    assert!(mystiko_is_initialized());

    let result = config_version();
    assert!(result.is_ok());
    let version = result.unwrap();
    assert_eq!(version.as_str(), "0.1.0");
}
