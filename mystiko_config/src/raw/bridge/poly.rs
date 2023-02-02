use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::BridgeType;
use crate::raw::bridge::base::RawBridgeConfig;

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct RawPolyBridgeConfig {
    raw: RawBridgeConfig,
    bridge_type: BridgeType,
    explorer_url: String,
    explorer_prefix: String,
    api_url: String,
    api_prefix: String,
}