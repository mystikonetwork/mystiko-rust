use crate::raw::provider::RawProviderConfig;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ProviderConfig {
    raw: Arc<RawProviderConfig>,
}

impl ProviderConfig {
    pub fn new(raw: Arc<RawProviderConfig>) -> Self {
        ProviderConfig { raw }
    }

    pub fn url(&self) -> &str {
        &self.raw.url
    }

    pub fn timeout_ms(&self) -> u32 {
        self.raw.timeout_ms
    }

    pub fn max_try_count(&self) -> u32 {
        self.raw.max_try_count
    }
}
