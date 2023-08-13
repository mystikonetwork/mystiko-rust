use crate::types::commitment_spent::NullifierForDataLoader;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Deserialize, Serialize, PartialEq, Debug, Eq)]
pub enum DepositStatus {
    #[serde(rename = "srcSucceeded")]
    SrcSucceeded,
    #[serde(rename = "Queued")]
    Queued,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Deserialize, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CommitmentFilter {
    #[builder(setter(strip_option), default = None)]
    pub status: Option<DepositStatus>,
    #[builder(setter(strip_option), default = None)]
    pub chain_id: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub contract_address: Option<String>,
    #[builder(setter(strip_option), default = None)]
    pub commit_hash: Option<String>,
    #[builder(setter(strip_option), default = None)]
    pub rollup_fee: Option<String>,
    #[builder(setter(strip_option), default = None)]
    pub encrypted_note: Option<String>,
    #[builder(setter(strip_option), default = None)]
    pub leaf_index: Option<u32>,
    #[builder(setter(strip_option), default = None)]
    pub block_num: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub src_chain_id: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub src_address: Option<String>,
    #[builder(setter(strip_option), default = None)]
    pub creation_transaction_hash: Option<String>,
    #[builder(setter(strip_option), default = None)]
    pub relay_transaction_hash: Option<String>,
    #[builder(setter(strip_option), default = None)]
    pub rollup_transaction_hash: Option<String>,
    #[builder(setter(strip_option), default = None)]
    pub create_at: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub update_at: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub contract_id: Option<u32>,
    #[builder(setter(strip_option), default = None)]
    pub src_contract_id: Option<u32>,
    #[builder(setter(strip_option), default = None)]
    pub transaction_id: Option<u32>,
    #[builder(setter(strip_option), default = None)]
    pub rollup_id: Option<u32>,
    #[builder(setter(strip_option), default = None)]
    pub deposit_id: Option<u32>,
}

#[derive(Deserialize, Serialize, TypedBuilder)]
pub struct CommitmentsForContractRequest {
    pub chain_id: u64,
    pub contract_address: String,
    #[builder(setter(strip_option), default = None)]
    pub start_block: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub end_block: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub where_filter: Option<CommitmentFilter>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CommitmentResponse {
    pub id: u32,
    pub status: DepositStatus,
    #[builder(setter(strip_option), default = None)]
    pub chain_id: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub contract_address: Option<String>,
    pub commit_hash: String,
    pub rollup_fee: String,
    pub encrypted_note: String,
    #[builder(setter(strip_option), default = None)]
    pub leaf_index: Option<u32>,
    #[builder(setter(strip_option), default = None)]
    pub block_num: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub src_chain_id: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub src_address: Option<String>,
    pub creation_transaction_hash: String,
    #[builder(setter(strip_option), default = None)]
    pub relay_transaction_hash: Option<String>,
    #[builder(setter(strip_option), default = None)]
    pub rollup_transaction_hash: Option<String>,
    #[builder(setter(strip_option), default = None)]
    pub create_at: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub update_at: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub contract_id: Option<u32>,
    #[builder(setter(strip_option), default = None)]
    pub src_contract_id: Option<u32>,
    #[builder(setter(strip_option), default = None)]
    pub transaction_id: Option<u32>,
    #[builder(setter(strip_option), default = None)]
    pub rollup_id: Option<u32>,
    #[builder(setter(strip_option), default = None)]
    pub deposit_id: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ContractResultDataResponse {
    pub contract_address: String,
    pub start_block: u64,
    pub actual_end_block: u64,
    pub commitments: Vec<CommitmentForDataLoader>,
    pub nullifiers: Option<Vec<NullifierForDataLoader>>,
    #[serde(default, rename = "error")]
    pub is_error: bool,
    pub error_msg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CommitmentForDataLoader {
    pub commitment_hash: String,
    pub status: DepositStatus,
    pub block_number: u64,
    #[builder(setter(strip_option), default = None)]
    pub included_block_number: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub src_chain_block_number: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub leaf_index: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    pub rollup_fee: Option<String>,
    #[builder(setter(strip_option), default = None)]
    pub encrypted_note: Option<String>,
    #[builder(setter(strip_option), default = None)]
    pub queued_transaction_hash: Option<String>,
    #[builder(setter(strip_option), default = None)]
    pub included_transaction_hash: Option<String>,
    #[builder(setter(strip_option), default = None)]
    pub src_chain_transaction_hash: Option<String>,
}
