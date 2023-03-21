use crate::raw::base::{RawConfig, Validator};
use crate::raw::validator::{array_unique, is_number_string};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use typed_builder::TypedBuilder;
use validator::Validate;

pub const EXPLORER_TX_PLACEHOLDER: &str = "%tx%";
pub const EXPLORER_DEFAULT_PREFIX: &str = "/tx/%tx%";

fn default_event_filter_size() -> u64 {
    200000
}

fn default_indexer_filter_size() -> u64 {
    500000
}

fn default_explorer_prefix() -> String {
    EXPLORER_DEFAULT_PREFIX.to_string()
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, Eq, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RawChainConfig {
    #[serde(default)]
    #[builder(default)]
    pub base: RawConfig,

    #[validate(range(min = 1))]
    pub chain_id: u32,

    #[validate(length(min = 1))]
    pub name: String,

    #[validate(length(min = 1))]
    pub asset_symbol: String,

    #[validate(range(min = 1))]
    pub asset_decimals: u32,

    #[validate(
        custom(function = "array_unique"),
        custom(function = "is_number_string::<true, true>")
    )]
    #[serde(default)]
    #[builder(default = vec ! [])]
    pub recommended_amounts: Vec<String>,

    #[validate(url)]
    pub explorer_url: String,

    #[validate(contains = "%tx%")]
    #[serde(default = "default_explorer_prefix")]
    pub explorer_prefix: String,

    #[validate(url)]
    pub signer_endpoint: String,

    #[validate(range(min = 1))]
    #[serde(default = "default_event_filter_size")]
    #[builder(default = default_event_filter_size())]
    pub event_filter_size: u64,

    #[validate(range(min = 1))]
    #[serde(default = "default_indexer_filter_size")]
    #[builder(default = default_indexer_filter_size())]
    pub indexer_filter_size: u64,
}

impl PartialEq for RawChainConfig {
    fn eq(&self, other: &Self) -> bool {
        self.chain_id == other.chain_id
    }
}

impl Hash for RawChainConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.chain_id.hash(state)
    }
}

impl Validator for RawChainConfig {
    fn validation(&self) -> Result<(), anyhow::Error> {
        self.base.validate_object::<RawChainConfig>(self)
    }
}
