use config::FileFormat;
use mystiko_utils::config::{load_config, ConfigFile, ConfigLoadOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, PartialEq)]
struct MockChainConfig {
    #[builder(default)]
    pub url: Option<String>,
    #[builder(default)]
    pub api_key: Option<String>,
}

impl Default for MockChainConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, PartialEq)]
struct MockOptionCheckerActorConfig {
    #[builder(default)]
    pub chains: HashMap<u64, MockChainConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
struct MockOptionCheckerConfig {
    #[builder(default)]
    pub control: Option<MockOptionCheckerActorConfig>,
    #[builder(default)]
    pub actor: Option<MockOptionCheckerActorConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
struct MockOptionChainConfig {
    #[builder(default)]
    pub name: Option<String>,
    #[builder(default)]
    pub chains: HashMap<u64, MockChainConfig>,
    #[builder(default)]
    pub chain_checker: Option<MockOptionCheckerConfig>,
}

impl Default for MockOptionChainConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
struct MockConfig {
    #[builder(default)]
    pub timeout_ms: Option<u64>,
    #[builder(default)]
    pub chains: HashMap<u64, MockChainConfig>,
    #[builder(default)]
    pub option_chains: MockOptionChainConfig,
}

impl Default for MockConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[test]
fn test_load_chain_from_env() {
    let params = ConfigLoadOptions::builder()
        .paths(vec![])
        .env_prefix("MOCK_TEST_CONFIG_CHAIN")
        .build();
    let cfg = load_config::<PathBuf, MockConfig>(&params).unwrap();
    assert!(cfg.timeout_ms.is_none());
    assert_eq!(cfg.chains.len(), 0);

    std::env::set_var("MOCK_TEST_CONFIG_CHAIN.CHAINS.1.URL", "http://mock_localhost:8546");
    std::env::set_var("MOCK_TEST_CONFIG_CHAIN.CHAINS.1.API_KEY", "key123");
    let params = ConfigLoadOptions::builder()
        .paths(vec![])
        .env_prefix("MOCK_TEST_CONFIG_CHAIN")
        .build();
    let cfg = load_config::<PathBuf, MockConfig>(&params).unwrap();
    assert!(cfg.timeout_ms.is_none());
    assert_eq!(
        cfg.chains.get(&1).unwrap().url,
        Some("http://mock_localhost:8546".to_string())
    );
    assert_eq!(cfg.chains.get(&1).unwrap().api_key, Some("key123".to_string()));

    let path = PathBuf::from("./tests/files/config");
    let format = FileFormat::Json;
    let params = ConfigLoadOptions::builder()
        .paths(ConfigFile::builder().path(path).format(format).build())
        .env_prefix("MOCK_TEST_CONFIG_CHAIN")
        .build();
    let cfg = load_config::<PathBuf, MockConfig>(&params).unwrap();
    assert_eq!(cfg.timeout_ms, Some(2000));
    assert_eq!(cfg.chains.len(), 2);
    assert_eq!(
        cfg.chains.get(&1).unwrap().url,
        Some("http://mock_localhost:8546".to_string())
    );
    assert_eq!(cfg.chains.get(&1).unwrap().api_key, Some("key123".to_string()));
    assert_eq!(
        cfg.chains.get(&2).unwrap().url,
        Some("http://localhost:8546".to_string())
    );
    assert_eq!(cfg.chains.get(&2).unwrap().api_key, Some("".to_string()));

    let path = "./tests/files/config";
    let format = FileFormat::Toml;
    let params = ConfigLoadOptions::builder()
        .paths(ConfigFile::builder().path(path).format(format).build())
        .env_prefix("MOCK_TEST_CONFIG_CHAIN")
        .build();
    let cfg = load_config::<PathBuf, MockConfig>(&params).unwrap();
    assert_eq!(cfg.timeout_ms, Some(2000));
    assert_eq!(cfg.chains.len(), 2);
    assert_eq!(
        cfg.chains.get(&1).unwrap().url,
        Some("http://mock_localhost:8546".to_string())
    );
    assert_eq!(cfg.chains.get(&1).unwrap().api_key, Some("key123".to_string()));
    assert_eq!(
        cfg.chains.get(&2).unwrap().url,
        Some("http://localhost.toml:8546".to_string())
    );
    assert_eq!(cfg.chains.get(&2).unwrap().api_key, Some("".to_string()));

    std::env::remove_var("MOCK_TEST_CONFIG_CHAIN.CHAINS.1.URL");
    std::env::remove_var("MOCK_TEST_CONFIG_CHAIN.CHAINS.1.API_KEY");
}

#[test]
fn test_load_option_chain_from_env() {
    let path = PathBuf::from("./tests/files/config");
    let format = FileFormat::Json;
    let params = ConfigLoadOptions::builder()
        .paths(ConfigFile::builder().path(path.clone()).format(format).build())
        .env_prefix("MOCK_TEST_CONFIG_OPTION_CHAIN")
        .build();
    let cfg = load_config::<PathBuf, MockConfig>(&params).unwrap();
    assert_eq!(cfg.timeout_ms, Some(2000));
    assert_eq!(cfg.option_chains.name, Some("abc".to_string()));
    assert_eq!(cfg.option_chains.chains.len(), 2);
    assert_eq!(
        cfg.option_chains.chains.get(&3).unwrap().url,
        Some("http://localhost:8545".to_string())
    );
    assert!(cfg.option_chains.chains.get(&3).unwrap().api_key.is_none());
    assert_eq!(
        cfg.option_chains.chains.get(&4).unwrap().url,
        Some("http://localhost:8546".to_string())
    );
    assert!(cfg.option_chains.chains.get(&4).unwrap().api_key.is_none());
    assert_eq!(
        cfg.option_chains
            .chain_checker
            .as_ref()
            .unwrap()
            .actor
            .as_ref()
            .unwrap()
            .chains
            .len(),
        2
    );
    assert_eq!(
        cfg.option_chains
            .chain_checker
            .as_ref()
            .unwrap()
            .actor
            .as_ref()
            .unwrap()
            .chains
            .get(&5)
            .unwrap()
            .url,
        Some("http://localhost:8545".to_string())
    );
    assert_eq!(
        cfg.option_chains
            .chain_checker
            .as_ref()
            .unwrap()
            .actor
            .as_ref()
            .unwrap()
            .chains
            .get(&6)
            .unwrap()
            .url,
        Some("http://localhost:8546".to_string())
    );

    std::env::set_var(
        "MOCK_TEST_CONFIG_OPTION_CHAIN.OPTION_CHAINS.CHAINS.3.URL",
        "http://mock_localhost.option:8546",
    );
    std::env::set_var(
        "MOCK_TEST_CONFIG_OPTION_CHAIN.OPTION_CHAINS.CHAIN_CHECKER.ACTOR.CHAINS.5.URL",
        "http://mock_localhost.chain_checker:8546",
    );

    let params = ConfigLoadOptions::builder()
        .paths(vec![])
        .env_prefix("MOCK_TEST_CONFIG_OPTION_CHAIN")
        .build();
    let cfg = load_config::<PathBuf, MockConfig>(&params).unwrap();
    assert!(cfg.timeout_ms.is_none());
    assert_eq!(cfg.option_chains.chains.len(), 1);
    assert_eq!(
        cfg.option_chains.chains.get(&3).unwrap().url,
        Some("http://mock_localhost.option:8546".to_string())
    );
    assert!(cfg.option_chains.chains.get(&3).unwrap().api_key.is_none());
    assert_eq!(
        cfg.option_chains
            .chain_checker
            .as_ref()
            .unwrap()
            .actor
            .as_ref()
            .unwrap()
            .chains
            .len(),
        1
    );
    assert_eq!(
        cfg.option_chains
            .chain_checker
            .as_ref()
            .unwrap()
            .actor
            .as_ref()
            .unwrap()
            .chains
            .get(&5)
            .unwrap()
            .url,
        Some("http://mock_localhost.chain_checker:8546".to_string())
    );

    let params = ConfigLoadOptions::builder()
        .paths(ConfigFile::builder().path(path).format(format).build())
        .env_prefix("MOCK_TEST_CONFIG_OPTION_CHAIN")
        .build();
    let cfg = load_config::<PathBuf, MockConfig>(&params).unwrap();

    assert_eq!(cfg.timeout_ms, Some(2000));
    assert_eq!(cfg.option_chains.name, Some("abc".to_string()));
    assert_eq!(cfg.option_chains.chains.len(), 2);
    assert_eq!(
        cfg.option_chains.chains.get(&3).unwrap().url,
        Some("http://mock_localhost.option:8546".to_string())
    );
    assert!(cfg.option_chains.chains.get(&3).unwrap().api_key.is_none());
    assert_eq!(
        cfg.option_chains.chains.get(&4).unwrap().url,
        Some("http://localhost:8546".to_string())
    );
    assert!(cfg.option_chains.chains.get(&4).unwrap().api_key.is_none());
    assert_eq!(
        cfg.option_chains
            .chain_checker
            .as_ref()
            .unwrap()
            .actor
            .as_ref()
            .unwrap()
            .chains
            .len(),
        2
    );
    assert_eq!(
        cfg.option_chains
            .chain_checker
            .as_ref()
            .unwrap()
            .actor
            .as_ref()
            .unwrap()
            .chains
            .get(&5)
            .unwrap()
            .url,
        Some("http://mock_localhost.chain_checker:8546".to_string())
    );
    assert_eq!(
        cfg.option_chains
            .chain_checker
            .as_ref()
            .unwrap()
            .actor
            .as_ref()
            .unwrap()
            .chains
            .get(&6)
            .unwrap()
            .url,
        Some("http://localhost:8546".to_string())
    );

    std::env::remove_var("MOCK_TEST_CONFIG_OPTION_CHAIN.OPTION_CHAINS.CHAINS.3.URL");
    std::env::remove_var("MOCK_TEST_CONFIG_OPTION_CHAIN.OPTION_CHAINS.CHAIN_CHECKER.ACTOR.CHAINS.5.URL");
}
