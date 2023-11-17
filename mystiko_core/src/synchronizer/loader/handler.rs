use crate::{CommitmentColumn, Database, Deposit, DepositColumn};
use async_trait::async_trait;
use mystiko_config::ContractConfig;
use mystiko_dataloader::data::{ChainData, ChainResult, ContractData, FullData};
use mystiko_dataloader::handler::{
    dedup_commitments, CommitmentQueryOption, DataHandler, HandleOption, NullifierQueryOption, QueryResult,
};
use mystiko_dataloader::handler::{HandlerError, Result as handlerResult};
use mystiko_dataloader::loader::ResetOptions;
use mystiko_protos::core::v1::DepositStatus;
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_protos::storage::v1::{Condition, SubFilter};
use mystiko_storage::{Document, StatementFormatter, Storage, StorageError};
use mystiko_utils::convert::bytes_to_biguint;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use thiserror::Error;
use typed_builder::TypedBuilder;

type SyncLoaderHandlerResult<T> = Result<T, SyncLoaderHandlerError>;

#[derive(Debug, Error)]
pub enum SyncLoaderHandlerError {
    #[error("invalid commitment status: {0}")]
    CommitmentStatusError(String),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error(transparent)]
    StorageError(#[from] StorageError),
}

#[derive(TypedBuilder)]
pub struct SyncLoaderHandler<F: StatementFormatter, S: Storage, H: DataHandler<FullData>> {
    db: Arc<Database<F, S>>,
    raw: H,
}

#[async_trait]
impl<F, S, H> DataHandler<FullData> for SyncLoaderHandler<F, S, H>
where
    F: StatementFormatter,
    S: Storage,
    H: DataHandler<FullData>,
{
    async fn query_loading_contracts(&self, chain_id: u64) -> handlerResult<Option<Vec<ContractConfig>>> {
        self.raw.query_loading_contracts(chain_id).await
    }

    async fn query_chain_loaded_block(&self, chain_id: u64) -> handlerResult<Option<u64>> {
        self.raw.query_chain_loaded_block(chain_id).await
    }

    async fn query_contract_loaded_block(&self, chain_id: u64, contract_address: &str) -> handlerResult<Option<u64>> {
        self.raw.query_contract_loaded_block(chain_id, contract_address).await
    }

    async fn query_commitment(&self, option: &CommitmentQueryOption) -> handlerResult<Option<Commitment>> {
        self.raw.query_commitment(option).await
    }

    async fn query_commitments(&self, option: &CommitmentQueryOption) -> handlerResult<QueryResult<Vec<Commitment>>> {
        self.raw.query_commitments(option).await
    }

    async fn count_commitments(&self, option: &CommitmentQueryOption) -> handlerResult<QueryResult<u64>> {
        self.raw.count_commitments(option).await
    }

    async fn query_nullifier(&self, option: &NullifierQueryOption) -> handlerResult<Option<Nullifier>> {
        self.raw.query_nullifier(option).await
    }

    async fn query_nullifiers(&self, option: &NullifierQueryOption) -> handlerResult<QueryResult<Vec<Nullifier>>> {
        self.raw.query_nullifiers(option).await
    }

    async fn count_nullifiers(&self, option: &NullifierQueryOption) -> handlerResult<QueryResult<u64>> {
        self.raw.count_nullifiers(option).await
    }

    async fn handle(&self, data: &ChainData<FullData>, option: &HandleOption) -> handlerResult<ChainResult<()>> {
        let result = self.update(data).await;
        if let Err(e) = result {
            log::error!("failed to update data: {:?}", e);
            return Err(HandlerError::from(anyhow::Error::from(e)));
        }
        self.raw.handle(data, option).await
    }

    async fn reset(&self, options: &ResetOptions) -> handlerResult<()> {
        self.raw.reset(options).await
    }
}

impl<F, S, H> SyncLoaderHandler<F, S, H>
where
    F: StatementFormatter,
    S: Storage,
    H: DataHandler<FullData>,
{
    async fn update(&self, data: &ChainData<FullData>) -> SyncLoaderHandlerResult<()> {
        let tasks = data
            .contracts_data
            .iter()
            .filter(|contract_data| contract_data.data.is_some())
            .map(|contract_data| self.update_by_contract(data.chain_id, contract_data))
            .collect::<Vec<_>>();
        let _ = futures::future::try_join_all(tasks).await?;
        Ok(())
    }

    async fn update_by_contract(
        &self,
        chain_id: u64,
        contract_data: &ContractData<FullData>,
    ) -> SyncLoaderHandlerResult<()> {
        if let Some(full) = &contract_data.data {
            self.update_by_commitments(chain_id, &contract_data.address, &full.commitments)
                .await?;
            self.update_by_nullifiers(chain_id, &contract_data.address, &full.nullifiers)
                .await?;
        }
        Ok(())
    }

    async fn update_by_commitments(
        &self,
        chain_id: u64,
        address: &str,
        cms: &[Commitment],
    ) -> SyncLoaderHandlerResult<()> {
        let dedup_cms = dedup_commitments(cms.to_vec());
        let tasks = [
            CommitmentStatus::SrcSucceeded,
            CommitmentStatus::Queued,
            CommitmentStatus::Included,
        ]
        .iter()
        .map(|cm_status| self.update_by_commitments_status(chain_id, address, &dedup_cms, *cm_status))
        .collect::<Vec<_>>();
        let _ = futures::future::try_join_all(tasks).await?;
        Ok(())
    }

    async fn update_by_nullifiers(
        &self,
        chain_id: u64,
        address: &str,
        nullifiers: &[Nullifier],
    ) -> SyncLoaderHandlerResult<()> {
        let nullifiers = nullifiers
            .iter()
            .map(|n| bytes_to_biguint(&n.nullifier))
            .collect::<HashSet<_>>();
        if !nullifiers.is_empty() {
            let condition = Condition::and(vec![
                SubFilter::equal(CommitmentColumn::ChainId, chain_id),
                SubFilter::equal(CommitmentColumn::ContractAddress, address.to_string()),
                SubFilter::equal(CommitmentColumn::Spent, false),
                SubFilter::is_not_null(CommitmentColumn::Nullifier),
            ]);
            let commitments = self.db.commitments.find(condition).await?;
            let to_update: Vec<_> = commitments
                .into_iter()
                .filter(|commitment| {
                    commitment
                        .data
                        .nullifier
                        .as_ref()
                        .map_or(false, |n| nullifiers.contains(n))
                })
                .map(|mut commitment| {
                    commitment.data.spent = true;
                    commitment
                })
                .collect();
            self.db.commitments.update_batch(to_update.as_slice()).await?;
        }

        Ok(())
    }

    async fn update_by_commitments_status(
        &self,
        chain_id: u64,
        address: &str,
        dedup_cms: &[Commitment],
        cm_status: CommitmentStatus,
    ) -> SyncLoaderHandlerResult<()> {
        let cms = dedup_cms
            .iter()
            .filter(|cm| cm.status == cm_status as i32)
            .map(|cm| (bytes_to_biguint(&cm.commitment_hash).to_string(), cm))
            .collect::<HashMap<_, _>>();
        if !cms.is_empty() {
            let condition = build_filter_by_commitment_status(chain_id, address, cm_status)?;
            let deposits = self.db.deposits.find(condition).await?;
            let to_update = deposits
                .into_iter()
                .filter_map(|deposit| {
                    cms.get(&deposit.data.commitment_hash)
                        .map(|c| update_deposit_by_commitment_status(deposit, c))
                })
                .collect::<Result<Vec<_>, _>>()?;

            if !to_update.is_empty() {
                self.db.deposits.update_batch(to_update.as_slice()).await?;
            }
        }
        Ok(())
    }
}

fn build_filter_by_commitment_status(
    chain_id: u64,
    address: &str,
    cm_status: CommitmentStatus,
) -> SyncLoaderHandlerResult<Condition> {
    let mut filter_status = vec![
        DepositStatus::Unspecified as i32,
        DepositStatus::AssetApproving as i32,
        DepositStatus::AssetApproved as i32,
        DepositStatus::Failed as i32,
    ];
    let condition = match cm_status {
        CommitmentStatus::SrcSucceeded => {
            filter_status.push(DepositStatus::SrcPending as i32);
            Condition::and(vec![
                SubFilter::equal(DepositColumn::ChainId, chain_id),
                SubFilter::equal(DepositColumn::ContractAddress, address.to_string()),
                SubFilter::in_list(DepositColumn::Status, filter_status),
            ])
        }
        CommitmentStatus::Queued => Condition::and(vec![
            SubFilter::equal(DepositColumn::DstChainId, chain_id),
            SubFilter::equal(DepositColumn::DstPoolAddress, address.to_string()),
            SubFilter::in_list(DepositColumn::Status, filter_status),
        ]),
        CommitmentStatus::Included => {
            filter_status.push(DepositStatus::Queued as i32);
            Condition::and(vec![
                SubFilter::equal(DepositColumn::DstChainId, chain_id),
                SubFilter::equal(DepositColumn::DstPoolAddress, address.to_string()),
                SubFilter::in_list(DepositColumn::Status, filter_status),
            ])
        }
        s => {
            return Err(SyncLoaderHandlerError::CommitmentStatusError(
                s.as_str_name().to_string(),
            ))
        }
    };

    Ok(condition)
}

fn update_deposit_by_commitment_status(
    mut deposit: Document<Deposit>,
    cm: &Commitment,
) -> SyncLoaderHandlerResult<Document<Deposit>> {
    match CommitmentStatus::from_i32(cm.status) {
        Some(CommitmentStatus::SrcSucceeded) => {
            if let Some(tx) = cm.src_chain_transaction_hash_as_hex() {
                deposit.data.src_chain_transaction_hash = Some(tx);
            }
            deposit.data.status = DepositStatus::SrcSucceeded as i32;
        }
        Some(CommitmentStatus::Queued) => {
            deposit.data.status = DepositStatus::Queued as i32;
        }
        Some(CommitmentStatus::Included) => {
            deposit.data.status = DepositStatus::Included as i32;
        }
        Some(s) => {
            return Err(SyncLoaderHandlerError::CommitmentStatusError(
                s.as_str_name().to_string(),
            ))
        }
        None => {
            return Err(SyncLoaderHandlerError::CommitmentStatusError(format!(
                "unknown commitment status {:?}",
                cm.status
            )))
        }
    }

    if let Some(tx) = cm.queued_transaction_hash_as_hex() {
        deposit.data.queued_transaction_hash = Some(tx);
    }

    if let Some(tx) = cm.included_transaction_hash_as_hex() {
        deposit.data.included_transaction_hash = Some(tx);
    }

    Ok(deposit)
}
