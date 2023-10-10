use crate::handler::document::DatabaseContract;
use anyhow::Result;
use mystiko_config::MystikoConfig;
use mystiko_storage::{DocumentData, IndexColumns, UniqueColumns};
use mystiko_storage_macros::CollectionBuilder;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(CollectionBuilder, Clone, PartialEq, Debug, Deserialize, Serialize, TypedBuilder)]
#[collection(uniques = uniques(), indexes = indexes())]
#[builder(field_defaults(setter(into)))]
pub struct Contract {
    pub chain_id: u64,
    #[column(length_limit = 64)]
    pub contract_address: String,
    pub loaded_block: u64,
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

impl DatabaseContract for Contract {
    fn column_chain_id() -> String {
        ContractColumn::ChainId.to_string()
    }

    fn column_contract_address() -> String {
        ContractColumn::ContractAddress.to_string()
    }

    fn column_loaded_block() -> String {
        ContractColumn::LoadedBlock.to_string()
    }

    fn from_config(config: Arc<MystikoConfig>, chain_id: u64, address: &str) -> Result<Self> {
        if let Some(chain_config) = config.find_chain(chain_id) {
            if let Some(contract_config) = chain_config.find_contract_by_address(address) {
                return Ok(Contract::builder()
                    .chain_id(chain_id)
                    .contract_address(address)
                    .loaded_block(contract_config.start_block())
                    .build());
            }
        }
        Err(anyhow::anyhow!(
            "chain_id {} contract_address {} not found",
            chain_id,
            address
        ))
    }

    fn get_chain_id(&self) -> u64 {
        self.chain_id
    }

    fn get_contract_address(&self) -> &String {
        &self.contract_address
    }

    fn get_loaded_block(&self) -> u64 {
        self.loaded_block
    }

    fn update_loaded_block(&mut self, block: u64) {
        self.loaded_block = block;
    }
}
