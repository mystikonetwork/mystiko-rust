#![forbid(unsafe_code)]

use mystiko_storage::column::{IndexColumns, UniqueColumns};
use mystiko_storage::document::DocumentData;
use mystiko_storage_macros::CollectionBuilder;
use mystiko_types::ContractType;

#[derive(CollectionBuilder, Clone, PartialEq, Debug)]
#[collection(uniques = uniques(), indexes = indexes())]
pub struct Contract {
    #[column(length_limit = 32)]
    pub contract_type: ContractType,
    pub chain_id: u64,
    #[column(length_limit = 64)]
    pub contract_address: String,
    pub disabled: bool,
    pub sync_start: u64,
    pub sync_size: u64,
    pub synced_block_number: u64,
    pub checked_leaf_index: Option<u64>,
}

fn uniques() -> Vec<UniqueColumns> {
    vec![vec![ContractColumn::ChainId, ContractColumn::ContractAddress].into()]
}

fn indexes() -> Vec<IndexColumns> {
    vec![
        vec![ContractColumn::ChainId].into(),
        vec![ContractColumn::ContractAddress].into(),
    ]
}
