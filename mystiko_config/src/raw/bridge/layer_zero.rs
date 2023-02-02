use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::BridgeType;
use crate::raw::bridge::base::RawBridgeConfig;

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct RawLayerZeroBridgeConfig {
    raw: RawBridgeConfig,
    bridge_type: BridgeType,
}