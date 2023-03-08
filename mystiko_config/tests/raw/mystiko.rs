use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::raw::base::{RawConfig, Validator};
use mystiko_config::raw::bridge::tbridge::RawTBridgeConfig;
use mystiko_config::raw::indexer::RawIndexerConfig;
use mystiko_config::raw::mystiko::{RawBridgeConfigType, RawMystikoConfig};

async fn default_config() -> RawMystikoConfig {
    RawConfig::create_from_file::<RawMystikoConfig>(
        "tests/files/mystiko.valid.json"
    ).await
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<RawMystikoConfig> = AsyncOnce::new(async {
       default_config().await
    });
}

#[tokio::test]
async fn test_valid_success() {
    let config = CONFIG_CREATER.get().await;
    config.validation();
}

#[tokio::test]
#[should_panic]
async fn test_invalid_version_0() {
    let mut config = default_config().await;
    config.version = String::from("");
    config.validation();
}

#[tokio::test]
#[should_panic]
async fn test_invalid_version_1() {
    let mut config = default_config().await;
    config.version = String::from("wrong version");
    config.validation();
}

#[tokio::test]
#[should_panic]
async fn test_invalid_chains() {
    let mut config = default_config().await;
    config.chains.append(&mut config.chains.clone());
    config.validation()
}

#[tokio::test]
#[should_panic]
async fn test_invalid_bridges_0() {
    let mut config = default_config().await;
    config.bridges.append(&mut config.bridges.clone());
    config.validation();
}

#[tokio::test]
#[should_panic]
async fn test_invalid_bridges_1() {
    let mut config = default_config().await;
    let bridge_config = RawTBridgeConfig::new("".to_string());
    config.bridges.push(RawBridgeConfigType::Tbridge(bridge_config));
    config.validation();
}

#[tokio::test]
#[should_panic]
async fn test_invalid_circuits_0() {
    let mut config = default_config().await;
    config.circuits.append(&mut config.circuits.clone());
    config.validation();
}

#[tokio::test]
#[should_panic]
async fn test_invalid_circuits_1() {
    let mut config = default_config().await;
    let mut circuit_configs = config.circuits;
    circuit_configs[0].name = "".to_string();
    config.circuits = circuit_configs;
    config.validation();
}

#[tokio::test]
#[should_panic]
async fn test_invalid_indexer() {
    let mut config = default_config().await;
    config.indexer = Some(RawIndexerConfig::new("not a url".to_string(), 1000));
    config.validation();
}

#[tokio::test]
#[should_panic]
async fn test_import_invalid_json_file() {
    let _file_config =
        RawConfig::create_from_file::<RawMystikoConfig>("tests/files/mystiko.invalid.json").await;
}