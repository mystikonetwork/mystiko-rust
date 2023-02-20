use std::hash::{Hash, Hasher};
use serde::{Deserialize, Deserializer, Serialize};
use validator::{Validate, ValidationError};
use crate::common::{BridgeType};
use crate::raw::base::Validator;
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

#[derive(Validate, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawPolyBridgeConfig {
    #[validate]
    #[serde(flatten)]
    pub(crate) base: RawBridgeConfig,

    #[serde(rename = "type")]
    #[serde(skip_serializing)]
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
        let bridge_type = default_bridge_type();
        Self {
            base: RawBridgeConfig::new(name, bridge_type.clone()),
            bridge_type,
            explorer_url,
            explorer_prefix,
            api_url,
            api_prefix,
        }
    }
}

impl Validator for RawPolyBridgeConfig {
    fn validation(&self) {
        self.base.base.validate_object(self)
    }
}

impl RawBridgeConfigTrait for RawPolyBridgeConfig {
    fn name(&self) -> &String {
        &self.base.name
    }

    fn bridge_type(&self) -> &BridgeType {
        &self.bridge_type
    }
}

impl Hash for RawPolyBridgeConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bridge_type.hash(state)
    }
}

impl<'de> Deserialize<'de> for RawPolyBridgeConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Inner {
            #[serde(rename = "type")]
            bridge_type: Option<BridgeType>,
            name: String,
            explorer_url: String,
            explorer_prefix: String,
            api_url: String,
            api_prefix: String,
        }
        let inner = Inner::deserialize(deserializer)?;
        let bridge_type = inner.bridge_type.unwrap_or_else(|| BridgeType::Poly);
        let base_bridge_type = bridge_type.clone();
        Ok(Self {
            base: RawBridgeConfig {
                base: Default::default(),
                bridge_type: base_bridge_type,
                name: inner.name,
            },
            bridge_type,
            explorer_url: inner.explorer_url,
            explorer_prefix: inner.explorer_prefix,
            api_url: inner.api_url,
            api_prefix: inner.api_prefix,
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
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_explorer_url_0() {
        let mut config = default_config().await;
        config.explorer_url = String::from("");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_explorer_url_1() {
        let mut config = default_config().await;
        config.explorer_url = String::from("wrong url");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_explorer_prefix_0() {
        let mut config = default_config().await;
        config.explorer_prefix = String::from("");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_explorer_prefix_1() {
        let mut config = default_config().await;
        config.explorer_prefix = String::from("wrong prefix");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_api_url_0() {
        let mut config = default_config().await;
        config.api_url = String::from("");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_api_url_1() {
        let mut config = default_config().await;
        config.api_url = String::from("wrong url");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_api_prefix_0() {
        let mut config = default_config().await;
        config.api_prefix = String::from("");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_api_prefix_1() {
        let mut config = default_config().await;
        config.api_prefix = String::from("wrong prefix");
        config.validation();
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
        let _file_config =
            RawConfig::create_from_file::<RawPolyBridgeConfig>("src/tests/files/bridge/poly.invalid.json").await;
    }

    #[tokio::test]
    async fn test_import_valid_json_str() {
        let json_str = r#"{
            "name": "Poly Bridge",
            "explorerUrl": "https://explorer.poly.network",
            "explorerPrefix": "/tx/%tx%",
            "apiUrl": "https://explorer.poly.network",
            "apiPrefix": "/testnet/api/v1/getcrosstx?txhash=%tx%"
        }"#;
        let str_config =
            RawConfig::create_from_json_string::<RawPolyBridgeConfig>(json_str).await;
        assert_eq!(str_config.bridge_type, BridgeType::Poly);
        assert_eq!(str_config.bridge_type, str_config.base.bridge_type)
    }
}