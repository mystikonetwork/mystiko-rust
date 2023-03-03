use std::hash::{Hash, Hasher};
use serde::{Deserialize, Deserializer, Serialize};
use validator::{Validate, ValidationError};
use crate::common::{BridgeType, ContractType};
use crate::raw::base::Validator;
use crate::raw::contract::base::{RawContractConfig, RawContractConfigTrait};
use crate::raw::validator::{is_ethereum_address, is_number_string, string_vec_each_not_empty};

fn validate_contract_type(t: &ContractType) -> Result<(), ValidationError> {
    if *t == ContractType::Pool {
        return Ok(());
    }
    Err(ValidationError::new("contract type error"))
}

fn default_min_rollup_fee() -> String {
    String::from("0")
}

#[derive(Validate, Serialize, Debug, Clone, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawPoolContractConfig {
    #[serde(flatten)]
    pub base: RawContractConfig,

    #[serde(rename = "type")]
    #[serde(skip_serializing)]
    #[validate(custom = "validate_contract_type")]
    pub contract_type: ContractType,

    #[validate(length(min = 1))]
    pub pool_name: String,

    pub bridge_type: BridgeType,

    #[validate(custom = "is_ethereum_address")]
    pub asset_address: Option<String>,

    #[validate(custom = "is_number_string::<true,false>")]
    #[serde(default = "default_min_rollup_fee")]
    pub min_rollup_fee: String,

    #[validate(custom = "string_vec_each_not_empty")]
    #[serde(default)]
    pub circuits: Vec<String>,
}

impl RawPoolContractConfig {
    pub fn new(
        base: RawContractConfig,
        pool_name: String,
        bridge_type: BridgeType,
        asset_address: Option<String>,
        min_rollup_fee: String,
        circuits: Vec<String>,
    ) -> Self {
        Self {
            base,
            pool_name,
            bridge_type,
            asset_address,
            min_rollup_fee,
            circuits,
            contract_type: ContractType::Pool,
        }
    }
}

impl Hash for RawPoolContractConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.address.hash(state)
    }
}

impl PartialEq for RawPoolContractConfig {
    fn eq(&self, other: &Self) -> bool {
        self.base.address == other.base.address
    }
}

impl Validator for RawPoolContractConfig {
    fn validation(&self) {
        self.base.base.validate_object(self)
    }
}

impl<'de> Deserialize<'de> for RawPoolContractConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Inner {
            version: u32,
            name: String,
            address: String,
            #[serde(rename = "type")]
            contract_type: Option<ContractType>,
            start_block: u32,
            even_filter_size: Option<u64>,
            indexer_filter_size: Option<u64>,
            pool_name: String,
            bridge_type: BridgeType,
            asset_address: Option<String>,
            min_rollup_fee: Option<String>,
            circuits: Option<Vec<String>>,
        }
        let inner = Inner::deserialize(deserializer)?;
        let contract_type = inner.contract_type.unwrap_or_else(|| ContractType::Pool);
        let base_contract_type = contract_type.clone();
        let min_rollup_fee = inner.min_rollup_fee.unwrap_or_else(|| String::from("0"));
        let circuits = inner.circuits.unwrap_or_else(|| vec![]);
        Ok(Self {
            base: RawContractConfig {
                base: Default::default(),
                version: inner.version,
                name: inner.name,
                address: inner.address,
                contract_type: base_contract_type,
                start_block: inner.start_block,
                event_filter_size: inner.even_filter_size,
                indexer_filter_size: inner.indexer_filter_size,
            },
            contract_type,
            pool_name: inner.pool_name,
            bridge_type: inner.bridge_type,
            asset_address: inner.asset_address,
            min_rollup_fee,
            circuits,
        })
    }
}

impl RawContractConfigTrait for RawPoolContractConfig {
    fn version(&self) -> &u32 {
        &self.base.version
    }

    fn name(&self) -> &str {
        &self.base.name
    }

    fn address(&self) -> &str {
        &self.base.address
    }

    fn contract_type(&self) -> &ContractType {
        &self.contract_type
    }

    fn start_block(&self) -> &u32 {
        &self.base.start_block
    }

    fn event_filter_size(&self) -> &Option<u64> {
        &self.base.event_filter_size
    }

    fn indexer_filter_size(&self) -> &Option<u64> {
        &self.base.indexer_filter_size
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{BridgeType, ContractType};
    use crate::raw::base::{RawConfig, Validator};
    use crate::raw::contract::base::{RawContractConfig, RawContractConfigTrait};
    use crate::raw::contract::pool::{default_min_rollup_fee, RawPoolContractConfig};

    #[test]
    fn test_default_min_rollup_fee() {
        assert_eq!(default_min_rollup_fee(), "0".to_string())
    }

    async fn default_config() -> RawPoolContractConfig {
        RawConfig::create_from_object::<RawPoolContractConfig>(
            RawPoolContractConfig::new(
                RawContractConfig::new(
                    2,
                    "CommitmentPool".to_string(),
                    "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67".to_string(),
                    ContractType::Pool,
                    1000000,
                    None,
                    None,
                ),
                "A Pool(since 07/20/2022)".to_string(),
                BridgeType::Tbridge,
                Some("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string()),
                "120000000000000000".to_string(),
                vec![String::from("circuit-1.0")],
            )
        ).await
    }

    #[tokio::test]
    async fn test_raw_contract_config_trait() {
        let config = default_config().await;
        assert_eq!(config.version(), &config.base.version);
        assert_eq!(config.name(), config.base.name);
        assert_eq!(config.address(), config.base.address);
        assert_eq!(config.contract_type(), &config.contract_type);
        assert_eq!(config.start_block(), &config.base.start_block);
        assert_eq!(config.event_filter_size(), &config.base.event_filter_size);
        assert_eq!(config.indexer_filter_size(), &config.base.indexer_filter_size);
    }

    #[tokio::test]
    async fn test_validate_success() {
        let mut config = default_config().await;
        config.asset_address = None;
        config.min_rollup_fee = "0".to_string();
        config.circuits = vec![];
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_pool_name() {
        let mut config = default_config().await;
        config.pool_name = "".to_string();
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_contract_type() {
        let mut config = default_config().await;
        config.contract_type = ContractType::Deposit;
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_min_rollup_fee_0() {
        let mut config = default_config().await;
        config.min_rollup_fee = String::from("");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_min_rollup_fee_1() {
        let mut config = default_config().await;
        config.min_rollup_fee = String::from("0xdeadbeef");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_min_rollup_fee_2() {
        let mut config = default_config().await;
        config.min_rollup_fee = String::from("-1");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_min_rollup_fee_3() {
        let mut config = default_config().await;
        config.min_rollup_fee = String::from("1.2");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_circuits() {
        let mut config = default_config().await;
        config.circuits = vec![String::from("")];
        config.validation();
    }

    #[tokio::test]
    async fn test_import_valid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawPoolContractConfig>("src/tests/files/contract/pool.valid.json").await;
        assert_eq!(file_config, default_config().await);
        assert_eq!(file_config.contract_type, file_config.base.contract_type);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_import_invalid_json_file() {
        let _file_config =
            RawConfig::create_from_file::<RawPoolContractConfig>("src/tests/files/contract/pool.invalid.json").await;
    }

    #[tokio::test]
    async fn test_import_valid_json_str() {
        let json_str = r#"
            {
              "version": 2,
              "name": "CommitmentPool",
              "poolName": "A Pool(since 07/20/2022)",
              "bridgeType": "tbridge",
              "address": "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67",
              "startBlock": 1000000,
              "assetAddress": "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a",
              "minRollupFee": "120000000000000000",
              "circuits": ["circuit-1.0"]
            }
        "#;
        let str_config =
            RawConfig::create_from_json_string::<RawPoolContractConfig>(json_str).await;
        assert_eq!(str_config.contract_type, ContractType::Pool);
        assert_eq!(str_config.contract_type, str_config.base.contract_type);
    }
}
