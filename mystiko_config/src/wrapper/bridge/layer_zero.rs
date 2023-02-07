use crate::raw::bridge::layer_zero::RawLayerZeroBridgeConfig;
use crate::wrapper::bridge::base::BridgeConfig;

#[derive(Clone)]
pub struct LayerZeroBridgeConfig {
    base: BridgeConfig<RawLayerZeroBridgeConfig>,
}

impl LayerZeroBridgeConfig {
    pub fn new(data: RawLayerZeroBridgeConfig) -> Self {
        Self {
            base: BridgeConfig::new(data, None)
        }
    }

    pub fn mutate(&self, data: Option<RawLayerZeroBridgeConfig>) -> Self {
        match data {
            Some(value) => LayerZeroBridgeConfig::new(value),
            None => LayerZeroBridgeConfig::new(self.base.base.data.clone())
        }
    }
}