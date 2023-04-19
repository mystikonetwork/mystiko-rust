use crate::error::MystikoError;
use crate::types::Result;
use mystiko_config::wrapper::chain::ChainConfig;
use mystiko_config::wrapper::contract::deposit::DepositContractConfig;
use mystiko_config::wrapper::contract::pool::PoolContractConfig;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_database::database::Database;
use mystiko_database::document::contract::{
    Contract, CHAIN_ID_FIELD_NAME, CONTRACT_ADDRESS_FIELD_NAME,
};
use mystiko_storage::document::{Document, DocumentRawData, DOCUMENT_ID_FIELD};
use mystiko_storage::filter::{Condition, QueryFilter, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::storage::Storage;
use mystiko_types::ContractType;
use std::sync::Arc;

pub struct ContractHandler<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    db: Arc<Database<F, R, S>>,
    config: Arc<MystikoConfig>,
}

impl<F, R, S> ContractHandler<F, R, S>
where
    F: StatementFormatter,
    R: DocumentRawData,
    S: Storage<R>,
{
    pub fn new(db: Arc<Database<F, R, S>>, config: Arc<MystikoConfig>) -> Self {
        Self { db, config }
    }

    pub async fn find(&self, query_filter: QueryFilter) -> Result<Vec<Document<Contract>>> {
        self.db
            .contracts
            .find(query_filter)
            .await
            .map_err(MystikoError::DatabaseError)
    }

    pub async fn find_all(&self) -> Result<Vec<Document<Contract>>> {
        self.db
            .contracts
            .find_all()
            .await
            .map_err(MystikoError::DatabaseError)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Document<Contract>>> {
        let query_filter = QueryFilterBuilder::new()
            .filter(Condition::FILTER(SubFilter::Equal(
                DOCUMENT_ID_FIELD.to_string(),
                id.to_string(),
            )))
            .build();
        self.db
            .contracts
            .find_one(query_filter)
            .await
            .map_err(MystikoError::DatabaseError)
    }

    pub async fn find_by_address(
        &self,
        chain_id: u64,
        address: &str,
    ) -> Result<Option<Document<Contract>>> {
        let query_filter = QueryFilterBuilder::new()
            .filter(Condition::FILTER(SubFilter::Equal(
                CHAIN_ID_FIELD_NAME.to_string(),
                format!("{}", chain_id),
            )))
            .filter(Condition::FILTER(SubFilter::Equal(
                CONTRACT_ADDRESS_FIELD_NAME.to_string(),
                address.to_string(),
            )))
            .build();
        self.db
            .contracts
            .find_one(query_filter)
            .await
            .map_err(MystikoError::DatabaseError)
    }

    pub async fn count(&self, query_filter: QueryFilter) -> Result<u64> {
        self.db
            .contracts
            .count(query_filter)
            .await
            .map_err(MystikoError::DatabaseError)
    }

    pub async fn count_all(&self) -> Result<u64> {
        self.db
            .contracts
            .count_all()
            .await
            .map_err(MystikoError::DatabaseError)
    }

    pub async fn initialize(&self) -> Result<Vec<Document<Contract>>> {
        let mut contracts: Vec<Document<Contract>> = vec![];
        for chain_config in self.config.chains() {
            for deposit_contract_config in chain_config.deposit_contracts_with_disabled() {
                let contract = self
                    .upsert_deposit_contract(chain_config, deposit_contract_config)
                    .await?;
                contracts.push(contract);
            }
            for pool_contract_config in chain_config.pool_contracts() {
                let contract = self
                    .upsert_pool_contract(chain_config, pool_contract_config)
                    .await?;
                contracts.push(contract);
            }
        }
        Ok(contracts)
    }

    async fn upsert_deposit_contract(
        &self,
        chain_config: &ChainConfig,
        contract_config: &DepositContractConfig,
    ) -> Result<Document<Contract>> {
        let event_filter_size = chain_config.contract_event_filter_size(contract_config.address());
        if let Some(existing_contract) = self
            .update_contract(
                chain_config.chain_id(),
                contract_config.address(),
                event_filter_size,
                contract_config.disabled(),
            )
            .await?
        {
            return Ok(existing_contract);
        }
        self.db
            .contracts
            .insert(&Contract {
                chain_id: chain_config.chain_id(),
                contract_address: contract_config.address().to_string(),
                contract_type: ContractType::Deposit,
                sync_size: event_filter_size,
                sync_start: contract_config.start_block(),
                synced_block_number: contract_config.start_block(),
                disabled: contract_config.disabled(),
                checked_leaf_index: None,
            })
            .await
            .map_err(MystikoError::DatabaseError)
    }

    async fn upsert_pool_contract(
        &self,
        chain_config: &ChainConfig,
        contract_config: &PoolContractConfig,
    ) -> Result<Document<Contract>> {
        let event_filter_size = chain_config.contract_event_filter_size(contract_config.address());
        if let Some(existing_contract) = self
            .update_contract(
                chain_config.chain_id(),
                contract_config.address(),
                event_filter_size,
                false,
            )
            .await?
        {
            return Ok(existing_contract);
        }
        self.db
            .contracts
            .insert(&Contract {
                chain_id: chain_config.chain_id(),
                contract_address: contract_config.address().to_string(),
                contract_type: ContractType::Pool,
                sync_size: event_filter_size,
                sync_start: contract_config.start_block(),
                synced_block_number: contract_config.start_block(),
                disabled: false,
                checked_leaf_index: None,
            })
            .await
            .map_err(MystikoError::DatabaseError)
    }

    async fn update_contract(
        &self,
        chain_id: u64,
        address: &str,
        event_filter_size: u64,
        disabled: bool,
    ) -> Result<Option<Document<Contract>>> {
        if let Some(mut existing_contract) = self.find_by_address(chain_id, address).await? {
            existing_contract.data.sync_size = event_filter_size;
            existing_contract.data.disabled = disabled;
            return Ok(Some(
                self.db
                    .contracts
                    .update(&existing_contract)
                    .await
                    .map_err(MystikoError::DatabaseError)?,
            ));
        }
        Ok(None)
    }
}
