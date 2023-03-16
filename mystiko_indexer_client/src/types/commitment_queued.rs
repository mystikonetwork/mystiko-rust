use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Deserialize, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CommitmentQueuedFilter {
    #[builder(setter(strip_option), default=None)]
    pub commit_hash: Option<String>,
    #[builder(setter(strip_option), default=None)]
    pub tx_hash: Option<String>,
    #[builder(setter(strip_option), default=None)]
    pub encrypted_note: Option<String>,
    #[builder(setter(strip_option), default=None)]
    pub leaf_index: Option<u32>,
    #[builder(setter(strip_option), default=None)]
    pub create_at: Option<u64>,
    #[builder(setter(strip_option), default=None)]
    pub status: Option<u32>,
    #[builder(setter(strip_option), default=None)]
    pub contract_id: Option<u64>,
}

#[derive(Deserialize, Serialize, TypedBuilder)]
pub struct CommitmentQueuedForChainRequest {
    pub chain_id: u32,
    #[builder(setter(strip_option), default=None)]
    pub start_block: Option<u32>,
    #[builder(setter(strip_option), default=None)]
    pub end_block: Option<u32>,
    #[builder(setter(strip_option), default=None)]
    pub where_filter: Option<CommitmentQueuedFilter>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CommitmentQueuedResponse {
    pub id: u64,
    pub chain_id: u32,
    pub contract_address: String,
    pub commit_hash: String,
    pub tx_hash: String,
    pub block_num: u64,
    pub encrypted_note: String,
    pub rollup_fee: String,
    pub leaf_index: u32,
    pub create_at: Option<u64>,
    pub status: Option<u32>,
    pub contract_id: u64,
}
