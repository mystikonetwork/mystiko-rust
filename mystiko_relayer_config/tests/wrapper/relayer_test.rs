use mystiko_relayer_config::raw::chain::RawChainConfig;
use mystiko_relayer_config::raw::create_raw_from_file;
use mystiko_relayer_config::raw::relayer::RawRelayerConfig;
use mystiko_relayer_config::wrapper::chain::ChainConfig;
use mystiko_relayer_config::wrapper::relayer::RelayerConfig;
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/relayer.valid.json";
const VALID_CHAIN_CONFIG_FILE: &str = "tests/files/chain.valid.json";

async fn create_chain_config() -> ChainConfig {
    let raw_config = create_raw_from_file::<RawChainConfig>(VALID_CHAIN_CONFIG_FILE)
        .await
        .unwrap();
    let config = ChainConfig::new(Arc::new(raw_config)).unwrap();
    config.validate().unwrap();
    config
}

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawRelayerConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = RelayerConfig::from_raw(raw_config).unwrap();
    config.validate().unwrap();

    assert_eq!(config.version(), "0.0.1");
    let chain_config_option = config.find_chain_config(5);
    assert!(chain_config_option.is_some());
    let chain_config = chain_config_option.unwrap();
    assert_eq!(chain_config, &create_chain_config().await);
    assert!(config.find_chain_config(97).is_none());

    assert_eq!(config.chains().len(), 1);
}

#[tokio::test]
#[should_panic(expected = "duplicate default chain config for chain_id 5")]
async fn test_create_invalid_config() {
    let mut raw_config = create_raw_from_file::<RawRelayerConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let raw_chain_config = create_raw_from_file::<RawChainConfig>(VALID_CHAIN_CONFIG_FILE)
        .await
        .unwrap();
    raw_config.chains.push(Arc::new(raw_chain_config));
    let config = RelayerConfig::from_raw(raw_config);
    assert!(config.is_err());
    config.unwrap();
}

#[test]
fn test_create_from_json_str() {
    assert!(RelayerConfig::from_json_str("{}").is_err());
    let json_str = r#"
        {
          "version": "0.0.1",
          "chains": []
        }
    "#;
    assert!(RelayerConfig::from_json_str(json_str).is_ok());
}

#[tokio::test]
async fn test_create_from_json_file() {
    let config = RelayerConfig::from_json_file(VALID_CONFIG_FILE)
        .await
        .unwrap();
    config.validate().unwrap();
}
