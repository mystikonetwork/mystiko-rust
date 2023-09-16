#![forbid(unsafe_code)]

use mystiko_storage::{DocumentData, IndexColumns, UniqueColumns};
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
    pub loaded_block_number: u64,
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

#[cfg(feature = "loader")]
impl mystiko_dataloader::handler::document::DatabaseContract for Contract {
    fn column_chain_id() -> String {
        ContractColumn::ChainId.to_string()
    }

    fn column_contract_address() -> String {
        ContractColumn::ContractAddress.to_string()
    }

    fn column_loaded_block() -> String {
        ContractColumn::LoadedBlockNumber.to_string()
    }

    fn from_config(
        config: std::sync::Arc<mystiko_config::MystikoConfig>,
        chain_id: u64,
        address: &str,
    ) -> anyhow::Result<Self> {
        if let Some(contract_config) = config.find_contract_by_address(chain_id, address) {
            Ok(Self {
                contract_type: contract_config.contract_type().clone(),
                chain_id,
                contract_address: address.to_string(),
                disabled: contract_config.disabled(),
                loaded_block_number: contract_config.start_block(),
            })
        } else {
            Err(anyhow::anyhow!(
                "chain_id {} contract {} not found in config",
                chain_id,
                address
            ))
        }
    }

    fn get_chain_id(&self) -> u64 {
        self.chain_id
    }

    fn get_contract_address(&self) -> &String {
        &self.contract_address
    }

    fn get_loaded_block(&self) -> u64 {
        self.loaded_block_number
    }

    fn update_loaded_block(&mut self, block: u64) {
        self.loaded_block_number = block;
    }
}
