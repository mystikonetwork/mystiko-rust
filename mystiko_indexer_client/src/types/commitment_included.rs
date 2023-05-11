use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Deserialize, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CommitmentIncludedFilter {
    #[builder(setter(strip_option), default=None)]
    pub chain_id: Option<u64>,
    #[builder(setter(strip_option), default=None)]
    pub contract_address: Option<String>,
    #[builder(setter(strip_option), default=None)]
    pub commit_hash: Option<String>,
    #[builder(setter(strip_option), default=None)]
    pub tx_hash: Option<String>,
    #[builder(setter(strip_option), default=None)]
    pub block_num: Option<u64>,
    #[builder(setter(strip_option), default=None)]
    pub create_at: Option<u64>,
    #[builder(setter(strip_option), default=None)]
    pub status: Option<u32>,
    #[builder(setter(strip_option), default=None)]
    pub contract_id: Option<u64>,
}

#[derive(Deserialize, Serialize, TypedBuilder)]
pub struct CommitmentIncludedForChainRequest {
    pub chain_id: u64,
    #[builder(setter(strip_option), default=None)]
    pub start_block: Option<u64>,
    #[builder(setter(strip_option), default=None)]
    pub end_block: Option<u64>,
    #[builder(setter(strip_option), default=None)]
    pub where_filter: Option<CommitmentIncludedFilter>,
}

#[derive(Deserialize, Serialize, TypedBuilder)]
pub struct CommitmentIncludedForContractRequest {
    pub chain_id: u64,
    pub address: String,
    #[builder(setter(strip_option), default=None)]
    pub start_block: Option<u64>,
    #[builder(setter(strip_option), default=None)]
    pub end_block: Option<u64>,
    #[builder(setter(strip_option), default=None)]
    pub where_filter: Option<CommitmentIncludedFilter>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CommitmentIncludedResponse {
    pub id: u64,
    pub chain_id: u64,
    pub contract_address: String,
    pub commit_hash: String,
    pub tx_hash: String,
    pub block_num: u64,
    pub create_at: u64,
    pub status: u32,
    pub contract_id: u64,
}
