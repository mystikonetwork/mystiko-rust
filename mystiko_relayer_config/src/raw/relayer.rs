use crate::raw::chain::RawChainConfig;
use mystiko_validator::validate::is_sem_ver;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(TypedBuilder, Validate, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawRelayerConfig {
    #[validate(custom = "is_sem_ver")]
    pub version: String,

    #[validate]
    pub chains: Vec<Arc<RawChainConfig>>,
}
