use crate::common::BridgeType;
use crate::raw::bridge::axelar::RawAxelarBridgeConfig;
use crate::wrapper::bridge::base::BridgeConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct AxelarBridgeConfig {
    base: BridgeConfig<RawAxelarBridgeConfig>,
}

impl AxelarBridgeConfig {
    pub fn new(data: RawAxelarBridgeConfig) -> Self {
        Self {
            base: BridgeConfig::new(data, None),
        }
    }

    pub fn data(&self) -> &RawAxelarBridgeConfig {
        &self.base.base.data
    }

    pub fn copy_data(&self) -> RawAxelarBridgeConfig {
        self.base.base.copy_data()
    }

    pub fn bridge_type(&self) -> &BridgeType {
        self.base.bridge_type()
    }

    pub fn name(&self) -> &String {
        self.base.name()
    }

    pub fn to_json_string(&self) -> String {
        self.base.base.to_json_string()
    }

    pub fn mutate(&self, data: Option<RawAxelarBridgeConfig>) -> Self {
        match data {
            Some(value) => AxelarBridgeConfig::new(value),
            None => AxelarBridgeConfig::new(self.data().clone()),
        }
    }
}
