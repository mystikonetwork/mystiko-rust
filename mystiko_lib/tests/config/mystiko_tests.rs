use anyhow::Result;
use mystiko_lib::config::get;
use mystiko_lib::initialize;
use mystiko_protos::common::v1::ConfigOptions;
use mystiko_protos::core::v1::MystikoOptions;
use prost::Message;

#[test]
fn test_get_config() {
    assert!(setup().is_ok());
    let result = get();
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.version, "0.1.0");
}

fn setup() -> Result<()> {
    let options = MystikoOptions::builder()
        .config_options(
            ConfigOptions::builder()
                .file_path(String::from("tests/files/config.json"))
                .is_testnet(true)
                .build(),
        )
        .build();
    initialize(&options.encode_to_vec())
}
