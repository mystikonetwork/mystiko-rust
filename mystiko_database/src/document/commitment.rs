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
        UniqueColumns::builder()
            .column_names(vec![
                CommitmentColumn::ChainId.to_string(),
                CommitmentColumn::ContractAddress.to_string(),
                CommitmentColumn::CommitmentHash.to_string(),
            ])
            .build(),
        UniqueColumns::builder()
            .column_names(vec![
                CommitmentColumn::ChainId.to_string(),
                CommitmentColumn::ContractAddress.to_string(),
                CommitmentColumn::LeafIndex.to_string(),
            ])
            .build(),
    ]
}

fn indexes() -> Vec<IndexColumns> {
    vec![
        IndexColumns::builder()
            .column_names(vec![CommitmentColumn::ChainId.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![CommitmentColumn::ContractAddress.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![CommitmentColumn::CommitmentHash.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![CommitmentColumn::LeafIndex.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![CommitmentColumn::Status.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![CommitmentColumn::AssetSymbol.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![CommitmentColumn::AssetAddress.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![CommitmentColumn::Nullifier.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![CommitmentColumn::ShieldedAddress.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![CommitmentColumn::CreationTransactionHash.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![CommitmentColumn::SpendingTransactionHash.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![CommitmentColumn::RollupTransactionHash.to_string()])
            .build(),
    ]
}
