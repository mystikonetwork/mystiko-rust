use crate::common::BridgeType;
use crate::raw::bridge::layer_zero::RawLayerZeroBridgeConfig;
use crate::wrapper::bridge::base::BridgeConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct LayerZeroBridgeConfig {
    base: BridgeConfig<RawLayerZeroBridgeConfig>,
}

impl LayerZeroBridgeConfig {
    pub fn new(data: RawLayerZeroBridgeConfig) -> Self {
        Self {
            base: BridgeConfig::new(data, None)
        }
    }

    pub fn data(&self) -> &RawLayerZeroBridgeConfig {
        &self.base.base.data
    }

    pub fn copy_data(&self) -> RawLayerZeroBridgeConfig {
        self.base.base.copy_data()
    }

    pub fn bridge_type(&self) -> &BridgeType {
        &self.base.bridge_type()
    }

    pub fn name(&self) -> &String {
        &self.base.name()
    }

    pub fn to_json_string(&self) -> String {
        self.base.base.to_json_string()
    }

    pub fn mutate(&self, data: Option<RawLayerZeroBridgeConfig>) -> Self {
        match data {
            Some(value) => LayerZeroBridgeConfig::new(value),
            None => LayerZeroBridgeConfig::new(self.data().clone())
        }
    }
}
