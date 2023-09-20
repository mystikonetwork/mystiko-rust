use anyhow::Result;
use mystiko_lib::{destroy, initialize, is_initialized};
use mystiko_protos::common::v1::ConfigOptions;
use mystiko_protos::core::v1::MystikoOptions;
use serial_test::serial;

mod config;
mod handler;

const VALID_CONFIG_FILE: &str = "tests/files/valid.json";
const FULL_CONFIG_FILE: &str = "tests/files/full.json";

pub fn setup(full_config: bool) -> Result<()> {
    destroy();
    let file_path = if full_config {
        FULL_CONFIG_FILE.to_string()
    } else {
        VALID_CONFIG_FILE.to_string()
    };
    let options = MystikoOptions::builder()
        .config_options(ConfigOptions::builder().file_path(file_path).is_testnet(true).build())
        .build();
    initialize(options)
}

#[test]
#[serial]
fn test_initialize() {
    assert!(setup(false).is_ok());
    assert!(is_initialized());
}
