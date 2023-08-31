use mystiko_lib::core::{mystiko_initialize, mystiko_is_initialized};
use mystiko_protos::core::v1::MystikoOptions;
use prost::Message;

#[test]
fn test_mystiko_initialize() {
    let options = MystikoOptions::builder()
        .is_testnet(true)
        .is_staging(false)
        .config_file_path(String::from("tests/files/config.json"))
        .build();
    let result = mystiko_initialize(&options.encode_to_vec());
    assert!(result.is_ok());
    assert!(mystiko_is_initialized());
}
