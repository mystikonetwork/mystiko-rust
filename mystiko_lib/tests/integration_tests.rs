use mystiko_lib::{initialize, is_initialized};
use mystiko_protos::config::v1::ConfigOptions;
use mystiko_protos::core::v1::MystikoOptions;
use prost::Message;

mod config;
mod handler;

#[test]
fn test_initialize() {
    let options = MystikoOptions::builder()
        .config_options(
            ConfigOptions::builder()
                .file_path(String::from("tests/files/config.json"))
                .is_testnet(true)
                .is_staging(false)
                .build(),
        )
        .build();
    let result = initialize(&options.encode_to_vec());
    assert!(result.is_ok());
    assert!(is_initialized());
}
