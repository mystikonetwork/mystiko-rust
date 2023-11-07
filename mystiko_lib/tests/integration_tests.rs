use mystiko_lib::{destroy, initialize, is_initialized};
use mystiko_protos::api::v1::{api_response, StatusCode};
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
    assert_eq!(response.code, StatusCode::Success as i32);
}

pub fn extract_data(result: api_response::Result) -> Vec<u8> {
    match result {
        api_response::Result::Data(data) => {
            return data;
        }
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
