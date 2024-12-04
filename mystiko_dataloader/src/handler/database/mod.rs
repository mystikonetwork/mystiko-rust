pub mod document;

mod utils;
pub use utils::*;

use crate::data::{ChainData, ChainResult, ContractData, ContractResult, DataRef, DataType, LoadedData};
use crate::handler::document::{DatabaseCommitment, DatabaseContract, DatabaseNullifier};
use crate::handler::{
    CommitmentQueryOption, DataHandler, HandleOption, HandleResult, HandlerError, NullifierQueryOption, QueryResult,
};
use crate::loader::ResetOptions;
use anyhow::Result;
use async_trait::async_trait;
use mystiko_config::{ChainConfig, MystikoConfig};
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_protos::storage::v1::{Condition, ConditionOperator, QueryFilter, SubFilter};
use mystiko_storage::{Collection, Document, DocumentData, MigrationHistory, StatementFormatter, Storage};
use mystiko_utils::time::current_timestamp;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use typed_builder::TypedBuilder;

pub const DEFAULT_HANDLE_BATCH_SIZE: usize = 10000;
pub const DEFAULT_HANDLE_CONCURRENCY: usize = 1;

#[derive(Debug, thiserror::Error)]
pub enum DatabaseHandlerError {
    #[error("unsupported chain id: {0}")]
    UnsupportedChainError(u64),
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
    #[builder(default)]
    pub handle_batch_size: Option<usize>,
    #[builder(default)]
    pub handle_concurrency: Option<usize>,
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
        let result = self.query_commitments(option).await?;
        Ok(result.result.into_iter().next())
    }

    async fn query_commitments(
        &self,
        option: &CommitmentQueryOption,
    ) -> Result<QueryResult<Vec<Commitment>>, HandlerError> {
        let (filter, actual_end_block) = self.to_commitment_filter(option).await?;
        let commitments: Vec<Document<C>> = self.collection.find(Some(filter)).await.map_err(anyhow::Error::new)?;
        let commitments: Vec<Commitment> = commitments
            .into_iter()
            .map(|c| c.data.into_proto())
            .collect::<Result<Vec<_>>>()?;
        Ok(QueryResult::builder()
            .end_block(actual_end_block)
            .result(recover_commitments_status(
                commitments,
                actual_end_block,
                &option.status,
            ))
            .build())
    }

    async fn count_commitments(&self, option: &CommitmentQueryOption) -> Result<QueryResult<u64>, HandlerError> {
        let result = self.query_commitments(option).await?;
        Ok(QueryResult::builder()
            .end_block(result.end_block)
            .result(result.result.len() as u64)
            .build())
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
        let concurrency = self.handle_concurrency.unwrap_or(DEFAULT_HANDLE_CONCURRENCY).max(1);
        let chunk_size = data.contracts_data.len().div_ceil(concurrency);
        let chunks = data.contracts_data.chunks(chunk_size);
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

    async fn reset(&self, options: &ResetOptions) -> Result<(), HandlerError> {
        let loaded_block = self.build_reset_loaded_block(options).await?;
        self.reset_contract_loaded_block(options, loaded_block).await?;
        self.delete_commitments_by_loaded_block(options, loaded_block).await?;
        self.delete_nullifiers_by_loaded_block(options, loaded_block).await?;
        Ok(())
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
        let existing_contracts: Vec<Document<X>> = self.collection.find::<X, QueryFilter>(None).await?;
        let existing_contracts_set = existing_contracts
            .iter()
            .map(|contract| {
                format!(
                    "{}/{}",
                    contract.data.get_chain_id(),
                    contract.data.get_contract_address()
                )
            })
            .collect::<HashSet<_>>();
        let mut insert_contracts: Vec<X> = Vec::new();
        for chain_config in self.config.chains().into_iter() {
            for contract_config in chain_config.contracts().into_iter() {
                let insert_contract =
                    X::from_config(self.config.clone(), chain_config.chain_id(), contract_config.address())?;
                if !existing_contracts_set.contains(&format!(
                    "{}/{}",
                    chain_config.chain_id(),
                    contract_config.address()
                )) {
                    insert_contracts.push(insert_contract);
                }
            }
        }
        self.collection.insert_batch::<X>(&insert_contracts).await?;
        let mut delete_contracts: Vec<Document<X>> = Vec::new();
        for existing_contract in existing_contracts.into_iter() {
            if self
                .config
                .find_contract_by_address(
                    existing_contract.data.get_chain_id(),
                    existing_contract.data.get_contract_address(),
                )
                .is_none()
            {
                delete_contracts.push(existing_contract);
            }
        }
        self.collection.delete_batch(&delete_contracts).await?;
        log::info!(
            "database is successfully initialized, \
            inserted {} new contracts, \
            deleted {} deprecated contracts \
            from mystiko config",
            insert_contracts.len(),
            delete_contracts.len(),
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
        let start_block = option.start_block.unwrap_or(chain_config.start_block());
        let actual_end_block = self
            .wrap_contract_end_block(chain_config, option.contract_address.as_str(), option.end_block)
            .await?;
        let mut conditions = vec![
            Condition::from(SubFilter::equal(C::column_chain_id(), option.chain_id)),
            Condition::from(SubFilter::equal(
                C::column_contract_address(),
                option.contract_address.to_string(),
            )),
            Condition::builder()
                .sub_filters(vec![
                    SubFilter::between_and(C::column_src_block_number(), start_block, actual_end_block),
                    SubFilter::between_and(C::column_block_number(), start_block, actual_end_block),
                    SubFilter::between_and(C::column_included_block_number(), start_block, actual_end_block),
                ])
                .operator(ConditionOperator::Or)
                .build(),
        ];
        if let Some(commitment_hashes) = option.commitment_hash.clone() {
            conditions.push(Condition::from(SubFilter::in_list(
                C::column_commitment_hash(),
                commitment_hashes,
            )));
        }
        Ok((
            QueryFilter::builder()
                .conditions(conditions)
                .conditions_operator(ConditionOperator::And)
                .build(),
            actual_end_block,
        ))
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
            Ok(())
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
        for commitment_chunk in commitments.chunks(self.handle_batch_size.unwrap_or(DEFAULT_HANDLE_BATCH_SIZE)) {
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
        for nullifier_chunks in nullifiers.chunks(self.handle_batch_size.unwrap_or(DEFAULT_HANDLE_BATCH_SIZE)) {
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

    async fn build_reset_loaded_block(&self, options: &ResetOptions) -> Result<u64> {
        let start_block = self
            .config
            .find_chain(options.chain_id)
            .ok_or_else(|| {
                HandlerError::AnyhowError(DatabaseHandlerError::UnsupportedChainError(options.chain_id).into())
            })?
            .start_block();
        let block_number = options
            .block_number
            .map_or_else(|| start_block, |block| std::cmp::max(block, start_block));
        Ok(block_number)
    }

    async fn reset_contract_loaded_block(&self, options: &ResetOptions, loaded_block: u64) -> Result<()> {
        let condition = build_reset_condition(
            options,
            loaded_block,
            X::column_chain_id(),
            X::column_contract_address(),
            X::column_loaded_block(),
        );
        self.collection
            .update_by_filter::<X, _, _>((X::column_loaded_block(), loaded_block), Some(condition))
            .await?;
        Ok(())
    }

    async fn delete_commitments_by_loaded_block(&self, options: &ResetOptions, loaded_block: u64) -> Result<()> {
        let condition = build_remove_condition(
            options,
            loaded_block,
            C::column_chain_id(),
            C::column_contract_address(),
            C::column_block_number(),
        );
        self.collection.delete_by_filter::<C, _>(Some(condition)).await?;
        Ok(())
    }

    async fn delete_nullifiers_by_loaded_block(&self, options: &ResetOptions, loaded_block: u64) -> Result<()> {
        let condition = build_remove_condition(
            options,
            loaded_block,
            N::column_chain_id(),
            N::column_contract_address(),
            N::column_block_number(),
        );
        self.collection.delete_by_filter::<N, _>(Some(condition)).await?;
        Ok(())
    }
}

fn recover_commitments_status(
    mut commitments: Vec<Commitment>,
    end_block: u64,
    status: &Option<CommitmentStatus>,
) -> Vec<Commitment> {
    for commitment in commitments.iter_mut() {
        if commitment.status == CommitmentStatus::Included as i32 {
            if let Some(included_block_number) = commitment.included_block_number {
                if included_block_number > end_block {
                    commitment.status = CommitmentStatus::Queued as i32;
                }
            }
        }
    }
    if let Some(status) = status {
        commitments.into_iter().filter(|c| c.status == *status as i32).collect()
    } else {
        commitments
    }
}

fn build_reset_condition(
    options: &ResetOptions,
    loaded_block: u64,
    chain_id_column: String,
    contract_address_column: String,
    loaded_block_column: String,
) -> Condition {
    let mut sub_filters = build_reset_filters(options, chain_id_column, contract_address_column);
    sub_filters.push(SubFilter::greater(loaded_block_column, loaded_block));
    Condition::and(sub_filters)
}

fn build_remove_condition(
    options: &ResetOptions,
    loaded_block: u64,
    chain_id_column: String,
    contract_address_column: String,
    block_number_column: String,
) -> Condition {
    let mut sub_filters = build_reset_filters(options, chain_id_column, contract_address_column);
    sub_filters.push(SubFilter::greater(block_number_column, loaded_block));
    Condition::and(sub_filters)
}

fn build_reset_filters(
    options: &ResetOptions,
    chain_id_column: String,
    contract_address_column: String,
) -> Vec<SubFilter> {
    let mut sub_filters = vec![SubFilter::equal(chain_id_column, options.chain_id)];
    if !options.contract_addresses.is_empty() {
        sub_filters.push(SubFilter::in_list(
            contract_address_column,
            options.contract_addresses.clone(),
        ));
    }
    sub_filters
}
