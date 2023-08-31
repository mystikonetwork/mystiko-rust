use mystiko_lib::{initialize, is_initialized};
use mystiko_protos::core::v1::MystikoOptions;
use prost::Message;

mod config;
mod handler;

#[test]
fn test_initialize() {
    let options = MystikoOptions::builder()
        .is_testnet(true)
        .is_staging(false)
        .config_file_path(String::from("tests/files/config.json"))
        .build();
    let result = initialize(&options.encode_to_vec());
    assert!(result.is_ok());
    assert!(is_initialized());
}
