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

    pub fn data(&self) -> &RawIndexerConfig {
        &self.base.data
    }

    pub fn copy_data(&self) -> RawIndexerConfig {
        self.base.copy_data()
    }

    pub fn to_json_string(&self) -> String {
        self.base.to_json_string()
    }

    pub fn url(&self) -> &str {
        &self.base.data.url
    }

    pub fn time_out_ms(&self) -> &u32 {
        &self.base.data.timeout_ms
    }

    pub fn mutate(&self, data: Option<RawIndexerConfig>) -> Self {
        let data = match data {
            None => { self.data().clone() }
            Some(value) => { value }
        };
        IndexerConfig::new(data)
    }
}
