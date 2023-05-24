#![forbid(unsafe_code)]

use mystiko_storage::column::{IndexColumns, UniqueColumns};
use mystiko_storage::document::DocumentData;
use mystiko_storage_macros::CollectionBuilder;
use num_bigint::BigInt;

#[derive(CollectionBuilder, Clone, PartialEq, Debug)]
#[collection(uniques = uniques(), indexes = indexes())]
pub struct Nullifier {
    pub chain_id: u64,
    #[column(length_limit = 64)]
    pub contract_address: String,
    #[column(length_limit = 128)]
    pub nullifier: BigInt,
    #[column(length_limit = 128)]
    pub transaction_hash: String,
}

fn uniques() -> Vec<UniqueColumns> {
    vec![UniqueColumns::builder()
        .column_names(vec![
            NullifierColumn::ChainId.to_string(),
            NullifierColumn::ContractAddress.to_string(),
            NullifierColumn::Nullifier.to_string(),
        ])
        .build()]
}

fn indexes() -> Vec<IndexColumns> {
    vec![
        IndexColumns::builder()
            .column_names(vec![NullifierColumn::ChainId.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![NullifierColumn::ContractAddress.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![NullifierColumn::Nullifier.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![NullifierColumn::TransactionHash.to_string()])
            .build(),
    ]
}
