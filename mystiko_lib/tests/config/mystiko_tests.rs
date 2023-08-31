use mystiko_lib::config::version;
use mystiko_lib::{initialize, is_initialized};
use mystiko_protos::core::v1::MystikoOptions;
use prost::Message;

#[test]
fn test_version() {
    let options = MystikoOptions::builder()
        .is_testnet(true)
        .config_file_path(String::from("tests/files/config.json"))
        .build();
    let result = initialize(&options.encode_to_vec());
    assert!(result.is_ok());
    assert!(is_initialized());

    let result = version();
    assert!(result.is_ok());
    let version = result.unwrap();
    assert_eq!(version.as_str(), "0.1.0");
}
