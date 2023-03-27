use crate::raw::indexer::RawIndexerConfig;
use std::sync::Arc;

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
}
