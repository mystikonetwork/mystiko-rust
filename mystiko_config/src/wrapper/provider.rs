use crate::raw::provider::RawProviderConfig;
use crate::wrapper::base::BaseConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct ProviderConfig {
    base: BaseConfig<RawProviderConfig>,
}

impl ProviderConfig {
    pub fn new(raw: RawProviderConfig) -> Self {
        Self {
            base: BaseConfig::new(raw, None)
        }
    }

    pub fn url(&self) -> &String {
        &self.base.data.url
    }

    pub fn timeout_ms(&self) -> &u32 {
        &self.base.data.timeout_ms
    }

    pub fn max_try_count(&self) -> &u32 {
        &self.base.data.max_try_count
    }

    pub fn mutate(&self, data: Option<RawProviderConfig>) -> Self {
        let data = match data {
            None => { self.base.data.clone() }
            Some(value) => { value }
        };
        ProviderConfig::new(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::base::RawConfig;
    use crate::raw::provider::RawProviderConfig;
    use crate::wrapper::provider::ProviderConfig;

    async fn default_config() -> (RawProviderConfig, ProviderConfig) {
        let raw_config =
            RawConfig::create_from_file::<RawProviderConfig>("src/tests/files/provider.valid.json").await;
        let config = ProviderConfig::new(raw_config.clone());
        (raw_config, config)
    }

    #[tokio::test]
    async fn test_equality() {
        let (raw_config, config) = default_config().await;
        assert_eq!(config.url(), &raw_config.url);
        assert_eq!(config.timeout_ms(), &raw_config.timeout_ms);
        assert_eq!(config.max_try_count(), &raw_config.max_try_count);
    }

    #[tokio::test]
    async fn test_copy() {
        let (_, config) = default_config().await;
        assert_eq!(
            ProviderConfig::new(config.base.copy_data()),
            config
        );
    }

    #[tokio::test]
    async fn test_mutate() {
        let (mut raw_config, config) = default_config().await;
        assert_eq!(config.mutate(None), config);
        raw_config.max_try_count = 10;
        let new_config = config.mutate(Some(raw_config));
        assert_eq!(*new_config.max_try_count(), 10);
    }

    #[tokio::test]
    async fn test_to_json_string() {
        let (raw_config, config) = default_config().await;
        let json_string = config.base.to_json_string();
        let loaded_raw_config =
            RawConfig::create_from_json_string::<RawProviderConfig>(json_string.as_str()).await;
        assert_eq!(loaded_raw_config, raw_config);
    }
}