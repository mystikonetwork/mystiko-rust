use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use crate::common::{BridgeType, validate_object};
use crate::raw::base::RawConfigTrait;
use crate::raw::bridge::base::{RawBridgeConfig, RawBridgeConfigTrait};

fn default_bridge_type() -> BridgeType {
    BridgeType::Celer
}

fn validate_bridge_type(t: &BridgeType) -> Result<(), ValidationError> {
    if *t == BridgeType::Celer {
        return Ok(());
    }
    Err(ValidationError::new("bridge type error"))
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawCelerBridgeConfig {
    #[serde(flatten)]
    pub base: RawBridgeConfig,

    #[serde(default = "default_bridge_type")]
    #[serde(rename = "type")]
    #[validate(custom = "validate_bridge_type")]
    pub bridge_type: BridgeType,
}

impl RawCelerBridgeConfig {
    pub fn new(name: String) -> Self {
        Self {
            base: RawBridgeConfig::new(name),
            bridge_type: default_bridge_type(),
        }
    }
}

impl RawConfigTrait for RawCelerBridgeConfig {
    fn validate(&self) {
        self.base.base.validate_object(self)
    }
}

impl RawBridgeConfigTrait for RawCelerBridgeConfig {
    fn name(&self) -> &String {
        &self.base.name()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::BridgeType;
    use crate::raw::base::{RawConfig, RawConfigTrait};
    use crate::raw::bridge::celer::RawCelerBridgeConfig;

    async fn default_config() -> RawCelerBridgeConfig {
        RawConfig::create_from_object::<RawCelerBridgeConfig>(
            RawCelerBridgeConfig::new(String::from("Celer Bridge"))
        ).await
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_type() {
        let mut config = default_config().await;
        config.bridge_type = BridgeType::Tbridge;
        config.validate();
    }

    #[tokio::test]
    async fn test_import_valid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawCelerBridgeConfig>("src/tests/files/bridge/celer.valid.json").await;
        assert_eq!(file_config, default_config().await);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_import_invalid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawCelerBridgeConfig>("src/tests/files/bridge/celer.invalid.json").await;
    }
}