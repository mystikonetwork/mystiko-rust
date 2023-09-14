use crate::RawIndexerConfig;
use anyhow::Result;
use std::sync::Arc;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct IndexerConfig {
    raw: Arc<RawIndexerConfig>,
}

impl IndexerConfig {
    pub fn new(raw: Arc<RawIndexerConfig>) -> Self {
        IndexerConfig { raw }
    }

    pub fn url(&self) -> &str {
        &self.raw.url
    }

    pub fn timeout_ms(&self) -> u64 {
        self.raw.timeout_ms
    }

    #[cfg(feature = "proto")]
    pub fn to_proto(&self) -> mystiko_protos::config::v1::IndexerConfig {
        mystiko_protos::config::v1::IndexerConfig::builder()
            .url(self.url().to_string())
            .timeout_ms(self.timeout_ms())
            .build()
    }

    pub fn validate(&self) -> Result<()> {
        Ok(self.raw.validate()?)
    }
}
