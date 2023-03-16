use crate::common::BridgeType;
use crate::raw::bridge::celer::RawCelerBridgeConfig;
use crate::wrapper::bridge::base::BridgeConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct CelerBridgeConfig {
    base: BridgeConfig<RawCelerBridgeConfig>,
}

impl CelerBridgeConfig {
    pub fn new(data: RawCelerBridgeConfig) -> Self {
        Self {
            base: BridgeConfig::new(data, None),
        }
    }

    pub fn data(&self) -> &RawCelerBridgeConfig {
        &self.base.base.data
    }

    pub fn copy_data(&self) -> RawCelerBridgeConfig {
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

    pub fn mutate(&self, data: Option<RawCelerBridgeConfig>) -> Self {
        match data {
            Some(value) => CelerBridgeConfig::new(value),
            None => CelerBridgeConfig::new(self.data().clone()),
        }
    }
}
