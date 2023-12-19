use mystiko_lib::{destroy, initialize, is_initialized};
use mystiko_protos::api::v1::api_response;
use mystiko_protos::common::v1::ConfigOptions;
use mystiko_protos::core::v1::MystikoOptions;
use serial_test::serial;

mod config;
mod handler;
mod scanner;
mod synchronizer;

const VALID_CONFIG_FILE: &str = "tests/files/valid.json";
const FULL_CONFIG_FILE: &str = "tests/files/full.json";
const DEPOSIT_CONFIG_FILE: &str = "tests/files/deposit.json";

pub fn setup(config_path: Option<String>) {
    destroy();
    let config_path = match config_path {
        None => VALID_CONFIG_FILE.to_string(),
        Some(s) => s,
    };
    let options = MystikoOptions::builder()
        .config_options(ConfigOptions::builder().file_path(config_path).is_testnet(true).build())
        .build();
    let response = initialize(options);
    assert!(response.code.unwrap().success);
}

pub fn extract_data(result: api_response::Result) -> Vec<u8> {
    match result {
        api_response::Result::Data(data) => data,
        api_response::Result::ErrorMessage(err) => {
            panic!("{}", err);
        }
    }
}

#[test]
#[serial]
fn test_initialize() {
    setup(None);
    assert!(is_initialized());
}

#[test]
#[serial]
fn test_initialize_with_error_config() {
    destroy();
    let file_path = "tests/files/error.json";
    let options = MystikoOptions::builder()
        .config_options(
            ConfigOptions::builder()
                .file_path(file_path.to_string())
                .is_testnet(true)
                .build(),
        )
        .build();
    let response = initialize(options);
    let status_code = response.code.unwrap();
    assert!(!status_code.success);
    assert!(status_code.error.is_some());
}
