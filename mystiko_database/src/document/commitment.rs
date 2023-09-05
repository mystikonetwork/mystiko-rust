#![forbid(unsafe_code)]

use mystiko_storage::{DocumentData, IndexColumns, UniqueColumns};
use mystiko_storage_macros::CollectionBuilder;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

#[derive(CollectionBuilder, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[collection(uniques = uniques(), indexes = indexes())]
pub struct Commitment {
    pub chain_id: u64,
    #[column(length_limit = 64)]
    pub contract_address: String,
    #[column(length_limit = 128)]
    pub commitment_hash: BigUint,
    #[column(length_limit = 16)]
    pub asset_symbol: String,
    pub asset_decimals: u32,
    #[column(length_limit = 64)]
    pub asset_address: Option<String>,
    pub status: i32,
    pub block_number: u64,
    pub src_chain_block_number: Option<u64>,
    pub included_block_number: Option<u64>,
    #[column(length_limit = 128)]
    pub rollup_fee_amount: Option<BigUint>,
    pub encrypted_note: Option<String>,
    #[column(length_limit = 128)]
    pub leaf_index: Option<BigUint>,
    #[column(length_limit = 128)]
    pub amount: Option<BigUint>,
    #[column(length_limit = 128)]
    pub nullifier: Option<BigUint>,
    #[column(length_limit = 128)]
    pub shielded_address: Option<String>,
    #[column(length_limit = 128)]
    pub queued_transaction_hash: Option<String>,
    #[column(length_limit = 128)]
    pub included_transaction_hash: Option<String>,
    #[column(length_limit = 128)]
    pub src_chain_transaction_hash: Option<String>,
}

fn uniques() -> Vec<UniqueColumns> {
    vec![
        vec![
            CommitmentColumn::ChainId,
            CommitmentColumn::ContractAddress,
            CommitmentColumn::CommitmentHash,
        ]
        .into(),
        vec![
            CommitmentColumn::ChainId,
            CommitmentColumn::ContractAddress,
            CommitmentColumn::LeafIndex,
        ]
        .into(),
    ]
}

fn indexes() -> Vec<IndexColumns> {
    vec![
        vec![CommitmentColumn::ChainId].into(),
        vec![CommitmentColumn::ContractAddress].into(),
        vec![CommitmentColumn::CommitmentHash].into(),
        vec![CommitmentColumn::LeafIndex].into(),
        vec![CommitmentColumn::Status].into(),
        vec![CommitmentColumn::BlockNumber].into(),
        vec![CommitmentColumn::SrcChainBlockNumber].into(),
        vec![CommitmentColumn::IncludedBlockNumber].into(),
        vec![CommitmentColumn::AssetSymbol].into(),
        vec![CommitmentColumn::AssetAddress].into(),
        vec![CommitmentColumn::Nullifier].into(),
        vec![CommitmentColumn::ShieldedAddress].into(),
        vec![CommitmentColumn::QueuedTransactionHash].into(),
        vec![CommitmentColumn::IncludedTransactionHash].into(),
        vec![CommitmentColumn::SrcChainTransactionHash].into(),
    ]
}
