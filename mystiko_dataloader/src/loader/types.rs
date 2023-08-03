use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

pub const DEFAULT_LOAD_INTERVAL_MS: u64 = 1200000_u64;
pub const DEFAULT_MAX_BATCH_BLOCK: u64 = 10000_u64;
pub const DEFAULT_DELAY_BLOCK: u64 = 2_u64;

#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct StartOption {
    #[builder(setter(strip_option))]
    pub load_interval_ms: Option<u64>,
    #[builder(setter(strip_option))]
    pub max_batch_block: Option<u64>,
    #[builder(setter(strip_option))]
    pub delay_block: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct LoaderState {
    pub is_running: bool,
    pub is_loading: bool,
}
