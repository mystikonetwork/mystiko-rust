use crate::raw::indexer::RawIndexerConfig;
use crate::wrapper::base::BaseConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct IndexerConfig {
    base: BaseConfig<RawIndexerConfig>,
}

impl IndexerConfig {
    pub fn new(data: RawIndexerConfig) -> Self {
        Self { base: BaseConfig::new(data, None) }
    }

    pub fn url(&self) -> &str {
        &self.base.data.url
    }

    pub fn time_out_ms(&self) -> &u32 {
        &self.base.data.timeout_ms
    }

    pub fn mutate(&self, data: Option<RawIndexerConfig>) -> Self {
        let data = match data {
            None => { self.base.data.clone() }
            Some(value) => { value }
        };
        IndexerConfig::new(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::base::RawConfig;
    use crate::raw::indexer::RawIndexerConfig;
    use crate::wrapper::indexer::IndexerConfig;

    async fn default_config() -> (RawIndexerConfig, IndexerConfig) {
        let raw_config =
            RawConfig::create_from_file::<RawIndexerConfig>("src/tests/files/indexer.valid.json").await;
        let config = IndexerConfig::new(raw_config.clone());
        (raw_config, config)
    }

    #[tokio::test]
    async fn test_equality() {
        let (raw_config, config) = default_config().await;
        assert_eq!(config.url(), raw_config.url);
        assert_eq!(config.time_out_ms(), &raw_config.timeout_ms);
    }

    #[tokio::test]
    async fn test_copy() {
        let (_, config) = default_config().await;
        assert_eq!(
            IndexerConfig::new(config.base.copy_data()),
            config
        );
    }

    #[tokio::test]
    async fn test_mutate() {
        let (mut raw_config, config) = default_config().await;
        assert_eq!(config.mutate(None), config);
        raw_config.url = "https://example1.com".to_string();
        let new_config = config.mutate(Some(raw_config));
        assert_eq!(new_config.url(), "https://example1.com");
    }

    #[tokio::test]
    async fn test_to_json_string() {
        let (raw_config, config) = default_config().await;
        let json_string = config.base.to_json_string();
        let loaded_raw_config =
            RawConfig::create_from_json_string::<RawIndexerConfig>(json_string.as_str()).await;
        assert_eq!(loaded_raw_config, raw_config);
    }
}
