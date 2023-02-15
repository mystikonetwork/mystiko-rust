use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use crate::common::{BridgeType, ContractType, validate_object};
use crate::raw::base::RawConfigTrait;
use crate::raw::chain::RawChainConfig;
use crate::raw::contract::base::{RawContractConfig, RawContractConfigTrait};
use crate::raw::validator::{is_ethereum_address, is_number_string, string_vec_each_not_empty};

fn default_contract_type() -> ContractType {
    ContractType::Pool
}

fn validate_contract_type(t: &ContractType) -> Result<(), ValidationError> {
    if *t == ContractType::Pool {
        return Ok(());
    }
    Err(ValidationError::new("contract type error"))
}

fn default_min_rollup_fee() -> String {
    String::from("0")
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawPoolContractConfig {
    #[serde(flatten)]
    pub base: RawContractConfig,

    #[serde(default = "default_contract_type")]
    #[serde(rename = "type")]
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
            contract_type: default_contract_type(),
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

impl RawConfigTrait for RawPoolContractConfig {
    fn validation(&self) {
        self.base.base.validate_object(self)
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

    fn event_filter_size(&self) -> &Option<u32> {
        &self.base.event_filter_size
    }

    fn indexer_filter_size(&self) -> &Option<u32> {
        &self.base.indexer_filter_size
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{BridgeType, ContractType};
    use crate::raw::base::{RawConfig, RawConfigTrait};
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
        assert_eq!(file_config, default_config().await)
    }

    #[tokio::test]
    #[should_panic]
    async fn test_import_invalid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawPoolContractConfig>("src/tests/files/contract/pool.invalid.json").await;
    }
}
