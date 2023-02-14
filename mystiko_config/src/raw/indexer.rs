use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::validate_object;
use crate::raw::base::{RawConfig, RawConfigTrait};

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawIndexerConfig {
    #[serde(default)]
    base: RawConfig,

    #[validate(url)]
    url: String,

    #[validate(range(min = 1))]
    timeout_ms: u32,
}

impl RawIndexerConfig {
    pub fn new(
        url: String,
        timeout_ms: u32,
    ) -> Self {
        Self {
            base: RawConfig::default(),
            url,
            timeout_ms,
        }
    }
}

impl RawConfigTrait for RawIndexerConfig {
    fn validate(&self) {
        self.base.validate_object(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::base::{RawConfig, RawConfigTrait};
    use crate::raw::indexer::RawIndexerConfig;

    async fn default_config() -> RawIndexerConfig {
        RawConfig::create_from_object::<RawIndexerConfig>(
            RawIndexerConfig::new("https://example.com".to_string(), 1000)
        ).await
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_url() {
        let mut config = default_config().await;
        config.url = String::from("not a valid url");
        config.validate();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_time_out_ms() {
        let mut config = default_config().await;
        config.timeout_ms = 0;
        config.validate();
    }

    #[tokio::test]
    async fn test_import_valid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawIndexerConfig>("src/tests/files/indexer.valid.json").await;
        assert_eq!(file_config, default_config().await);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_import_invalid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawIndexerConfig>("src/tests/files/indexer.invalid.json").await;
    }
}