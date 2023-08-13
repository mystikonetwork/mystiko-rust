use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Deserialize, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CommitmentSpentFilter {
    #[builder(setter(strip_option), default = None)]
    pub chain_id: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub contract_address: Option<String>,
    #[builder(setter(strip_option), default = None)]
    pub root_hash: Option<String>,
    #[builder(setter(strip_option), default = None)]
    pub tx_hash: Option<String>,
    #[builder(setter(strip_option), default = None)]
    pub serial_num: Option<String>,
    #[builder(setter(strip_option), default = None)]
    pub block_num: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub create_at: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub update_at: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub status: Option<u32>,
    #[builder(setter(strip_option), default = None)]
    pub contract_id: Option<u64>,
}

#[derive(Deserialize, Serialize, TypedBuilder)]
pub struct CommitmentSpentForChainRequest {
    pub chain_id: u64,
    #[builder(setter(strip_option), default = None)]
    pub start_block: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub end_block: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub where_filter: Option<CommitmentSpentFilter>,
}

#[derive(Deserialize, Serialize, TypedBuilder)]
pub struct CommitmentSpentForContractRequest {
    pub chain_id: u64,
    pub address: String,
    #[builder(setter(strip_option), default = None)]
    pub start_block: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub end_block: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub where_filter: Option<CommitmentSpentFilter>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CommitmentSpentResponse {
    pub id: u64,
    pub chain_id: u64,
    pub contract_address: String,
    pub root_hash: String,
    pub tx_hash: String,
    pub serial_num: String,
    pub block_num: u64,
    pub create_at: u64,
    pub update_at: u64,
    pub status: u32,
    pub contract_id: u64,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DataLoaderRequest {
    pub contract_address: String,
    pub start_block: Option<u64>,
    pub end_block: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct NullifierForDataLoader {
    pub nullifier: String,
    pub block_number: u64,
    pub transaction_hash: String,
}
