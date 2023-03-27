use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, Deserialize, TypedBuilder, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ChainSyncRepsonse {
    pub chain_id: u32,
    pub current_sync_block_num: u32,
    pub contracts: Vec<ContractSyncResponse>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContractSyncResponse {
    #[builder(setter(strip_option), default=None)]
    pub chain_id: Option<u32>,
    pub contract_address: String,
    pub current_sync_block_num: u32,
    pub current_sync_time: u64,
}
