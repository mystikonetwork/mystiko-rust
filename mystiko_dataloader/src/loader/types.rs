use crate::filter::ContractFilter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use typed_builder::TypedBuilder;

pub const DEFAULT_LOAD_INTERVAL_MS: u64 = 1200000_u64;
pub const DEFAULT_MAX_BATCH_BLOCK: u64 = 100000_u64;

#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct StartOption {
    #[builder(setter(strip_option))]
    pub load_interval_ms: Option<u64>,
    #[builder(setter(strip_option))]
    pub max_batch_block: Option<u64>,
    #[builder(default, setter(strip_option))]
    pub contract_filter: Option<Arc<Box<dyn ContractFilter>>>,
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct ContractState {
    pub loaded_block: u64,
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct ChainState {
    pub loaded_block: u64,
    pub is_running: bool,
    pub is_loading: bool,
    pub contract_states: HashMap<String, ContractState>,
}
