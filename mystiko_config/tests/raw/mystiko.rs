use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::raw::bridge::tbridge::RawTBridgeConfig;
use mystiko_config::raw::indexer::RawIndexerConfig;
use mystiko_config::raw::mystiko::{RawBridgeConfigType, RawMystikoConfig};
use mystiko_config::raw::{create_raw_from_file, Validator};

async fn default_config() -> RawMystikoConfig {
    create_raw_from_file::<RawMystikoConfig>("tests/files/mystiko.valid.json")
        .await
        .unwrap()
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<RawMystikoConfig> =
        AsyncOnce::new(async { default_config().await });
}

#[tokio::test]
async fn test_valid_success() {
    let config = CONFIG_CREATER.get().await;
    assert_eq!(config.validation().is_err(), false);
}

#[tokio::test]
async fn test_invalid_version_0() {
    let mut config = default_config().await;
    config.version = String::from("");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_version_1() {
    let mut config = default_config().await;
    config.version = String::from("wrong version");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_chains() {
    let mut config = default_config().await;
    config.chains.append(&mut config.chains.clone());
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_bridges_0() {
    let mut config = default_config().await;
    config.bridges.append(&mut config.bridges.clone());
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_bridges_1() {
    let mut config = default_config().await;
    let bridge_config = RawTBridgeConfig::builder().name("".to_string()).build();
    config
        .bridges
        .push(RawBridgeConfigType::Tbridge(bridge_config));
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_circuits_0() {
    let mut config = default_config().await;
    config.circuits.append(&mut config.circuits.clone());
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_circuits_1() {
    let mut config = default_config().await;
    let mut circuit_configs = config.circuits;
    circuit_configs[0].name = "".to_string();
    config.circuits = circuit_configs;
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_indexer() {
    let mut config = default_config().await;
    config.indexer = Some(
        RawIndexerConfig::builder()
            .url("not a url".to_string())
            .timeout_ms(1000)
            .build(),
    );
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        create_raw_from_file::<RawMystikoConfig>("tests/files/mystiko.invalid.json").await;
    assert_eq!(file_config.is_err(), true);
}
