use crate::raw::bridge::celer::RawCelerBridgeConfig;
use crate::wrapper::bridge::base::BridgeConfig;

pub struct CelerBridgeConfig {
    base: BridgeConfig<RawCelerBridgeConfig>,
}

impl CelerBridgeConfig {
    pub fn new(data: RawCelerBridgeConfig) -> Self {
        Self {
            base: BridgeConfig::new(data, None)
        }
    }

    pub fn mutate(&self, data: Option<RawCelerBridgeConfig>) -> Self {
        match data {
            Some(value) => CelerBridgeConfig::new(value),
            None => CelerBridgeConfig::new(self.base.base.data.clone()),
        }
    }
}