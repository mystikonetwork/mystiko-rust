use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{ContractType};
use crate::raw::base::{RawConfig, Validator};
use crate::raw::validator::{is_ethereum_address};

pub trait RawContractConfigTrait: Validator {
    fn version(&self) -> &u32;
    fn name(&self) -> &str;
    fn address(&self) -> &str;
    fn contract_type(&self) -> &ContractType;
    fn start_block(&self) -> &u32;
    fn event_filter_size(&self) -> &Option<u64>;
    fn indexer_filter_size(&self) -> &Option<u64>;
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawContractConfig {
    #[serde(default)]
    pub base: RawConfig,

    #[validate(range(min = 1))]
    pub version: u32,

    #[validate(length(min = 1))]
    pub name: String,

    #[validate(custom = "is_ethereum_address")]
    pub address: String,

    #[serde(rename = "type")]
    pub contract_type: ContractType,

    #[validate(range(min = 1))]
    pub start_block: u32,

    #[validate(range(min = 1))]
    pub event_filter_size: Option<u64>,

    #[validate(range(min = 1))]
    pub indexer_filter_size: Option<u64>,
}

impl RawContractConfig {
    pub fn new(
        version: u32,
        name: String,
        address: String,
        contract_type: ContractType,
        start_block: u32,
        event_filter_size: Option<u64>,
        indexer_filter_size: Option<u64>,
    ) -> Self {
        Self {
            base: RawConfig::default(),
            version,
            name,
            address,
            contract_type,
            start_block,
            event_filter_size,
            indexer_filter_size,
        }
    }
}

impl Validator for RawContractConfig {
    fn validation(&self) {
        self.base.validate_object(self)
    }
}

impl RawContractConfigTrait for RawContractConfig {
    fn version(&self) -> &u32 {
        &self.version
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn address(&self) -> &str {
        &self.address
    }

    fn contract_type(&self) -> &ContractType {
        &self.contract_type
    }

    fn start_block(&self) -> &u32 {
        &self.start_block
    }

    fn event_filter_size(&self) -> &Option<u64> {
        &self.event_filter_size
    }

    fn indexer_filter_size(&self) -> &Option<u64> {
        &self.indexer_filter_size
    }
}

#[cfg(test)]
mod tests {
    use crate::common::ContractType;
    use crate::raw::base::{RawConfig, Validator};
    use crate::raw::contract::base::RawContractConfig;

    async fn default_config() -> RawContractConfig {
        RawConfig::create_from_object::<RawContractConfig>(
            RawContractConfig::new(
                2,
                "MystikoWithPolyERC20".to_string(),
                "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67".to_string(),
                ContractType::Deposit,
                1000000,
                Some(10000),
                Some(100000),
            )
        ).await
    }

    #[tokio::test]
    async fn test_validate_success() {
        let mut config = default_config().await;
        config.event_filter_size = None;
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_version() {
        let mut config = default_config().await;
        config.version = 0;
        config.validation();
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
    async fn test_invalid_address_0() {
        let mut config = default_config().await;
        config.address = String::from("0xdeadbeef");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_address_1() {
        let mut config = default_config().await;
        config.address = String::from("");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_start_block() {
        let mut config = default_config().await;
        config.start_block = 0;
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_event_filter_size() {
        let mut config = default_config().await;
        config.event_filter_size = Some(0);
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_indexer_filter_size() {
        let mut config = default_config().await;
        config.indexer_filter_size = Some(0);
        config.validation();
    }

    #[tokio::test]
    async fn test_import_valid_json_file() {
        let default_config = default_config().await;
        let file_config =
            RawConfig::create_from_file::<RawContractConfig>("src/tests/files/contract/base.valid.json").await;
        assert_eq!(file_config, default_config);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_import_invalid_json_file() {
        let _file_config =
            RawConfig::create_from_file::<RawContractConfig>("src/tests/files/contract/base.invalid.json").await;
    }
}
