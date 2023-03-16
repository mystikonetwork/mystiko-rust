use crate::common::BridgeType;
use crate::raw::bridge::tbridge::RawTBridgeConfig;
use crate::wrapper::bridge::base::BridgeConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct TBridgeConfig {
    base: BridgeConfig<RawTBridgeConfig>,
}

impl TBridgeConfig {
    pub fn new(data: RawTBridgeConfig) -> Self {
        Self {
            base: BridgeConfig::new(data, None),
        }
    }

    pub fn data(&self) -> &RawTBridgeConfig {
        &self.base.base.data
    }

    pub fn copy_data(&self) -> RawTBridgeConfig {
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

    pub fn mutate(&self, data: Option<RawTBridgeConfig>) -> Self {
        match data {
            Some(value) => TBridgeConfig::new(value),
            None => TBridgeConfig::new(self.data().clone()),
        }
    }
}
