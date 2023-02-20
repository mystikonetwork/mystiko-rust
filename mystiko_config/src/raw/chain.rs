use std::borrow::ToOwned;
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{AssetType, validate_object};
use crate::raw::asset::RawAssetConfig;
use crate::raw::base::{RawConfig, Validator};
use crate::raw::contract::deposit::RawDepositContractConfig;
use crate::raw::contract::pool::RawPoolContractConfig;
use crate::raw::provider::RawProviderConfig;
use crate::raw::validator::{array_unique, is_number_string, validate_nested_vec};

pub const EXPLORER_TX_PLACEHOLDER: &str = "%tx%";
//TODO use EXPLORER_TX_PLACEHOLDER replace %tx%
pub const EXPLORER_DEFAULT_PREFIX: &str = "/tx/%tx%";

fn default_event_filter_size() -> u32 {
    200000
}

fn default_indexer_filter_size() -> u32 {
    500000
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawChainConfig {
    #[serde(default)]
    pub base: RawConfig,

    #[validate(range(min = 1))]
    pub chain_id: u32,

    #[validate(length(min = 1))]
    pub name: String,

    #[validate(length(min = 1))]
    pub asset_symbol: String,

    #[validate(range(min = 1))]
    pub asset_decimals: u32,

    #[validate(
    custom(function = "array_unique"),
    custom(function = "is_number_string::<true, true>")
    )]
    #[serde(default)]
    pub recommended_amounts: Vec<String>,

    #[validate(url)]
    pub explorer_url: String,

    #[validate(contains = "%tx%")]
    pub explorer_prefix: String,

    #[validate(length(min = 1))]
    #[validate(custom = "validate_nested_vec")]
    pub providers: Vec<RawProviderConfig>,

    #[validate(url)]
    pub signer_endpoint: String,

    #[validate(range(min = 1))]
    #[serde(default = "default_event_filter_size")]
    pub event_filter_size: u32,

    #[validate(range(min = 1))]
    #[serde(default = "default_indexer_filter_size")]
    pub indexer_filter_size: u32,

    #[validate(custom(function = "array_unique"))]
    #[validate(custom = "validate_nested_vec")]
    pub deposit_contracts: Vec<RawDepositContractConfig>,

    #[validate(custom(function = "array_unique"))]
    #[validate(custom = "validate_nested_vec")]
    pub pool_contracts: Vec<RawPoolContractConfig>,

    #[validate(custom(function = "array_unique"))]
    #[validate(custom = "validate_nested_vec")]
    pub assets: Vec<RawAssetConfig>,
}

impl PartialEq for RawChainConfig {
    fn eq(&self, other: &Self) -> bool {
        self.chain_id == other.chain_id
    }
}

impl Hash for RawChainConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.chain_id.hash(state)
    }
}

impl Validator for RawChainConfig {
    fn validation(&self) {
        self.base.validate_object::<&RawChainConfig>(self)
    }
}

impl RawChainConfig {
    pub fn new(
        chain_id: u32,
        name: String,
        asset_symbol: String,
        asset_decimals: u32,
        recommended_amounts: Vec<String>,
        explorer_url: String,
        explorer_prefix: String,
        event_filter_size: u32,
        indexer_filter_size: u32,
        providers: Vec<RawProviderConfig>,
        signer_endpoint: String,
        deposit_contracts: Vec<RawDepositContractConfig>,
        pool_contracts: Vec<RawPoolContractConfig>,
        assets: Vec<RawAssetConfig>,
    ) -> RawChainConfig {
        Self {
            base: RawConfig::default(),
            chain_id,
            name,
            asset_symbol,
            asset_decimals,
            recommended_amounts,
            explorer_url,
            explorer_prefix,
            providers,
            signer_endpoint,
            event_filter_size,
            indexer_filter_size,
            deposit_contracts,
            pool_contracts,
            assets,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use crate::common::{AssetType, BridgeType, ContractType};
    use crate::raw::asset::RawAssetConfig;
    use crate::raw::base::{RawConfig, Validator};
    use crate::raw::chain::RawChainConfig;
    use crate::raw::contract::base::RawContractConfig;
    use crate::raw::contract::deposit::RawDepositContractConfig;
    use crate::raw::contract::pool::RawPoolContractConfig;
    use crate::raw::provider::RawProviderConfig;

    async fn init_provider_config() -> RawProviderConfig {
        RawConfig::create_from_object::<RawProviderConfig>(
            RawProviderConfig::new(
                "wss://ropsten.infura.io/ws/v3/9aa3d95b3bc440fa88ea12eaa4456161".to_string(),
                5000,
                5,
            )
        ).await
    }

    async fn init_deposit_contract_config() -> RawDepositContractConfig {
        RawConfig::create_from_object::<RawDepositContractConfig>(
            RawDepositContractConfig::new(
                RawContractConfig::new(
                    2,
                    "MystikoWithPolyERC20".to_string(),
                    "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67".to_string(),
                    ContractType::Deposit,
                    1000000,
                    None,
                    None,
                ),
                BridgeType::Tbridge,
                "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d".to_string(),
                true,
                Some(97),
                Some(String::from("0x98bF2d9e3bA2A8515E660BD4104432ce3e2D7547")),
                "10000000000000000".to_string(),
                "100000000000000000".to_string(),
                "20000000000000000".to_string(),
                "30000000000000000".to_string(),
                None,
                None,
                None,
                None,
            )
        ).await
    }

    async fn init_pool_contract_config() -> RawPoolContractConfig {
        RawConfig::create_from_object::<RawPoolContractConfig>(
            RawPoolContractConfig::new(
                RawContractConfig::new(
                    2,
                    "CommitmentPool".to_string(),
                    "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d".to_string(),
                    ContractType::Pool,
                    1000000,
                    None,
                    None,
                ),
                "A Pool(since 07/20/2022)".to_string(),
                BridgeType::Tbridge,
                Some(String::from("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")),
                "40000000000000000".to_string(),
                vec![String::from("circuit-1.0")],
            )
        ).await
    }

    async fn init_assets_config() -> RawAssetConfig {
        RawConfig::create_from_object::<RawAssetConfig>(
            RawAssetConfig::new(
                AssetType::Erc20,
                "MTT".to_string(),
                16,
                "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string(),
                vec![],
            )
        ).await
    }

    async fn default_config() -> RawChainConfig {
        let provider_config = init_provider_config().await;
        let deposit_contract_config = init_deposit_contract_config().await;
        let pool_contract_config = init_pool_contract_config().await;
        let asset_config = init_assets_config().await;
        RawConfig::create_from_object::<RawChainConfig>(
            RawChainConfig::new(
                3,
                "Ethereum Ropsten".to_string(),
                "ETH".to_string(),
                18,
                vec![
                    "1000000000000000000".to_string(),
                    "10000000000000000000".to_string(),
                ],
                "https://ropsten.etherscan.io".to_string(),
                "/tx/%tx%".to_string(),
                1000,
                10000,
                vec![provider_config],
                "https://ropsten.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161".to_string(),
                vec![deposit_contract_config],
                vec![pool_contract_config],
                vec![asset_config],
            )
        ).await
    }

    #[tokio::test]
    async fn test_eq() {
        let config1 = default_config().await;
        let mut config2 = default_config().await;
        let mut config3 = default_config().await;
        config2.name = "new name".to_string();
        config3.chain_id = 97;
        assert_eq!(config1, config2);
        assert_ne!(config1, config3);
    }

    #[tokio::test]
    async fn test_hash() {
        let config1 = default_config().await;
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
    #[should_panic]
    async fn test_invalid_name() {
        let mut config = default_config().await;
        config.name = String::from("");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_asset_symbol() {
        let mut config = default_config().await;
        config.asset_symbol = String::from("");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_asset_decimals() {
        let mut config = default_config().await;
        config.asset_decimals = 0;
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_recommended_amounts_0() {
        let mut config = default_config().await;
        config.recommended_amounts = vec![String::from("")];
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_recommended_amounts_1() {
        let mut config = default_config().await;
        config.recommended_amounts = vec![String::from("abcd")];
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_recommended_amounts_2() {
        let mut config = default_config().await;
        config.recommended_amounts = vec![String::from("1"), String::from("1")];
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_explore_url_0() {
        let mut config = default_config().await;
        config.explorer_url = String::from("");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_explore_url_1() {
        let mut config = default_config().await;
        config.explorer_url = String::from("wrong url");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_explore_prefix_0() {
        let mut config = default_config().await;
        config.explorer_prefix = String::from("");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_explore_prefix_1() {
        let mut config = default_config().await;
        config.explorer_prefix = String::from("wrong prefix");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_providers_0() {
        let mut config = default_config().await;
        config.providers = vec![];
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_providers_1() {
        let mut config = default_config().await;
        let mut provider_config = config.providers[0].clone();
        provider_config.url = String::from("wrong url");
        config.providers = vec![provider_config];
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_signer_endpoint_0() {
        let mut config = default_config().await;
        config.signer_endpoint = String::from("");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_signer_endpoint_1() {
        let mut config = default_config().await;
        config.signer_endpoint = String::from("wrong url");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_signer_endpoint_2() {
        let mut config = default_config().await;
        config.signer_endpoint = String::from("wrong_schema://127.0.0.1");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_event_filter_size() {
        let mut config = default_config().await;
        config.event_filter_size = 0;
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_indexer_filter_size() {
        let mut config = default_config().await;
        config.indexer_filter_size = 0;
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_pool_contracts_0() {
        let mut config = default_config().await;
        config.pool_contracts.push(init_pool_contract_config().await);
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_pool_contracts_1() {
        let mut config = default_config().await;
        let mut pool_contract = config.pool_contracts[0].clone();
        pool_contract.asset_address = Some(String::from("0xdeadbeef"));
        config.pool_contracts = vec![pool_contract];
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_deposit_contracts() {
        let mut config = default_config().await;
        config.deposit_contracts.push(init_deposit_contract_config().await);
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_assets() {
        let mut config = default_config().await;
        config.assets.push(init_assets_config().await);
        config.validation();
    }

    #[tokio::test]
    async fn test_import_valid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawChainConfig>("src/tests/files/chain.valid.json").await;
        assert_eq!(file_config, default_config().await)
    }

    #[tokio::test]
    #[should_panic]
    async fn test_import_invalid_json_file() {
        let _file_config =
            RawConfig::create_from_file::<RawChainConfig>("src/tests/files/chain.invalid.json").await;
    }
}