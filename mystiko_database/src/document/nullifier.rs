#![forbid(unsafe_code)]

use mystiko_storage::column::{IndexColumns, UniqueColumns};
use mystiko_storage::document::DocumentData;
use mystiko_storage_macros::CollectionBuilder;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

#[derive(CollectionBuilder, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[collection(uniques = uniques(), indexes = indexes())]
pub struct Nullifier {
    pub chain_id: u64,
    #[column(length_limit = 64)]
    pub contract_address: String,
    #[column(length_limit = 128)]
    pub nullifier: BigUint,
    #[column(length_limit = 128)]
    pub transaction_hash: String,
}

fn uniques() -> Vec<UniqueColumns> {
    vec![vec![
        NullifierColumn::ChainId,
        NullifierColumn::ContractAddress,
        NullifierColumn::Nullifier,
    ]
    .into()]
}

fn indexes() -> Vec<IndexColumns> {
    vec![
        vec![NullifierColumn::ChainId].into(),
        vec![NullifierColumn::ContractAddress].into(),
        vec![NullifierColumn::Nullifier].into(),
        vec![NullifierColumn::TransactionHash].into(),
    ]
}
