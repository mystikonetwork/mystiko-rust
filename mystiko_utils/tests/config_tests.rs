use config::FileFormat;
use mystiko_utils::config::{load_config, ConfigFile, ConfigLoadOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
struct MockChainConfig {
    #[builder(default = "http://localhost:8545".to_string())]
    pub url: String,
    #[builder(default = "".to_string())]
    pub api_key: String,
}

impl Default for MockChainConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
struct MockConfig {
    #[builder(default = 1000)]
    pub timeout_ms: u64,
    pub chains: HashMap<u64, MockChainConfig>,
}

impl Default for MockConfig {
    fn default() -> Self {
        let mut chains = HashMap::new();
        chains.insert(1, MockChainConfig::builder().build());
        Self::builder().chains(chains).build()
    }
}

#[test]
fn test_load_chain_url_from_env() {
    let params = ConfigLoadOptions::builder()
        .paths(vec![])
        .env_prefix("MOCK_TEST_CONFIG_123456")
        .build();
    let cfg = load_config::<PathBuf, MockConfig>(&params);
    assert_eq!(cfg.as_ref().unwrap().timeout_ms, 1000);
    assert_eq!(cfg.as_ref().unwrap().chains.len(), 1);
    assert_eq!(
        cfg.as_ref().unwrap().chains.get(&1).unwrap().url,
        "http://localhost:8545"
    );
    assert_eq!(cfg.as_ref().unwrap().chains.get(&1).unwrap().api_key, "");

    std::env::set_var("MOCK_TEST_CONFIG.CHAINS.1.URL", "http://mock_localhost:8546");
    std::env::set_var("MOCK_TEST_CONFIG.CHAINS.1.API_KEY", "key123");
    let params = ConfigLoadOptions::builder()
        .paths(vec![])
        .env_prefix("MOCK_TEST_CONFIG")
        .build();
    let cfg = load_config::<PathBuf, MockConfig>(&params);
    assert_eq!(cfg.as_ref().unwrap().timeout_ms, 1000);
    assert_eq!(
        cfg.as_ref().unwrap().chains.get(&1).unwrap().url,
        "http://mock_localhost:8546"
    );
    assert_eq!(cfg.as_ref().unwrap().chains.get(&1).unwrap().api_key, "key123");

    let path = PathBuf::from("./tests/files/config");
    let format = FileFormat::Json;
    let params = ConfigLoadOptions::builder()
        .paths(ConfigFile::builder().path(path).format(format).build())
        .env_prefix("MOCK_TEST_CONFIG")
        .build();
    let cfg = load_config::<PathBuf, MockConfig>(&params);
    assert_eq!(cfg.as_ref().unwrap().timeout_ms, 2000);
    assert_eq!(cfg.as_ref().unwrap().chains.len(), 2);
    assert_eq!(
        cfg.as_ref().unwrap().chains.get(&1).unwrap().url,
        "http://mock_localhost:8546"
    );
    assert_eq!(cfg.as_ref().unwrap().chains.get(&1).unwrap().api_key, "key123");
    assert_eq!(
        cfg.as_ref().unwrap().chains.get(&2).unwrap().url,
        "http://localhost:8546"
    );
    assert_eq!(cfg.as_ref().unwrap().chains.get(&2).unwrap().api_key, "");

    let path = "./tests/files/config";
    let format = FileFormat::Toml;
    let params = ConfigLoadOptions::builder()
        .paths(ConfigFile::builder().path(path).format(format).build())
        .env_prefix("MOCK_TEST_CONFIG")
        .build();
    let cfg = load_config::<PathBuf, MockConfig>(&params);
    assert_eq!(cfg.as_ref().unwrap().timeout_ms, 2000);
    assert_eq!(cfg.as_ref().unwrap().chains.len(), 2);
    assert_eq!(
        cfg.as_ref().unwrap().chains.get(&1).unwrap().url,
        "http://mock_localhost:8546"
    );
    assert_eq!(cfg.as_ref().unwrap().chains.get(&1).unwrap().api_key, "key123");
    assert_eq!(
        cfg.as_ref().unwrap().chains.get(&2).unwrap().url,
        "http://localhost.toml:8546"
    );
    assert_eq!(cfg.as_ref().unwrap().chains.get(&2).unwrap().api_key, "");

    std::env::remove_var("MOCK_TEST_CONFIG.CHAINS.1.URL");
    std::env::remove_var("MOCK_TEST_CONFIG.CHAINS.1.API_KEY");
}
