use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use crate::common::{BridgeType, validate_object};
use crate::raw::base::RawConfigTrait;
use crate::raw::bridge::base::{RawBridgeConfig, RawBridgeConfigTrait};

fn default_bridge_type() -> BridgeType {
    BridgeType::Poly
}

fn validate_bridge_type(t: &BridgeType) -> Result<(), ValidationError> {
    if *t == BridgeType::Poly {
        return Ok(());
    }
    Err(ValidationError::new("bridge type error"))
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawPolyBridgeConfig {
    #[serde(flatten)]
    base: RawBridgeConfig,

    #[serde(default = "default_bridge_type")]
    #[serde(rename = "type")]
    #[validate(custom = "validate_bridge_type")]
    pub bridge_type: BridgeType,

    #[validate(url)]
    pub explorer_url: String,

    #[validate(contains = "%tx%")]
    pub explorer_prefix: String,

    #[validate(url)]
    pub api_url: String,

    #[validate(contains = "%tx%")]
    pub api_prefix: String,
}

impl RawPolyBridgeConfig {
    pub fn new(
        name: String,
        explorer_url: String,
        explorer_prefix: String,
        api_url: String,
        api_prefix: String,
    ) -> Self {
        Self {
            base: RawBridgeConfig::new(name),
            bridge_type: default_bridge_type(),
            explorer_url,
            explorer_prefix,
            api_url,
            api_prefix,
        }
    }
}

impl RawConfigTrait for RawPolyBridgeConfig {
    fn validate(&self) {
        self.base.base.validate_object(self)
    }
}

impl RawBridgeConfigTrait for RawPolyBridgeConfig {
    fn name(&self) -> &String {
        &self.base.name()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::BridgeType;
    use crate::raw::base::{RawConfig, RawConfigTrait};
    use crate::raw::bridge::poly::RawPolyBridgeConfig;
    use crate::raw::chain::{EXPLORER_DEFAULT_PREFIX};

    async fn default_config() -> RawPolyBridgeConfig {
        RawConfig::create_from_object::<RawPolyBridgeConfig>(
            RawPolyBridgeConfig::new(
                "Poly Bridge".to_string(),
                "https://explorer.poly.network".to_string(),
                "/tx/%tx%".to_string(),
                "https://explorer.poly.network".to_string(),
                "/testnet/api/v1/getcrosstx?txhash=%tx%".to_string(),
            )
        ).await
    }

    #[tokio::test]
    async fn test_validate_success() {
        let config = default_config().await;
        assert_eq!(config.bridge_type, BridgeType::Poly);
        assert_eq!(config.explorer_prefix, EXPLORER_DEFAULT_PREFIX);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_type() {
        let mut config = default_config().await;
        config.bridge_type = BridgeType::Tbridge;
        config.validate();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_explorer_url_0() {
        let mut config = default_config().await;
        config.explorer_url = String::from("");
        config.validate();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_explorer_url_1() {
        let mut config = default_config().await;
        config.explorer_url = String::from("wrong url");
        config.validate();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_explorer_prefix_0() {
        let mut config = default_config().await;
        config.explorer_prefix = String::from("");
        config.validate();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_explorer_prefix_1() {
        let mut config = default_config().await;
        config.explorer_prefix = String::from("wrong prefix");
        config.validate();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_api_url_0() {
        let mut config = default_config().await;
        config.api_url = String::from("");
        config.validate();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_api_url_1() {
        let mut config = default_config().await;
        config.api_url = String::from("wrong url");
        config.validate();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_api_prefix_0() {
        let mut config = default_config().await;
        config.api_prefix = String::from("");
        config.validate();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_api_prefix_1() {
        let mut config = default_config().await;
        config.api_prefix = String::from("wrong prefix");
        config.validate();
    }

    #[tokio::test]
    async fn test_import_valid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawPolyBridgeConfig>("src/tests/files/bridge/poly.valid.json").await;
        assert_eq!(file_config, default_config().await)
    }

    #[tokio::test]
    #[should_panic]
    async fn test_import_invalid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawPolyBridgeConfig>("src/tests/files/bridge/poly.invalid.json").await;
    }
}