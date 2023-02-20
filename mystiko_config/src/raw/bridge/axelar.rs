use std::hash::{Hash, Hasher};
use serde::{Deserialize, Deserializer, Serialize};
use validator::{Validate, ValidationError};
use crate::common::{BridgeType};
use crate::raw::base::Validator;
use crate::raw::bridge::base::{RawBridgeConfig, RawBridgeConfigTrait};

fn default_bridge_type() -> BridgeType {
    BridgeType::Axelar
}

fn validate_bridge_type(t: &BridgeType) -> Result<(), ValidationError> {
    if *t == BridgeType::Axelar {
        return Ok(());
    }
    Err(ValidationError::new("bridge type error"))
}

#[derive(Validate, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawAxelarBridgeConfig {
    #[validate]
    #[serde(flatten)]
    pub base: RawBridgeConfig,

    #[serde(rename = "type")]
    #[serde(skip_serializing)]
    #[validate(custom = "validate_bridge_type")]
    pub bridge_type: BridgeType,
}

impl RawAxelarBridgeConfig {
    pub fn new(name: String) -> Self {
        let bridge_type = default_bridge_type();
        Self {
            base: RawBridgeConfig::new(name, bridge_type.clone()),
            bridge_type,
        }
    }
}

impl Validator for RawAxelarBridgeConfig {
    fn validation(&self) {
        self.base.base.validate_object(self)
    }
}

impl RawBridgeConfigTrait for RawAxelarBridgeConfig {
    fn name(&self) -> &String {
        &self.base.name
    }

    fn bridge_type(&self) -> &BridgeType {
        &self.bridge_type
    }
}

impl Hash for RawAxelarBridgeConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bridge_type.hash(state)
    }
}

impl<'de> Deserialize<'de> for RawAxelarBridgeConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct Inner {
            #[serde(rename = "type")]
            bridge_type: Option<BridgeType>,
            name: String,
        }
        let inner = Inner::deserialize(deserializer)?;
        let bridge_type = inner.bridge_type.unwrap_or_else(|| BridgeType::Axelar);
        let base_bridge_type = bridge_type.clone();
        Ok(Self {
            base: RawBridgeConfig {
                base: Default::default(),
                bridge_type: base_bridge_type,
                name: inner.name,
            },
            bridge_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use crate::common::BridgeType;
    use crate::raw::base::{RawConfig, Validator};
    use crate::raw::bridge::axelar::RawAxelarBridgeConfig;
    use crate::raw::bridge::base::RawBridgeConfigTrait;

    async fn default_config() -> RawAxelarBridgeConfig {
        RawConfig::create_from_object::<RawAxelarBridgeConfig>(
            RawAxelarBridgeConfig::new(String::from("Axelar Bridge"))
        ).await
    }

    #[tokio::test]
    async fn test_hash() {
        let config1 = default_config().await;
        let mut hasher = DefaultHasher::new();
        config1.hash(&mut hasher);
        let hash1 = hasher.finish();

        hasher = DefaultHasher::new();
        let mut config2 = default_config().await;
        config2.bridge_type = BridgeType::Tbridge;
        config2.hash(&mut hasher);
        let hash2 = hasher.finish();

        assert_ne!(hash1, hash2);
    }

    #[tokio::test]
    async fn test_name() {
        let config = default_config().await;
        assert_eq!(config.name(), &config.base.name);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_type() {
        let mut config = default_config().await;
        config.bridge_type = BridgeType::Tbridge;
        config.validation();
    }

    #[tokio::test]
    async fn test_import_valid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawAxelarBridgeConfig>("src/tests/files/bridge/axelar.valid.json").await;
        assert_eq!(file_config, default_config().await);
        assert_eq!(file_config.bridge_type, file_config.base.bridge_type);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_import_invalid_json_file() {
        let _file_config =
            RawConfig::create_from_file::<RawAxelarBridgeConfig>("src/tests/files/bridge/axelar.invalid.json").await;
    }

    #[tokio::test]
    async fn test_import_valid_json_str() {
        let json_str = r#"{
            "name": "Axelar Bridge"
        }"#;
        let str_config =
            RawConfig::create_from_json_string::<RawAxelarBridgeConfig>(json_str).await;
        assert_eq!(str_config.bridge_type, BridgeType::Axelar);
        assert_eq!(str_config.bridge_type, str_config.base.bridge_type)
    }
}