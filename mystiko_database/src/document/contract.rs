#![forbid(unsafe_code)]

use mystiko_storage::column::{IndexColumns, UniqueColumns};
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
    vec![UniqueColumns::builder()
        .column_names(vec![
            ContractColumn::ChainId.to_string(),
            ContractColumn::ContractAddress.to_string(),
        ])
        .build()]
}

fn indexes() -> Vec<IndexColumns> {
    vec![
        IndexColumns::builder()
            .column_names(vec![ContractColumn::ChainId.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![ContractColumn::ContractAddress.to_string()])
            .build(),
    ]
}
