#![forbid(unsafe_code)]

use mystiko_storage::column::{IndexColumns, UniqueColumns};
use mystiko_storage::document::DocumentData;
use mystiko_storage_macros::CollectionBuilder;
use mystiko_types::CommitmentStatus;
use num_bigint::BigInt;

#[derive(CollectionBuilder, Clone, PartialEq, Debug)]
#[collection(uniques = uniques(), indexes = indexes())]
pub struct Commitment {
    pub chain_id: u64,
    #[column(length_limit = 64)]
    pub contract_address: String,
    #[column(length_limit = 128)]
    pub commitment_hash: BigInt,
    #[column(length_limit = 16)]
    pub asset_symbol: String,
    pub asset_decimals: u32,
    #[column(length_limit = 64)]
    pub asset_address: Option<String>,
    #[column(length_limit = 32)]
    pub status: CommitmentStatus,
    #[column(length_limit = 128)]
    pub rollup_fee_amount: Option<BigInt>,
    pub encrypted_note: Option<String>,
    #[column(length_limit = 128)]
    pub leaf_index: Option<BigInt>,
    #[column(length_limit = 128)]
    pub amount: Option<BigInt>,
    #[column(length_limit = 128)]
    pub nullifier: Option<BigInt>,
    #[column(length_limit = 128)]
    pub shielded_address: Option<String>,
    #[column(length_limit = 128)]
    pub creation_transaction_hash: Option<String>,
    #[column(length_limit = 128)]
    pub spending_transaction_hash: Option<String>,
    #[column(length_limit = 128)]
    pub rollup_transaction_hash: Option<String>,
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
        vec![CommitmentColumn::AssetSymbol].into(),
        vec![CommitmentColumn::AssetAddress].into(),
        vec![CommitmentColumn::Nullifier].into(),
        vec![CommitmentColumn::ShieldedAddress].into(),
        vec![CommitmentColumn::CreationTransactionHash].into(),
        vec![CommitmentColumn::SpendingTransactionHash].into(),
        vec![CommitmentColumn::RollupTransactionHash].into(),
    ]
}
