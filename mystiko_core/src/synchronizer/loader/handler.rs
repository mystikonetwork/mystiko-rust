use crate::{CommitmentColumn, Database, Deposit, DepositColumn};
use async_trait::async_trait;
use mystiko_config::ContractConfig;
use mystiko_dataloader::data::{ChainData, ChainResult, ContractData, DataRef, FullData, LiteData, LoadedData};
use mystiko_dataloader::handler::{
    dedup_commitments, CommitmentQueryOption, DataHandler, HandleOption, NullifierQueryOption, QueryResult,
};
use mystiko_dataloader::handler::{HandlerError, Result as handlerResult};
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_protos::storage::v1::{Condition, SubFilter};
use mystiko_storage::{Document, StatementFormatter, Storage, StorageError};
use mystiko_types::DepositStatus;
use mystiko_utils::convert::bytes_to_biguint;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use thiserror::Error;
use typed_builder::TypedBuilder;

type SyncLoaderHandlerResult<T> = Result<T, SyncLoaderHandlerError>;

#[derive(Debug, Error)]
pub enum SyncLoaderHandlerError {
    #[error("invalid commitment status: {0}")]
    CommitmentStatusError(i32),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error(transparent)]
    StorageError(#[from] StorageError),
}

#[derive(TypedBuilder)]
pub struct SyncLoaderHandler<F: StatementFormatter, S: Storage, R: LoadedData> {
    mystiko_db: Arc<Database<F, S>>,
    raw: Box<dyn DataHandler<R>>,
}

#[async_trait]
impl<F, S, R> DataHandler<R> for SyncLoaderHandler<F, S, R>
where
    F: StatementFormatter,
    S: Storage,
    R: LoadedData,
{
    async fn query_loading_contracts(&self, _chain_id: u64) -> handlerResult<Option<Vec<ContractConfig>>> {
        self.raw.query_loading_contracts(_chain_id).await
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

    async fn handle(&self, data: &ChainData<R>, option: &HandleOption) -> handlerResult<ChainResult<()>> {
        let result = self.update(data).await;
        if let Err(e) = result {
            log::error!("failed to update data: {:?}", e);
            return Err(HandlerError::from(anyhow::Error::from(e)));
        }

        self.raw.handle(data, option).await
    }
}

impl<F, S, R> SyncLoaderHandler<F, S, R>
where
    F: StatementFormatter,
    S: Storage,
    R: LoadedData,
{
    async fn update(&self, data: &ChainData<R>) -> SyncLoaderHandlerResult<()> {
        let tasks = data
            .contracts_data
            .iter()
            .filter(|contract_data| contract_data.data.is_some())
            .map(|contract_data| self.update_by_contract(data.chain_id, contract_data))
            .collect::<Vec<_>>();
        let _ = futures::future::try_join_all(tasks).await?;
        Ok(())
    }

    async fn update_by_contract(&self, chain_id: u64, contract_data: &ContractData<R>) -> SyncLoaderHandlerResult<()> {
        if let Some(ref d) = contract_data.data {
            match R::data_ref(d) {
                DataRef::Full(full) => self.update_by_full_data(chain_id, &contract_data.address, full).await?,
                DataRef::Lite(lite) => self.update_by_lite_data(chain_id, &contract_data.address, lite).await?,
            }
        }
        Ok(())
    }

    async fn update_by_full_data(&self, chain_id: u64, address: &str, full: &FullData) -> SyncLoaderHandlerResult<()> {
        self.update_by_commitments(chain_id, address, &full.commitments).await?;
        self.update_by_nullifiers(chain_id, address, &full.nullifiers).await
    }

    async fn update_by_lite_data(&self, chain_id: u64, address: &str, lite: &LiteData) -> SyncLoaderHandlerResult<()> {
        self.update_by_commitments(chain_id, address, &lite.commitments).await
    }

    async fn update_by_commitments(
        &self,
        chain_id: u64,
        address: &str,
        cms: &[Commitment],
    ) -> SyncLoaderHandlerResult<()> {
        let dedup_cms = dedup_commitments(cms.to_vec());

        for cm_status in [
            CommitmentStatus::SrcSucceeded,
            CommitmentStatus::Queued,
            CommitmentStatus::Included,
        ] {
            self.update_by_commitments_status(chain_id, address, &dedup_cms, cm_status)
                .await?;
        }
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
            let commitments = self.mystiko_db.commitments.find(condition).await?;
            let mut to_update = Vec::new();
            for mut commitment in &mut commitments.into_iter() {
                if let Some(n) = &commitment.data.nullifier {
                    if nullifiers.contains(n) {
                        commitment.data.spent = true;
                        to_update.push(commitment);
                    }
                }
            }
            self.mystiko_db.commitments.update_batch(to_update.as_slice()).await?;
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
            .map(|cm| (bytes_to_biguint(&cm.commitment_hash), cm))
            .collect::<HashMap<_, _>>();
        if !cms.is_empty() {
            let condition = self.build_filter_by_commitment_status(chain_id, address, cm_status)?;
            let deposits = self.mystiko_db.deposits.find(condition).await?;
            let mut to_update = Vec::new();
            for mut deposit in &mut deposits.into_iter() {
                let cm = cms.get(&deposit.data.commitment_hash);
                if let Some(c) = cm {
                    self.update_deposit_by_commitment_status(&mut deposit, c)?;
                    to_update.push(deposit);
                }
            }
            self.mystiko_db.deposits.update_batch(to_update.as_slice()).await?;
        }
        Ok(())
    }

    fn build_filter_by_commitment_status(
        &self,
        chain_id: u64,
        address: &str,
        cm_status: CommitmentStatus,
    ) -> SyncLoaderHandlerResult<Condition> {
        let mut filter_status = vec![
            DepositStatus::Init as i32,
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
            s => return Err(SyncLoaderHandlerError::CommitmentStatusError(s as i32)),
        };

        Ok(condition)
    }

    fn update_deposit_by_commitment_status(
        &self,
        deposit: &mut Document<Deposit>,
        cm: &Commitment,
    ) -> SyncLoaderHandlerResult<()> {
        match cm.status {
            status if status == i32::from(CommitmentStatus::SrcSucceeded) => {
                if let Some(tx) = cm.src_chain_transaction_hash_as_hex() {
                    deposit.data.transaction_hash = Some(tx);
                }
                deposit.data.status = DepositStatus::SrcSucceeded;
            }
            status if status == i32::from(CommitmentStatus::Queued) => {
                deposit.data.status = DepositStatus::Queued;
            }
            status if status == i32::from(CommitmentStatus::Included) => {
                deposit.data.status = DepositStatus::Included;
            }
            s => return Err(SyncLoaderHandlerError::CommitmentStatusError(s)),
        }

        if let Some(tx) = cm.queued_transaction_hash_as_hex() {
            deposit.data.queued_transaction_hash = Some(tx);
        }

        if let Some(tx) = cm.included_transaction_hash_as_hex() {
            deposit.data.included_transaction_hash = Some(tx);
        }

        Ok(())
    }
}
