use crate::raw::bridge::poly::RawPolyBridgeConfig;
use crate::wrapper::bridge::base::BridgeConfig;

#[derive(Clone)]
pub struct PolyBridgeConfig {
    base: BridgeConfig<RawPolyBridgeConfig>,
}

impl PolyBridgeConfig {
    pub fn new(data: RawPolyBridgeConfig) -> Self {
        Self {
            base: BridgeConfig::new(data, None)
        }
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
}