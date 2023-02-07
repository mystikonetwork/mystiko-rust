use crate::raw::provider::RawProviderConfig;
use crate::wrapper::base::BaseConfig;

#[derive(Clone, Debug)]
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
}