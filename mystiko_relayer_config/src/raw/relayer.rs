use crate::raw::chain::RawChainConfig;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(TypedBuilder, Validate, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawRelayerConfig {
    // TODO #[validate(custom = "is_sem_ver")]
    pub version: String,

    #[validate]
    pub chains: Vec<RawChainConfig>,
}
