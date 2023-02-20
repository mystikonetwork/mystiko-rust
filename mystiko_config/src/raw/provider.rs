use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::validate_object;
use crate::raw::base::{RawConfig, Validator};

fn default_timeout_ms() -> u32 {
    5000
}

fn default_max_try_count() -> u32 {
    2
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawProviderConfig {
    #[serde(default)]
    pub base: RawConfig,

    #[validate(url)]
    pub url: String,

    #[validate(range(min = 1))]
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u32,

    #[validate(range(min = 1))]
    #[serde(default = "default_max_try_count")]
    pub max_try_count: u32,
}

impl RawProviderConfig {
    pub fn new(url: String, timeout_ms: u32, max_try_count: u32) -> Self {
        Self { base: RawConfig::default(), url, timeout_ms, max_try_count }
    }
}

impl Validator for RawProviderConfig {
    fn validation(&self) {
        self.base.validate_object(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::base::{RawConfig, Validator};
    use crate::raw::provider::RawProviderConfig;

    async fn default_config() -> RawProviderConfig {
        RawConfig::create_from_object::<RawProviderConfig>(
            RawProviderConfig::new("http://localhost:8545".to_string(), 100000, 5)
        ).await
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_url_0() {
        let mut config = default_config().await;
        config.url = "".to_string();
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_url_1() {
        let mut config = default_config().await;
        config.url = "not even a url".to_string();
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_url_2() {
        let mut config = default_config().await;
        config.url = "wrong_schema://localhost:8545".to_string();
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_timeout_ms() {
        let mut config = default_config().await;
        config.timeout_ms = 0;
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_max_try_count() {
        let mut config = default_config().await;
        config.max_try_count = 0;
        config.validation();
    }

    #[tokio::test]
    async fn test_import_valid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawProviderConfig>("src/tests/files/provider.valid.json").await;
        assert_eq!(file_config, default_config().await);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_import_invalid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawProviderConfig>("src/tests/files/provider.invalid.json").await;
    }

}