use crate::common::error::{Result, RollerError};
use crate::config::roller::PullConfig;
use crate::context::ContextTrait;
use crate::data::handler::DataHandler;
use crate::sync::indexer::SyncIndexer;
use crate::sync::provider::SyncProvider;
use crate::sync::SyncTrait;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};
use tracing::{debug, error};

pub struct PullHandle {
    pub chain_id: u64,
    pub contract_address: String,
    cfg: PullConfig,
    context: Arc<dyn ContextTrait>,
    data: Arc<RwLock<DataHandler>>,
}

impl PullHandle {
    pub fn new(contract_address: &str, context: Arc<dyn ContextTrait>, data: Arc<RwLock<DataHandler>>) -> Self {
        let cfg = context.cfg().pull.clone();
        PullHandle {
            chain_id: context.cfg().chain.chain_id,
            contract_address: contract_address.to_string(),
            cfg,
            context,
            data,
        }
    }

    async fn pull_from_provider(&self, sync: &SyncProvider) -> Result<()> {
        debug!("pull from {:?}", sync.sync_type());

        let start_block = self.data.write().await.get_next_sync_block();
        let latest_block = sync
            .get_latest_block_number(self.chain_id, &self.contract_address)
            .await?;
        if start_block > latest_block {
            error!(
                "{:?} start block {:?} is greater than latest block {:?}",
                sync.sync_type(),
                start_block,
                latest_block
            );
            return Err(RollerError::InvalidStartBlock);
        }

        let batch = self.cfg.batch_block(sync.sync_type());
        for start in (start_block..=latest_block).step_by(batch) {
            let end = std::cmp::min(start + batch as u64 - 1, latest_block);
            let cms = sync
                .get_queued_commitments(self.chain_id, &self.contract_address, start, end)
                .await?;
            self.data.write().await.insert_commitments(cms.as_slice()).await;
        }
        self.data.write().await.set_new_next_sync_block(latest_block + 1);
        Ok(())
    }

    async fn pull_from_indexer(&self, sync: RwLockReadGuard<'_, SyncIndexer>) -> Result<()> {
        debug!("pull from {:?}", sync.sync_type());

        let start_block = self.data.write().await.get_next_sync_block();
        let latest_block = sync
            .get_latest_block_number(self.chain_id, &self.contract_address)
            .await?;
        if start_block > latest_block {
            error!(
                "{:?} start block {:?} is greater than latest block {:?}",
                sync.sync_type(),
                start_block,
                latest_block
            );
            return Err(RollerError::InvalidStartBlock);
        }

        let batch = self.cfg.batch_block(sync.sync_type());
        for start in (start_block..=latest_block).step_by(batch) {
            let end = std::cmp::min(start + batch as u64 - 1, latest_block);
            let cms = sync
                .get_queued_commitments(self.chain_id, &self.contract_address, start, end)
                .await?;
            self.data.write().await.insert_commitments(cms.as_slice()).await;
        }
        self.data.write().await.set_new_next_sync_block(latest_block + 1);
        Ok(())
    }

    pub async fn pull(&self) -> Result<()> {
        debug!("pull");
        if let Some(indexer) = self.context.indexer().await {
            match self.pull_from_indexer(indexer).await {
                Ok(_) => return Ok(()),
                Err(e) => error!("pull from indexer failed: {:?}", e),
            }
        }

        let provider = self.context.provider().await?;
        let sync_provider = SyncProvider::new(&self.contract_address, provider);
        self.pull_from_provider(&sync_provider).await
    }
}
