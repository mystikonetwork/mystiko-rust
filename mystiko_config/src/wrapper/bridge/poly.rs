use crate::common::BridgeType;
use crate::raw::bridge::poly::RawPolyBridgeConfig;
use crate::wrapper::bridge::base::BridgeConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct PolyBridgeConfig {
    base: BridgeConfig<RawPolyBridgeConfig>,
}

impl PolyBridgeConfig {
    pub fn new(data: RawPolyBridgeConfig) -> Self {
        Self {
            base: BridgeConfig::new(data, None)
        }
    }

    pub fn data(&self) -> &RawPolyBridgeConfig {
        &self.base.base.data
    }

    pub fn copy_data(&self) -> RawPolyBridgeConfig {
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

    pub fn explorer_url(&self) -> &String {
        &self.base.base.data.explorer_url
    }

    pub fn explorer_prefix(&self) -> &String {
        &self.base.base.data.explorer_prefix
    }

    pub fn api_url(&self) -> &String {
        &self.base.base.data.api_url
    }

    pub fn api_prefix(&self) -> &String {
        &self.base.base.data.api_prefix
    }

    pub fn mutate(&self, data: Option<RawPolyBridgeConfig>) -> Self {
        match data {
            None => {
                PolyBridgeConfig::new(self.data().clone())
            }
            Some(value) => {
                PolyBridgeConfig::new(value)
            }
        }
    }
}
