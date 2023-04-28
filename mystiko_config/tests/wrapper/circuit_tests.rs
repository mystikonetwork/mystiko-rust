use mystiko_config::raw::circuit::RawCircuitConfig;
use mystiko_config::raw::create_raw_from_file;
use mystiko_config::wrapper::circuit::CircuitConfig;
use mystiko_types::CircuitType;
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/circuit/valid.json";

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawCircuitConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = CircuitConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    assert_eq!(config.name(), "zokrates-1.0-rollup1");
    assert_eq!(config.circuit_type(), &CircuitType::Rollup1);
    assert!(config.is_default());
    assert_eq!(config.program_file(), &vec![String::from("./Rollup1.program.gz")]);
    assert_eq!(config.abi_file(), &vec![String::from("./Rollup1.abi.json")]);
    assert_eq!(config.proving_key_file(), &vec![String::from("./Rollup1.pkey.gz")]);
    assert_eq!(config.verifying_key_file(), &vec![String::from("./Rollup1.vkey.gz")]);
}
