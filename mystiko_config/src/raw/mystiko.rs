use serde::{Deserialize, Serialize};
use crate::raw::bridge::axelar::RawAxelarBridgeConfig;
use crate::raw::bridge::celer::RawCelerBridgeConfig;
use crate::raw::bridge::layer_zero::RawLayerZeroBridgeConfig;
use crate::raw::bridge::poly::RawPolyBridgeConfig;
use crate::raw::bridge::tbridge::RawTBridgeConfig;
use crate::raw::chain::RawChainConfig;
use crate::raw::circuit::RawCircuitConfig;
use crate::raw::indexer::RawIndexerConfig;

use validator::{Validate};
use crate::common::validate_object;
use crate::raw::base::RawConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RawBridgeConfigType {
    RawAxelarBridgeConfig(RawAxelarBridgeConfig),
    RawCelerBridgeConfig(RawCelerBridgeConfig),
    RawLayerZeroBridgeConfig(RawLayerZeroBridgeConfig),
    RawPolyBridgeConfig(RawPolyBridgeConfig),
    RawTBridgeConfig(RawTBridgeConfig),
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawMystikoConfig {
    pub version: String,
    pub chains: Vec<RawChainConfig>,
    pub bridges: Vec<RawBridgeConfigType>,
    pub circuits: Vec<RawCircuitConfig>,
    pub indexer: Option<RawIndexerConfig>,
}

impl RawConfig for RawMystikoConfig {
    fn validate(&self) -> Result<(), Vec<String>> {
        let result = validate_object(self);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        Ok(())
    }
}