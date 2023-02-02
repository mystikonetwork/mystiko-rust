use crate::raw::bridge::axelar::RawAxelarBridgeConfig;
use crate::wrapper::bridge::base::BridgeConfig;

pub struct AxelarBridgeConfig {
    base: BridgeConfig<RawAxelarBridgeConfig>,
}

impl AxelarBridgeConfig {
    pub fn new(data: RawAxelarBridgeConfig) -> Self {
        Self {
            base: BridgeConfig::new(data, None)
        }
    }

    pub fn mutate(&self, data: Option<RawAxelarBridgeConfig>) -> Self {
        match data {
            Some(value) => AxelarBridgeConfig::new(value),
            None => AxelarBridgeConfig::new(self.base.base.data.clone()),
        }
    }
}