use crate::raw::bridge::RawBridgeConfig;
use crate::raw::chain::RawChainConfig;
use crate::raw::circuit::RawCircuitConfig;
use crate::raw::indexer::RawIndexerConfig;
use crate::raw::validator::{array_unique, is_sem_ver, validate_nested_vec};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawMystikoConfig {
    #[validate(custom = "is_sem_ver")]
    pub version: String,

    #[validate(
        custom(function = "array_unique"),
        custom(function = "validate_nested_vec")
    )]
    pub chains: Vec<Arc<RawChainConfig>>,

    #[validate(
        custom(function = "array_unique"),
        custom(function = "validate_nested_vec")
    )]
    pub bridges: Vec<Arc<RawBridgeConfig>>,

    #[validate(
        custom(function = "array_unique"),
        custom(function = "validate_nested_vec")
    )]
    pub circuits: Vec<Arc<RawCircuitConfig>>,

    #[validate]
    pub indexer: Option<Arc<RawIndexerConfig>>,
}
