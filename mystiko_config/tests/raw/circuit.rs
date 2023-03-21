use mystiko_config::common::CircuitType;
use mystiko_config::raw::base::{RawConfig, Validator};
use mystiko_config::raw::circuit::RawCircuitConfig;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

async fn default_config() -> RawCircuitConfig {
    RawConfig::from_object::<RawCircuitConfig>(
        RawCircuitConfig::builder()
            .name("zokrates-1.0-rollup1".to_string())
            .circuit_type(CircuitType::Rollup1)
            .is_default(true)
            .program_file(vec![String::from("./Rollup1.program.gz")])
            .abi_file(vec![String::from("./Rollup1.abi.json")])
            .proving_key_file(vec![String::from("./Rollup1.pkey.gz")])
            .verifying_key_file(vec![String::from("./Rollup1.vkey.gz")])
            .build(),
    )
    .unwrap()
}

#[tokio::test]
async fn test_hash() {
    let config1 = default_config().await;
    let mut hasher = DefaultHasher::new();
    config1.hash(&mut hasher);
    let hash1 = hasher.finish();

    hasher = DefaultHasher::new();
    let mut config2 = default_config().await;
    config2.name = "zokrates-1.1-rollup1".to_string();
    config2.hash(&mut hasher);
    let hash2 = hasher.finish();

    assert_ne!(hash1, hash2);
}

#[tokio::test]
async fn test_invalid_name() {
    let mut config = default_config().await;
    config.name = String::from("");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_program_file() {
    let mut config = default_config().await;
    config.program_file = vec![String::from("")];
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_abi_file() {
    let mut config = default_config().await;
    config.abi_file = vec![String::from("")];
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_proving_key_file() {
    let mut config = default_config().await;
    config.proving_key_file = vec![String::from("")];
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_verifying_key_file() {
    let mut config = default_config().await;
    config.verifying_key_file = vec![String::from("")];
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config = RawConfig::from_file::<RawCircuitConfig>("tests/files/circuit.valid.json")
        .await
        .unwrap();
    assert_eq!(file_config, default_config().await);
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        RawConfig::from_file::<RawCircuitConfig>("tests/files/circuit.invalid.json").await;
    assert!(file_config.is_err());
}
