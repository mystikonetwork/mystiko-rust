pub mod document;

mod utils;
pub use utils::*;

use crate::data::{ChainData, ChainResult, ContractData, ContractResult, DataRef, DataType, LoadedData};
use crate::handler::document::{DatabaseCommitment, DatabaseContract, DatabaseNullifier};
use crate::handler::{
    CommitmentQueryOption, DataHandler, HandleOption, HandleResult, HandlerError, NullifierQueryOption, QueryResult,
};
use anyhow::Result;
use async_trait::async_trait;
use mystiko_config::{ChainConfig, MystikoConfig};
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_protos::storage::v1::{Condition, QueryFilter, SubFilter};
use mystiko_storage::{Collection, Document, DocumentData, MigrationHistory, StatementFormatter, Storage};
use mystiko_utils::time::current_timestamp;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, thiserror::Error)]
pub enum DatabaseHandlerError {
    #[error("unsupported chain id: {0}")]
    UnsupportedChainError(u64),
    #[error("unsupported contract with chain id: {0}, address: {1}")]
    UnsupportedContractError(u64, String),
    #[error("unexpected database contract error: {0}")]
    UnexpectedDatabaseContractError(String),
    #[error("unsupported handler data type")]
    UnsupportedHandlerDataType,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DatabaseHandler<
    R: LoadedData,
    F: StatementFormatter,
    S: Storage,
    X: DocumentData + DatabaseContract = document::Contract,
    C: DocumentData + DatabaseCommitment = document::Commitment,
    N: DocumentData + DatabaseNullifier = document::Nullifier,
> {
    config: Arc<MystikoConfig>,
    collection: Arc<Collection<F, S>>,
    #[builder(default = 10000)]
    pub handle_batch_size: usize,
    #[builder(default = 1)]
    pub handle_concurrency: usize,
    #[builder(default, setter(skip))]
    _phantom: std::marker::PhantomData<(R, X, C, N)>,
}
#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
struct HandlerContractData<
    C: DocumentData + DatabaseCommitment = document::Commitment,
    N: DocumentData + DatabaseNullifier = document::Nullifier,
> {
    pub(crate) insert_commitments: Vec<C>,
    pub(crate) insert_nullifiers: Vec<N>,
    pub(crate) update_commitments: Vec<Document<C>>,
    pub(crate) update_nullifiers: Vec<Document<N>>,
}

#[async_trait]
impl<R, F, S, X, C, N> DataHandler<R> for DatabaseHandler<R, F, S, X, C, N>
where
    R: LoadedData,
    F: StatementFormatter,
    S: Storage,
    X: DocumentData + DatabaseContract,
    C: DocumentData + DatabaseCommitment,
    N: DocumentData + DatabaseNullifier,
{
    async fn query_chain_loaded_block(&self, chain_id: u64) -> Result<Option<u64>, HandlerError> {
        let contracts: Vec<Document<X>> = self
            .collection
            .find(Some(SubFilter::equal(X::column_chain_id(), chain_id)))
            .await
            .map_err(anyhow::Error::new)?;
        Ok(contracts.into_iter().map(|c| c.data.get_loaded_block()).min())
    }

    async fn query_contract_loaded_block(
        &self,
        chain_id: u64,
        contract_address: &str,
    ) -> Result<Option<u64>, HandlerError> {
        let contract: Option<Document<X>> = self
            .collection
            .find_one(Some(Condition::and(vec![
                SubFilter::equal(X::column_chain_id(), chain_id),
                SubFilter::equal(X::column_contract_address(), contract_address),
            ])))
            .await
            .map_err(anyhow::Error::new)?;
        Ok(contract.map(|c| c.data.get_loaded_block()))
    }

    async fn query_commitment(&self, option: &CommitmentQueryOption) -> crate::handler::Result<Option<Commitment>> {
        let (filter, _) = self.to_commitment_filter(option).await?;
        let commitment: Option<Document<C>> = self
            .collection
            .find_one(Some(filter))
            .await
            .map_err(anyhow::Error::new)?;
        Ok(commitment.map(|c| c.data.into_proto()).transpose()?)
    }

    async fn query_commitments(
        &self,
        option: &CommitmentQueryOption,
    ) -> Result<QueryResult<Vec<Commitment>>, HandlerError> {
        let (filter, actual_end_block) = self.to_commitment_filter(option).await?;
        let commitments: Vec<Document<C>> = self.collection.find(Some(filter)).await.map_err(anyhow::Error::new)?;
        Ok(QueryResult::builder()
            .end_block(actual_end_block)
            .result(
                commitments
                    .into_iter()
                    .map(|c| c.data.into_proto())
                    .collect::<Result<Vec<_>>>()?,
            )
            .build())
    }

    async fn count_commitments(&self, option: &CommitmentQueryOption) -> Result<QueryResult<u64>, HandlerError> {
        let (filter, actual_end_block) = self.to_commitment_filter(option).await?;
        let count = self
            .collection
            .count::<C, QueryFilter>(Some(filter))
            .await
            .map_err(anyhow::Error::new)?;
        Ok(QueryResult::builder().end_block(actual_end_block).result(count).build())
    }

    async fn query_nullifier(&self, option: &NullifierQueryOption) -> crate::handler::Result<Option<Nullifier>> {
        if R::data_type() == DataType::Lite {
            return Err(HandlerError::AnyhowError(
                DatabaseHandlerError::UnsupportedHandlerDataType.into(),
            ));
        }
        let (filter, _) = self.to_nullifier_filter(option).await?;
        let nullifier: Option<Document<N>> = self
            .collection
            .find_one(Some(filter))
            .await
            .map_err(anyhow::Error::new)?;
        Ok(nullifier.map(|n| n.data.into_proto()).transpose()?)
    }

    async fn query_nullifiers(
        &self,
        option: &NullifierQueryOption,
    ) -> Result<QueryResult<Vec<Nullifier>>, HandlerError> {
        if R::data_type() == DataType::Lite {
            return Err(HandlerError::AnyhowError(
                DatabaseHandlerError::UnsupportedHandlerDataType.into(),
            ));
        }
        let (filter, actual_end_block) = self.to_nullifier_filter(option).await?;
        let nullifiers: Vec<Document<N>> = self.collection.find(Some(filter)).await.map_err(anyhow::Error::new)?;
        Ok(QueryResult::builder()
            .end_block(actual_end_block)
            .result(
                nullifiers
                    .into_iter()
                    .map(|c| c.data.into_proto())
                    .collect::<Result<Vec<_>>>()?,
            )
            .build())
    }

    async fn count_nullifiers(&self, option: &NullifierQueryOption) -> Result<QueryResult<u64>, HandlerError> {
        if R::data_type() == DataType::Lite {
            return Err(HandlerError::AnyhowError(
                DatabaseHandlerError::UnsupportedHandlerDataType.into(),
            ));
        }
        let (filter, actual_end_block) = self.to_nullifier_filter(option).await?;
        let count = self
            .collection
            .count::<N, QueryFilter>(Some(filter))
            .await
            .map_err(anyhow::Error::new)?;
        Ok(QueryResult::builder().end_block(actual_end_block).result(count).build())
    }

    async fn handle(&self, data: &ChainData<R>, _option: &HandleOption) -> HandleResult {
        let mut contract_tasks = vec![];
        let chunks = data.contracts_data.chunks(self.handle_concurrency);
        for contracts_chunk in chunks {
            contract_tasks.push(self.handle_contracts(data.chain_id, contracts_chunk));
        }
        let results = futures::future::try_join_all(contract_tasks)
            .await?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        Ok(ChainResult::builder()
            .chain_id(data.chain_id)
            .contract_results(results)
            .build())
    }
}

impl<R, F, S, X, C, N> DatabaseHandler<R, F, S, X, C, N>
where
    R: LoadedData,
    F: StatementFormatter,
    S: Storage,
    X: DocumentData + DatabaseContract,
    C: DocumentData + DatabaseCommitment,
    N: DocumentData + DatabaseNullifier,
{
    pub async fn initialize(&self) -> Result<()> {
        let existing_contracts = self
            .collection
            .find::<X, QueryFilter>(None)
            .await?
            .into_iter()
            .map(|contract: Document<X>| {
                format!(
                    "{}/{}",
                    contract.data.get_chain_id(),
                    contract.data.get_contract_address()
                )
            })
            .collect::<HashSet<_>>();
        let mut insert_contracts: Vec<X> = Vec::new();
        for chain_config in self.config.chains().into_iter() {
            for contract_config in chain_config.contracts_with_disabled().into_iter() {
                let insert_contract =
                    X::from_config(self.config.clone(), chain_config.chain_id(), contract_config.address())?;
                if !existing_contracts.contains(&format!("{}/{}", chain_config.chain_id(), contract_config.address())) {
                    insert_contracts.push(insert_contract);
                }
            }
        }
        self.collection.insert_batch::<X>(&insert_contracts).await?;
        log::info!(
            "database is successfully initialized, inserted {} new contracts from mystiko config",
            insert_contracts.len()
        );
        Ok(())
    }

    pub async fn migrate(&self) -> Result<Vec<Document<MigrationHistory>>> {
        let mut migrations = vec![
            self.collection.migrate::<X>().await?,
            self.collection.migrate::<C>().await?,
        ];
        if R::data_type() == DataType::Full {
            migrations.push(self.collection.migrate::<N>().await?);
        }
        Ok(migrations)
    }

    async fn to_commitment_filter(&self, option: &CommitmentQueryOption) -> Result<(QueryFilter, u64)> {
        let chain_config = self.config.find_chain(option.chain_id).ok_or(anyhow::anyhow!(
            DatabaseHandlerError::UnsupportedChainError(option.chain_id)
        ))?;
        let actual_end_block = self
            .wrap_contract_end_block(chain_config, option.contract_address.as_str(), option.end_block)
            .await?;
        let mut sub_filters = vec![];
        sub_filters.push(SubFilter::equal(C::column_chain_id(), option.chain_id));
        sub_filters.push(SubFilter::equal(
            C::column_contract_address(),
            option.contract_address.to_string(),
        ));
        let start_block = option.start_block.unwrap_or(chain_config.start_block());
        let block_num_column = match option.status {
            Some(status) => match status {
                CommitmentStatus::Unspecified => {
                    sub_filters.push(SubFilter::equal(
                        C::column_status(),
                        CommitmentStatus::Unspecified as i32,
                    ));
                    C::column_block_number()
                }
                CommitmentStatus::SrcSucceeded => {
                    sub_filters.push(SubFilter::equal(
                        C::column_status(),
                        CommitmentStatus::SrcSucceeded as i32,
                    ));
                    C::column_src_block_number()
                }
                CommitmentStatus::Queued => {
                    sub_filters.push(SubFilter::equal(C::column_status(), CommitmentStatus::Queued as i32));
                    C::column_block_number()
                }
                CommitmentStatus::Included => {
                    sub_filters.push(SubFilter::equal(C::column_status(), CommitmentStatus::Included as i32));
                    C::column_included_block_number()
                }
            },
            None => C::column_block_number(),
        };
        sub_filters.push(SubFilter::between_and(block_num_column, start_block, actual_end_block));
        if let Some(commitment_hashes) = option.commitment_hash.clone() {
            sub_filters.push(SubFilter::in_list(C::column_commitment_hash(), commitment_hashes));
        }
        Ok((QueryFilter::from(sub_filters), actual_end_block))
    }

    async fn to_nullifier_filter(&self, option: &NullifierQueryOption) -> Result<(QueryFilter, u64)> {
        let chain_config = self.config.find_chain(option.chain_id).ok_or(anyhow::anyhow!(
            DatabaseHandlerError::UnsupportedChainError(option.chain_id)
        ))?;
        let actual_end_block = self
            .wrap_contract_end_block(chain_config, option.contract_address.as_str(), option.end_block)
            .await?;
        let mut sub_filters = vec![];
        sub_filters.push(SubFilter::equal(N::column_chain_id(), option.chain_id));
        sub_filters.push(SubFilter::equal(
            N::column_contract_address(),
            option.contract_address.to_string(),
        ));
        let start_block = option.start_block.unwrap_or(chain_config.start_block() + 1);
        sub_filters.push(SubFilter::between_and(
            N::column_block_number(),
            start_block,
            actual_end_block,
        ));
        if let Some(nullifiers) = option.nullifier.clone() {
            sub_filters.push(SubFilter::in_list(N::column_nullifier(), nullifiers));
        }
        Ok((QueryFilter::from(sub_filters), actual_end_block))
    }

    async fn handle_contracts(
        &self,
        chain_id: u64,
        contracts_data: &[ContractData<R>],
    ) -> Result<Vec<ContractResult<()>>> {
        let mut results = Vec::with_capacity(contracts_data.len());
        for contract_data in contracts_data.iter() {
            results.push(self.handle_contract(chain_id, contract_data).await?);
        }
        Ok(results)
    }

    async fn handle_contract(&self, chain_id: u64, contract_data: &ContractData<R>) -> Result<ContractResult<()>> {
        let contract: Option<Document<X>> = self
            .collection
            .find_one(Some(Condition::and(vec![
                SubFilter::equal(X::column_chain_id(), chain_id),
                SubFilter::equal(X::column_contract_address(), contract_data.address.to_string()),
            ])))
            .await?;
        let result = if let Some(contract) = contract {
            self.handle_contract_data(chain_id, contract_data, contract).await
        } else {
            Err(anyhow::anyhow!(DatabaseHandlerError::UnsupportedContractError(
                chain_id,
                contract_data.address.to_string()
            )))
        };
        Ok(ContractResult::builder()
            .address(contract_data.address.to_string())
            .result(result)
            .build())
    }

    async fn handle_contract_data(
        &self,
        chain_id: u64,
        contract_data: &ContractData<R>,
        mut contract: Document<X>,
    ) -> Result<()> {
        if let Some(data) = &contract_data.data {
            let contract_update = match data.data_ref() {
                DataRef::Full(full_data) => {
                    let (insert_commitments, update_commitments) = self
                        .handle_contract_commitments(
                            chain_id,
                            &contract_data.address,
                            dedup_commitments(full_data.commitments.clone()),
                        )
                        .await?;
                    let (insert_nullifiers, update_nullifiers) = self
                        .handle_contract_nullifiers(
                            chain_id,
                            &contract_data.address,
                            dedup_nullifiers(full_data.nullifiers.clone()),
                        )
                        .await?;
                    HandlerContractData::<C, N>::builder()
                        .insert_commitments(insert_commitments)
                        .insert_nullifiers(insert_nullifiers)
                        .update_commitments(update_commitments)
                        .update_nullifiers(update_nullifiers)
                        .build()
                }
                DataRef::Lite(lite_data) => {
                    let (insert_commitments, update_commitments) = self
                        .handle_contract_commitments(
                            chain_id,
                            &contract_data.address,
                            dedup_commitments(lite_data.commitments.clone()),
                        )
                        .await?;
                    HandlerContractData::<C, N>::builder()
                        .insert_commitments(insert_commitments)
                        .update_commitments(update_commitments)
                        .build()
                }
            };
            contract.data.update_loaded_block(contract_data.end_block);
            contract.updated_at = current_timestamp();
            self.upsert_contract_data(contract_update, &contract).await?;
        }
        Ok(())
    }

    async fn handle_contract_commitments(
        &self,
        chain_id: u64,
        contract_address: &str,
        commitments: Vec<Commitment>,
    ) -> Result<(Vec<C>, Vec<Document<C>>)> {
        let mut insert_commitments: Vec<C> = Vec::new();
        let mut update_commitments: Vec<Document<C>> = Vec::new();
        for commitment_chunk in commitments.chunks(self.handle_batch_size) {
            let existing_commitments: Vec<Document<C>> = self
                .collection
                .find(Some(Condition::and(vec![
                    SubFilter::equal(C::column_chain_id(), chain_id),
                    SubFilter::equal(C::column_contract_address(), contract_address.to_string()),
                    SubFilter::in_list(
                        C::column_commitment_hash(),
                        commitment_chunk
                            .iter()
                            .map(|p| p.commitment_hash_as_biguint())
                            .collect::<Vec<_>>(),
                    ),
                ])))
                .await?;
            let mut existing_commitments_map = existing_commitments
                .into_iter()
                .map(|commitment| (commitment.data.get_commitment_hash().clone(), commitment))
                .collect::<HashMap<_, _>>();
            for commitment_proto in commitment_chunk {
                if let Some(mut existing_commitment) =
                    existing_commitments_map.remove(&commitment_proto.commitment_hash_as_biguint())
                {
                    existing_commitment
                        .data
                        .update_by_proto(self.config.clone(), commitment_proto)?;
                    existing_commitment.updated_at = current_timestamp();
                    update_commitments.push(existing_commitment);
                } else {
                    insert_commitments.push(C::from_proto(
                        self.config.clone(),
                        chain_id,
                        contract_address,
                        commitment_proto.clone(),
                    )?);
                }
            }
        }

        Ok((insert_commitments, update_commitments))
    }

    async fn handle_contract_nullifiers(
        &self,
        chain_id: u64,
        contract_address: &str,
        nullifiers: Vec<Nullifier>,
    ) -> Result<(Vec<N>, Vec<Document<N>>)> {
        let mut insert_nullifiers: Vec<N> = Vec::new();
        let mut update_nullifiers: Vec<Document<N>> = Vec::new();
        for nullifier_chunks in nullifiers.chunks(self.handle_batch_size) {
            let existing_nullifiers: Vec<Document<N>> = self
                .collection
                .find(Some(Condition::and(vec![
                    SubFilter::equal(N::column_chain_id(), chain_id),
                    SubFilter::equal(N::column_contract_address(), contract_address.to_string()),
                    SubFilter::in_list(
                        N::column_nullifier(),
                        nullifier_chunks
                            .iter()
                            .map(|n| n.nullifier_as_biguint())
                            .collect::<Vec<_>>(),
                    ),
                ])))
                .await?;
            let mut existing_nullifiers_map = existing_nullifiers
                .into_iter()
                .map(|nullifier| (nullifier.data.get_nullifier().clone(), nullifier))
                .collect::<HashMap<_, _>>();
            for nullifier_proto in nullifier_chunks {
                if let Some(mut existing_nullifier) =
                    existing_nullifiers_map.remove(&nullifier_proto.nullifier_as_biguint())
                {
                    existing_nullifier
                        .data
                        .update_by_proto(self.config.clone(), nullifier_proto)?;
                    existing_nullifier.updated_at = current_timestamp();
                    update_nullifiers.push(existing_nullifier);
                } else {
                    insert_nullifiers.push(N::from_proto(
                        self.config.clone(),
                        chain_id,
                        contract_address,
                        nullifier_proto.clone(),
                    )?);
                }
            }
        }
        Ok((insert_nullifiers, update_nullifiers))
    }

    async fn upsert_contract_data(
        &self,
        handler_data: HandlerContractData<C, N>,
        contract: &Document<X>,
    ) -> Result<()> {
        let HandlerContractData {
            insert_commitments,
            insert_nullifiers,
            update_commitments,
            update_nullifiers,
        } = handler_data;
        self.collection.insert_batch::<C>(&insert_commitments).await?;
        self.collection.update_batch::<C>(&update_commitments).await?;
        if R::data_type() == DataType::Full {
            self.collection.insert_batch::<N>(&insert_nullifiers).await?;
            self.collection.update_batch::<N>(&update_nullifiers).await?;
        }
        self.collection.update::<X>(contract).await?;
        if R::data_type() == DataType::Full {
            log::info!(
                "data of contract[chain_id={}, address={}] were handled successfully: \
                inserted {} new commitments, \
                updated {} existing commitments, \
                inserted {} new nullifiers, \
                updated {} existing nullifiers",
                contract.data.get_chain_id(),
                contract.data.get_contract_address(),
                insert_commitments.len(),
                update_commitments.len(),
                insert_nullifiers.len(),
                update_nullifiers.len(),
            );
        } else {
            log::info!(
                "data of contract[chain_id={}, address={}] were handled successfully: \
                inserted {} new commitments, \
                updated {} existing commitments",
                contract.data.get_chain_id(),
                contract.data.get_contract_address(),
                insert_commitments.len(),
                update_commitments.len(),
            );
        }
        Ok(())
    }

    async fn wrap_contract_end_block(
        &self,
        chain_config: &ChainConfig,
        contract_address: &str,
        end_block: u64,
    ) -> Result<u64> {
        let contract_loaded_block = self
            .query_contract_loaded_block(chain_config.chain_id(), contract_address)
            .await?
            .unwrap_or(chain_config.start_block());
        Ok(std::cmp::min(contract_loaded_block, end_block))
    }
}
