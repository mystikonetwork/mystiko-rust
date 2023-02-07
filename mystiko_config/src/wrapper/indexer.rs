use crate::raw::indexer::RawIndexerConfig;
use crate::wrapper::base::BaseConfig;

#[derive(Clone, Debug)]
pub struct IndexerConfig {
    base: BaseConfig<RawIndexerConfig>,
}

impl IndexerConfig {
    pub fn new(data: RawIndexerConfig) -> Self {
        Self { base: BaseConfig::new(data, None) }
    }
}