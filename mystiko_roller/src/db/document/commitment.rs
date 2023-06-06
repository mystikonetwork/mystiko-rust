#![forbid(unsafe_code)]
use mystiko_storage::column::{IndexColumns, UniqueColumns};
use mystiko_storage::document::DocumentData;
use mystiko_storage_macros::CollectionBuilder;
use num_bigint::BigInt;

#[derive(CollectionBuilder, Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
#[collection(uniques = uniques(), indexes = indexes())]
pub struct CommitmentInfo {
    pub chain_id: u64,
    #[column(length_limit = 64)]
    pub contract_address: String,
    #[column(length_limit = 128)]
    pub commitment_hash: BigInt,
    pub block_number: u64,
    #[column(length_limit = 128)]
    pub rollup_fee: String,
    pub leaf_index: u32,
    #[column(length_limit = 128)]
    pub tx_hash: String,
}

fn uniques() -> Vec<UniqueColumns> {
    vec![
        vec![CommitmentInfoColumn::CommitmentHash].into(),
        vec![CommitmentInfoColumn::LeafIndex].into(),
    ]
}

fn indexes() -> Vec<IndexColumns> {
    vec![
        vec![CommitmentInfoColumn::CommitmentHash].into(),
        vec![CommitmentInfoColumn::LeafIndex].into(),
    ]
}
