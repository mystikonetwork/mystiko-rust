use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use crate::common::{BridgeType, validate_object};
use crate::raw::base::RawConfigTrait;
use crate::raw::bridge::base::{RawBridgeConfig, RawBridgeConfigTrait};

fn default_bridge_type() -> BridgeType {
    BridgeType::Tbridge
}

fn validate_bridge_type(t: &BridgeType) -> Result<(), ValidationError> {
    if *t == BridgeType::Tbridge {
        return Ok(());
    }
    Err(ValidationError::new("bridge type error"))
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawTBridgeConfig {
    #[serde(flatten)]
    base: RawBridgeConfig,

    #[serde(default = "default_bridge_type")]
    #[serde(rename = "type")]
    #[validate(custom = "validate_bridge_type")]
    pub bridge_type: BridgeType,
}

impl RawTBridgeConfig {
    pub fn new(name: String) -> Self {
        Self {
            base: RawBridgeConfig::new(name),
            bridge_type: default_bridge_type(),
        }
    }
}

impl RawConfigTrait for RawTBridgeConfig {
    fn validate(&self) {
        self.base.base.validate_object(self)
    }
}

impl RawBridgeConfigTrait for RawTBridgeConfig {
    fn name(&self) -> &String {
        &self.base.name()
    }
}

impl Hash for RawTBridgeConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bridge_type.hash(state)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::BridgeType;
    use crate::raw::base::{RawConfig, RawConfigTrait};
    use crate::raw::bridge::tbridge::RawTBridgeConfig;

    async fn default_config() -> RawTBridgeConfig {
        RawConfig::create_from_object::<RawTBridgeConfig>(
            RawTBridgeConfig::new(String::from("Mystiko Testnet Bridge"))
        ).await
    }

    #[tokio::test]
    async fn test_validate_success() {
        let config = default_config().await;
        assert_eq!(config.bridge_type, BridgeType::Tbridge);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_type() {
        let mut config = default_config().await;
        config.bridge_type = BridgeType::Poly;
        config.validate();
    }

    #[tokio::test]
    async fn test_import_valid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawTBridgeConfig>("src/tests/files/bridge/tbridge.valid.json").await;
        assert_eq!(file_config, default_config().await);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_import_invalid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawTBridgeConfig>("src/tests/files/bridge/tbridge.invalid.json").await;
    }
}