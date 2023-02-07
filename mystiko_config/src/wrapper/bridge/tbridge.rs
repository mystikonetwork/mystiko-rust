use crate::raw::bridge::tbridge::RawTBridgeConfig;
use crate::wrapper::bridge::base::BridgeConfig;

#[derive(Clone, Debug)]
pub struct TBridgeConfig {
    base: BridgeConfig<RawTBridgeConfig>,
}

impl TBridgeConfig {
    pub fn new(data: RawTBridgeConfig) -> Self {
        Self {
            base: BridgeConfig::new(data, None)
        }
    }

    pub fn mutate(&self, data: Option<RawTBridgeConfig>) -> Self {
        match data {
            Some(value) => TBridgeConfig::new(value),
            None => TBridgeConfig::new(self.base.base.data.clone())
        }
    }
}