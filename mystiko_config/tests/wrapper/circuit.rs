use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::raw::base::RawConfig;
use mystiko_config::raw::circuit::RawCircuitConfig;
use mystiko_config::wrapper::circuit::CircuitConfig;

async fn default_raw_config() -> RawCircuitConfig {
    RawConfig::create_from_file::<RawCircuitConfig>("tests/files/circuit.valid.json")
        .await
        .unwrap()
}

async fn default_circuit_config() -> CircuitConfig {
    CircuitConfig::new(default_raw_config().await)
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<CircuitConfig> =
        AsyncOnce::new(async { default_circuit_config().await });
    static ref RAW_CONFIG_CREATER: AsyncOnce<RawCircuitConfig> =
        AsyncOnce::new(async { default_raw_config().await });
}

#[tokio::test]
async fn test_equality() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    assert_eq!(config.name(), &raw_config.name);
    assert_eq!(config.circuit_type(), &raw_config.circuit_type);
    assert_eq!(config.is_default(), raw_config.is_default);
    assert_eq!(config.program_file(), raw_config.program_file);
    assert_eq!(config.abi_file(), raw_config.abi_file);
    assert_eq!(config.proving_key_file(), raw_config.proving_key_file);
    assert_eq!(config.verifying_key_file(), raw_config.verifying_key_file);
}

#[tokio::test]
async fn test_copy() {
    let config = CONFIG_CREATER.get().await;
    assert_eq!(&CircuitConfig::new(config.copy_data()), config);
}

#[tokio::test]
async fn test_mutate() {
    let mut raw_config = default_raw_config().await;
    let config = CONFIG_CREATER.get().await;
    assert_eq!(&config.mutate(None), config);
    raw_config.name = "another name".to_string();
    let new_config = config.mutate(Some(raw_config));
    assert_eq!(new_config.name(), "another name");
}

#[tokio::test]
async fn test_to_json_string() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    let json_string = config.to_json_string();
    let loaded_raw_config =
        RawConfig::create_from_json_string::<RawCircuitConfig>(json_string.as_str())
            .await
            .unwrap();
    assert_eq!(&loaded_raw_config, raw_config);
}
