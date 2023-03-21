use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::common::{AssetType, BridgeType, ContractType};
use mystiko_config::raw::asset::RawAssetConfig;
use mystiko_config::raw::base::{RawConfig, Validator};
use mystiko_config::raw::chain::RawChainConfig;
use mystiko_config::raw::contract::base::RawContractConfig;
use mystiko_config::raw::contract::deposit::RawDepositContractConfig;
use mystiko_config::raw::contract::pool::RawPoolContractConfig;
use mystiko_config::raw::provider::RawProviderConfig;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

async fn init_provider_config() -> RawProviderConfig {
    RawConfig::from_object::<RawProviderConfig>(
        RawProviderConfig::builder()
            .url("wss://ropsten.infura.io/ws/v3/9aa3d95b3bc440fa88ea12eaa4456161".to_string())
            .timeout_ms(5000)
            .max_try_count(5)
            .build(),
    )
    .unwrap()
}

async fn init_deposit_contract_config() -> RawDepositContractConfig {
    let raw_deposit_contract_config = RawDepositContractConfig::builder()
        .base(
            RawContractConfig::builder()
                .version(2)
                .name("MystikoWithPolyERC20".to_string())
                .address("0x961f315a836542e603a3df2e0dd9d4ecd06ebc67".to_string())
                .contract_type(ContractType::Deposit)
                .start_block(1000000)
                .build(),
        )
        .bridge_type(BridgeType::Tbridge)
        .pool_address("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d".to_string())
        .disabled(true)
        .peer_chain_id(Some(97))
        .peer_contract_address(Some(
            "0x98bF2d9e3bA2A8515E660BD4104432ce3e2D7547".to_string(),
        ))
        .min_amount("10000000000000000".to_string())
        .max_amount("100000000000000000".to_string())
        .min_bridge_fee("20000000000000000".to_string())
        .min_executor_fee("30000000000000000".to_string())
        .build();
    RawConfig::from_object::<RawDepositContractConfig>(raw_deposit_contract_config).unwrap()
}

async fn init_pool_contract_config() -> RawPoolContractConfig {
    RawConfig::from_object::<RawPoolContractConfig>(
        RawPoolContractConfig::builder()
            .base(
                RawContractConfig::builder()
                    .version(2)
                    .name("CommitmentPool".to_string())
                    .address("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d".to_string())
                    .contract_type(ContractType::Pool)
                    .start_block(1000000)
                    .build(),
            )
            .pool_name("A Pool(since 07/20/2022)".to_string())
            .bridge_type(BridgeType::Tbridge)
            .asset_address(Some(String::from(
                "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a",
            )))
            .min_rollup_fee("40000000000000000".to_string())
            .circuits(vec![String::from("circuit-1.0")])
            .build(),
    )
    .unwrap()
}

async fn init_assets_config() -> RawAssetConfig {
    RawConfig::from_object::<RawAssetConfig>(
        RawAssetConfig::builder()
            .asset_type(AssetType::Erc20)
            .asset_symbol("MTT".to_string())
            .asset_decimals(16)
            .asset_address("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string())
            .build(),
    )
    .unwrap()
}

async fn default_config() -> RawChainConfig {
    let provider_config = init_provider_config().await;
    let deposit_contract_config = init_deposit_contract_config().await;
    let pool_contract_config = init_pool_contract_config().await;
    let asset_config = init_assets_config().await;
    let raw_chain_config = RawChainConfig::builder()
        .chain_id(3)
        .name("Ethereum Ropsten".to_string())
        .asset_symbol("ETH".to_string())
        .asset_decimals(18)
        .recommended_amounts(vec![
            "1000000000000000000".to_string(),
            "10000000000000000000".to_string(),
        ])
        .explorer_url("https://ropsten.etherscan.io".to_string())
        .explorer_prefix("/tx/%tx%".to_string())
        .providers(vec![provider_config])
        .signer_endpoint(
            "https://ropsten.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161".to_string(),
        )
        .deposit_contracts(vec![deposit_contract_config])
        .pool_contracts(vec![pool_contract_config])
        .assets(vec![asset_config])
        .build();
    RawConfig::from_object::<RawChainConfig>(raw_chain_config).unwrap()
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<RawChainConfig> =
        AsyncOnce::new(async { default_config().await });
}

#[tokio::test]
async fn test_eq() {
    let config1 = CONFIG_CREATER.get().await;
    let mut config2 = default_config().await;
    let mut config3 = default_config().await;
    config2.name = "new name".to_string();
    config3.chain_id = 97;
    assert_eq!(config1, &config2);
    assert_ne!(config1, &config3);
}

#[tokio::test]
async fn test_hash() {
    let config1 = CONFIG_CREATER.get().await;
    let mut hasher = DefaultHasher::new();
    config1.hash(&mut hasher);
    let hash1 = hasher.finish();

    hasher = DefaultHasher::new();
    let mut config2 = default_config().await;
    config2.chain_id = 97;
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
async fn test_invalid_asset_symbol() {
    let mut config = default_config().await;
    config.asset_symbol = String::from("");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_asset_decimals() {
    let mut config = default_config().await;
    config.asset_decimals = 0;
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_recommended_amounts_0() {
    let mut config = default_config().await;
    config.recommended_amounts = vec![String::from("")];
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_recommended_amounts_1() {
    let mut config = default_config().await;
    config.recommended_amounts = vec![String::from("abcd")];
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_recommended_amounts_2() {
    let mut config = default_config().await;
    config.recommended_amounts = vec![String::from("1"), String::from("1")];
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_explore_url_0() {
    let mut config = default_config().await;
    config.explorer_url = String::from("");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_explore_url_1() {
    let mut config = default_config().await;
    config.explorer_url = String::from("wrong url");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_explore_prefix_0() {
    let mut config = default_config().await;
    config.explorer_prefix = String::from("");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_explore_prefix_1() {
    let mut config = default_config().await;
    config.explorer_prefix = String::from("wrong prefix");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_providers_0() {
    let mut config = default_config().await;
    config.providers = vec![];
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_providers_1() {
    let mut config = default_config().await;
    let mut provider_config = config.providers[0].clone();
    provider_config.url = String::from("wrong url");
    config.providers = vec![provider_config];
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_signer_endpoint_0() {
    let mut config = default_config().await;
    config.signer_endpoint = String::from("");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_signer_endpoint_1() {
    let mut config = default_config().await;
    config.signer_endpoint = String::from("wrong url");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_signer_endpoint_2() {
    let mut config = default_config().await;
    config.signer_endpoint = String::from("wrong_schema://127.0.0.1");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_event_filter_size() {
    let mut config = default_config().await;
    config.event_filter_size = 0;
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_indexer_filter_size() {
    let mut config = default_config().await;
    config.indexer_filter_size = 0;
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_pool_contracts_0() {
    let mut config = default_config().await;
    config
        .pool_contracts
        .push(init_pool_contract_config().await);
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_pool_contracts_1() {
    let mut config = default_config().await;
    let mut pool_contract = config.pool_contracts[0].clone();
    pool_contract.asset_address = Some(String::from("0xdeadbeef"));
    config.pool_contracts = vec![pool_contract];
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_deposit_contracts() {
    let mut config = default_config().await;
    config
        .deposit_contracts
        .push(init_deposit_contract_config().await);
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_assets() {
    let mut config = default_config().await;
    config.assets.push(init_assets_config().await);
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config = RawConfig::from_file::<RawChainConfig>("tests/files/chain.valid.json")
        .await
        .unwrap();
    assert_eq!(file_config, default_config().await)
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        RawConfig::from_file::<RawChainConfig>("tests/files/chain.invalid.json").await;
    assert!(file_config.is_err());
}
