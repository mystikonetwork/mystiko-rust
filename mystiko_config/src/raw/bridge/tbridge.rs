use std::hash::{Hash, Hasher};
use serde::{Deserialize, Deserializer, Serialize};
use validator::{Validate, ValidationError};
use crate::common::{BridgeType};
use crate::raw::base::Validator;
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

#[derive(Validate, Serialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RawTBridgeConfig {
    #[validate]
    #[serde(flatten)]
    pub base: RawBridgeConfig,

    #[serde(rename = "type")]
    #[serde(skip_serializing)]
    #[validate(custom = "validate_bridge_type")]
    pub bridge_type: BridgeType,
}

impl RawTBridgeConfig {
    pub fn new(name: String) -> Self {
        let bridge_type = default_bridge_type();
        Self {
            base: RawBridgeConfig::new(name, bridge_type.clone()),
            bridge_type,
        }
    }
}

impl Validator for RawTBridgeConfig {
    fn validation(&self) {
        self.base.base.validate_object(self);
    }
}

impl RawBridgeConfigTrait for RawTBridgeConfig {
    fn name(&self) -> &String {
        &self.base.name
    }

    fn bridge_type(&self) -> &BridgeType {
        &self.bridge_type
    }
}

impl Hash for RawTBridgeConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bridge_type.hash(state)
    }
}

impl<'de> Deserialize<'de> for RawTBridgeConfig {
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
        let bridge_type = inner.bridge_type.unwrap_or_else(|| BridgeType::Tbridge);
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
    use crate::raw::bridge::base::RawBridgeConfigTrait;
    use crate::raw::bridge::tbridge::RawTBridgeConfig;

    async fn default_config() -> RawTBridgeConfig {
        RawConfig::create_from_object::<RawTBridgeConfig>(
            RawTBridgeConfig::new(String::from("Mystiko Testnet Bridge"))
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
        config2.bridge_type = BridgeType::Poly;
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
    async fn test_validate_success() {
        let config = default_config().await;
        assert_eq!(config.bridge_type, BridgeType::Tbridge);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_name() {
        let mut config = default_config().await;
        config.base.name = "".to_string();
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_type() {
        let mut config = default_config().await;
        config.bridge_type = BridgeType::Poly;
        config.validation();
    }

    #[tokio::test]
    async fn test_import_valid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawTBridgeConfig>("src/tests/files/bridge/tbridge.valid.json").await;
        assert_eq!(file_config, default_config().await);
        assert_eq!(file_config.bridge_type, file_config.base.bridge_type);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_import_invalid_json_file() {
        let _file_config =
            RawConfig::create_from_file::<RawTBridgeConfig>("src/tests/files/bridge/tbridge.invalid.json").await;
    }

    #[tokio::test]
    async fn test_import_valid_json_str() {
        let json_str = r#"{
            "name": "Mystiko Testnet Bridge"
        }"#;
        let str_config =
            RawConfig::create_from_json_string::<RawTBridgeConfig>(json_str).await;
        assert_eq!(str_config.bridge_type, BridgeType::Tbridge);
        assert_eq!(str_config.bridge_type, str_config.base.bridge_type)
    }
}