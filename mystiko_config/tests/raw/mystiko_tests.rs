use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::raw::bridge::tbridge::RawTBridgeConfig;
use mystiko_config::raw::bridge::RawBridgeConfig;
use mystiko_config::raw::create_raw_from_file;
use mystiko_config::raw::indexer::RawIndexerConfig;
use mystiko_config::raw::mystiko::RawMystikoConfig;
use mystiko_config::raw::packer::RawPackerConfig;
use std::sync::Arc;
use validator::Validate;

async fn default_config() -> RawMystikoConfig {
    create_raw_from_file::<RawMystikoConfig>("tests/files/mystiko/valid.json")
        .await
        .unwrap()
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<RawMystikoConfig> = AsyncOnce::new(async { default_config().await });
}

#[tokio::test]
async fn test_valid_success() {
    let config = CONFIG_CREATER.get().await;
    assert!(config.validate().is_ok());
}

#[tokio::test]
async fn test_invalid_version_0() {
    let mut config = default_config().await;
    config.version = String::from("");
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_invalid_version_1() {
    let mut config = default_config().await;
    config.version = String::from("wrong version");
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_invalid_git_revision() {
    let mut config = default_config().await;
    config.git_revision = Some(String::from(""));
    assert!(config.validate().is_err());
    config.git_revision = Some(String::from("wrong git revision"));
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_invalid_chains() {
    let mut config = default_config().await;
    config.chains.append(&mut config.chains.clone());
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_invalid_bridges_0() {
    let mut config = default_config().await;
    config.bridges.append(&mut config.bridges.clone());
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_invalid_bridges_1() {
    let mut config = default_config().await;
    let bridge_config = Arc::new(RawTBridgeConfig::builder().name("".to_string()).build());
    config.bridges.push(Arc::new(RawBridgeConfig::Tbridge(bridge_config)));
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_invalid_circuits_0() {
    let mut config = default_config().await;
    config.circuits.append(&mut config.circuits.clone());
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_invalid_circuits_1() {
    let mut config = default_config().await;
    let mut circuit_config = (*config.circuits.remove(0)).clone();
    circuit_config.name = "".to_string();
    config.circuits.insert(0, Arc::new(circuit_config));
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_invalid_indexer() {
    let mut config = default_config().await;
    config.indexer = Some(Arc::new(
        RawIndexerConfig::builder()
            .url("not a url".to_string())
            .timeout_ms(1000)
            .build(),
    ));
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_invalid_packer() {
    let mut config = default_config().await;
    config.packer = Some(Arc::new(
        RawPackerConfig::builder()
            .url("not a url".to_string())
            .client_timeout_ms(1000u64)
            .build(),
    ));
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config = create_raw_from_file::<RawMystikoConfig>("tests/files/mystiko/invalid.json").await;
    assert!(file_config.is_err());
}

#[tokio::test]
async fn test_empty_file() {
    let file_config = create_raw_from_file::<RawMystikoConfig>("tests/files/mystiko/empty.json").await;
    assert!(file_config.is_ok());
}
