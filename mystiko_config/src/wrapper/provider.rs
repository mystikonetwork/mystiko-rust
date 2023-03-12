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

    pub fn data(&self) -> &RawProviderConfig {
        &self.base.data
    }

    pub fn copy_data(&self) -> RawProviderConfig {
        self.base.copy_data()
    }

    pub fn to_json_string(&self) -> String {
        self.base.to_json_string()
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
            None => { self.data().clone() }
            Some(value) => { value }
        };
        ProviderConfig::new(data)
    }
}
