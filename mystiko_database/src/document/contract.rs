#![forbid(unsafe_code)]

use mystiko_protos::core::document::v1::Contract as ProtoContract;
use mystiko_storage::column::{IndexColumns, UniqueColumns};
use mystiko_storage::document::{Document, DocumentData};
use mystiko_storage_macros::CollectionBuilder;
use mystiko_types::ContractType;
use serde::{Deserialize, Serialize};

#[derive(CollectionBuilder, Clone, PartialEq, Debug, Deserialize, Serialize)]
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

impl Contract {
    pub fn from_proto(proto: ProtoContract) -> Document<Self> {
        Document::new(
            proto.id,
            proto.created_at,
            proto.updated_at,
            Contract {
                contract_type: proto.contract_type.into(),
                chain_id: proto.chain_id,
                contract_address: proto.contract_address,
                disabled: proto.disabled,
                sync_start: proto.sync_start,
                sync_size: proto.sync_size,
                synced_block_number: proto.synced_block_number,
                checked_leaf_index: proto.checked_leaf_index,
            },
        )
    }

    pub fn into_proto(contract: Document<Self>) -> ProtoContract {
        ProtoContract::builder()
            .id(contract.id)
            .created_at(contract.created_at)
            .updated_at(contract.updated_at)
            .chain_id(contract.data.chain_id)
            .contract_address(contract.data.contract_address)
            .disabled(contract.data.disabled)
            .sync_start(contract.data.sync_start)
            .sync_size(contract.data.sync_size)
            .synced_block_number(contract.data.synced_block_number)
            .checked_leaf_index(contract.data.checked_leaf_index)
            .contract_type(Into::<i32>::into(contract.data.contract_type))
            .build()
    }
}
