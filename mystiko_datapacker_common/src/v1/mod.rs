mod path;

pub use path::*;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GranularityIndex {
    pub saved_block: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MerkleTreeIndex {
    pub last_leaf_index: u64,
    #[builder(default)]
    #[serde(default)]
    pub loaded_block_number: Option<u64>,
}
