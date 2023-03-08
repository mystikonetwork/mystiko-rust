use crate::raw::indexer::RawIndexerConfig;
use crate::wrapper::base::BaseConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct IndexerConfig {
    pub base: BaseConfig<RawIndexerConfig>,
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
