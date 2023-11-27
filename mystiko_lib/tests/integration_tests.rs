use mystiko_lib::{destroy, initialize, is_initialized};
use mystiko_protos::api::v1::api_response;
use mystiko_protos::common::v1::ConfigOptions;
use mystiko_protos::core::v1::MystikoOptions;
use serial_test::serial;

mod config;
mod handler;

const VALID_CONFIG_FILE: &str = "tests/files/valid.json";
const FULL_CONFIG_FILE: &str = "tests/files/full.json";

pub fn setup(full_config: bool) {
    destroy();
    let file_path = if full_config {
        FULL_CONFIG_FILE.to_string()
    } else {
        VALID_CONFIG_FILE.to_string()
    };
    let options = MystikoOptions::builder()
        .config_options(ConfigOptions::builder().file_path(file_path).is_testnet(true).build())
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
    setup(false);
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
