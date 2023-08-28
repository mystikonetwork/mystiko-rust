use crate::error::MystikoError;
use crate::types::Result;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_database::database::Database;
use mystiko_database::document::contract::{Contract, ContractColumn};
use mystiko_protos::core::document::v1::Contract as ProtoContract;
use mystiko_protos::storage::v1::{Condition, QueryFilter, SubFilter};
use mystiko_storage::document::Document;
use mystiko_storage::formatter::types::StatementFormatter;
use mystiko_storage::storage::Storage;
use std::sync::Arc;

#[derive(Debug)]
pub struct ContractHandler<F: StatementFormatter, S: Storage> {
    db: Arc<Database<F, S>>,
    config: Arc<MystikoConfig>,
}

impl<F, S> ContractHandler<F, S>
where
    F: StatementFormatter,
    S: Storage,
{
    pub fn new(db: Arc<Database<F, S>>, config: Arc<MystikoConfig>) -> Self {
        Self { db, config }
    }

    pub async fn find<Q: Into<QueryFilter>>(&self, query_filter: Q) -> Result<Vec<ProtoContract>> {
        let documents = self
            .db
            .contracts
            .find(query_filter)
            .await
            .map_err(MystikoError::StorageError)?;
        Ok(documents.into_iter().map(Contract::into_proto).collect())
    }

    pub async fn find_all(&self) -> Result<Vec<ProtoContract>> {
        let documents = self.db.contracts.find_all().await.map_err(MystikoError::StorageError)?;
        Ok(documents.into_iter().map(Contract::into_proto).collect())
    }

    pub async fn find_by_chain_id(&self, chain_id: u64) -> Result<Vec<ProtoContract>> {
        let filters: Vec<Condition> = vec![
            SubFilter::equal(ContractColumn::ChainId, chain_id).into(),
            SubFilter::equal(ContractColumn::Disabled, false).into(),
        ];
        self.find(filters).await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<ProtoContract>> {
        self.db
            .contracts
            .find_by_id(id)
            .await
            .map_err(MystikoError::StorageError)
            .map(|doc| doc.map(Contract::into_proto))
    }

    pub async fn find_by_address(&self, chain_id: u64, address: &str) -> Result<Option<ProtoContract>> {
        let filters: Vec<Condition> = vec![
            SubFilter::equal(ContractColumn::ChainId, chain_id).into(),
            SubFilter::equal(ContractColumn::ContractAddress, address).into(),
        ];
        Ok(self
            .db
            .contracts
            .find_one(filters)
            .await
            .map_err(MystikoError::StorageError)?
            .map(Contract::into_proto))
    }

    pub async fn count<Q: Into<QueryFilter>>(&self, query_filter: Q) -> Result<u64> {
        self.db
            .contracts
            .count(query_filter)
            .await
            .map_err(MystikoError::StorageError)
    }

    pub async fn count_all(&self) -> Result<u64> {
        self.db.contracts.count_all().await.map_err(MystikoError::StorageError)
    }

    pub async fn initialize(&self) -> Result<Vec<ProtoContract>> {
        let mut insert_contracts: Vec<Contract> = vec![];
        let mut update_contracts: Vec<Document<Contract>> = vec![];
        let mut contracts: Vec<Document<Contract>> = vec![];
        for chain_config in self.config.chains() {
            for contract_config in chain_config.contracts_with_disabled().iter() {
                let event_filter_size = chain_config.contract_event_filter_size(contract_config.address());
                if let Some(existing_contract) = self
                    .find_by_address(chain_config.chain_id(), contract_config.address())
                    .await?
                {
                    let mut existing_contract = Contract::from_proto(existing_contract);
                    existing_contract.data.sync_size = event_filter_size;
                    existing_contract.data.disabled = contract_config.disabled();
                    update_contracts.push(existing_contract);
                } else {
                    insert_contracts.push(Contract {
                        chain_id: chain_config.chain_id(),
                        contract_address: contract_config.address().to_string(),
                        contract_type: contract_config.contract_type().clone(),
                        sync_size: event_filter_size,
                        sync_start: contract_config.start_block(),
                        synced_block_number: contract_config.start_block(),
                        disabled: contract_config.disabled(),
                        checked_leaf_index: None,
                    });
                }
            }
        }
        contracts.extend(
            self.db
                .contracts
                .insert_batch(&insert_contracts)
                .await
                .map_err(MystikoError::StorageError)?,
        );
        contracts.extend(
            self.db
                .contracts
                .update_batch(&update_contracts)
                .await
                .map_err(MystikoError::StorageError)?,
        );
        Ok(contracts
            .iter()
            .map(|contract| Contract::into_proto(contract.clone()))
            .collect())
    }

    pub async fn reset_synced_block(&self, chain_id: u64, address: &str) -> Result<Option<ProtoContract>> {
        self.rs_synced_block(chain_id, address, None).await
    }

    pub async fn reset_synced_block_to(
        &self,
        chain_id: u64,
        address: &str,
        to_block: u64,
    ) -> Result<Option<ProtoContract>> {
        self.rs_synced_block(chain_id, address, Some(to_block)).await
    }

    async fn rs_synced_block(
        &self,
        chain_id: u64,
        address: &str,
        to_block: Option<u64>,
    ) -> Result<Option<ProtoContract>> {
        if let Some(existing_contract) = self.find_by_address(chain_id, address).await? {
            let mut existing_contract = Contract::from_proto(existing_contract);
            existing_contract.data.synced_block_number = to_block.unwrap_or(existing_contract.data.sync_start);
            return Ok(Some(Contract::into_proto(
                self.db
                    .contracts
                    .update(&existing_contract)
                    .await
                    .map_err(MystikoError::StorageError)?,
            )));
        }
        Ok(None)
    }
}
