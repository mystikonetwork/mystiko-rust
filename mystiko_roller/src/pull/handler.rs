use crate::chain::provider::ProviderStub;
use crate::chain::ChainDataGiver;
use crate::common::error::{Result, RollerError};
use crate::config::roller::{ChainDataSource, PullConfig};
use crate::context::ContextTrait;
use crate::data::handler::DataHandler;
use std::cmp::Ordering;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

pub struct PullHandle {
    pub chain_id: u64,
    pub contract_address: String,
    cfg: PullConfig,
    context: Arc<dyn ContextTrait>,
    data: Arc<RwLock<DataHandler>>,
    stub_provider: Arc<ProviderStub>,
}

impl PullHandle {
    pub fn new(contract_address: &str, context: Arc<dyn ContextTrait + Send>, data: Arc<RwLock<DataHandler>>) -> Self {
        let cfg = context.cfg().pull.clone();
        let stub_provider = Arc::new(ProviderStub::new(contract_address, context.provider()));
        PullHandle {
            chain_id: context.cfg().chain.chain_id,
            contract_address: contract_address.to_string(),
            cfg,
            context,
            data,
            stub_provider,
        }
    }

    async fn pull_queued_commitments(&self, giver: Arc<dyn ChainDataGiver>, start: u64, end: u64) -> Result<()> {
        debug!("pull queued commitments");
        let cms = giver
            .get_queued_commitments(self.chain_id, &self.contract_address, start, end)
            .await?;
        self.data.write().await.insert_commitments(cms.as_slice()).await?;
        Ok(())
    }

    async fn pull_from_chain_data_giver(&self, giver: Arc<dyn ChainDataGiver>) -> Result<()> {
        debug!("pull from giver {:?}", giver.data_source());
        let current_block = self.data.write().await.get_next_sync_block();
        let latest_block = giver
            .get_latest_block_number(self.chain_id, &self.contract_address)
            .await?;

        match current_block.cmp(&(latest_block + 1)) {
            Ordering::Less => {
                let batch = self.cfg.batch_block(giver.data_source());
                for start in (current_block..=latest_block).step_by(batch) {
                    let end = std::cmp::min(start + batch as u64 - 1, latest_block);
                    self.pull_queued_commitments(giver.clone(), start, end).await?;
                }
                self.data.write().await.set_next_sync_block(latest_block + 1);
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                info!(
                    "start block {:?} is greater/equal than {:?} latest block {:?}",
                    current_block,
                    giver.data_source(),
                    latest_block
                );
            }
        }
        Ok(())
    }

    pub async fn pull(&self, source: &ChainDataSource) -> Result<()> {
        debug!("pull");
        match *source {
            ChainDataSource::Provider => self.pull_from_chain_data_giver(self.stub_provider.clone()).await,
            ChainDataSource::Indexer => match self.context.indexer() {
                Some(indexer) => self.pull_from_chain_data_giver(indexer).await,
                None => Err(RollerError::NoIndexer),
            },
            ChainDataSource::Explorer => panic!("un support"),
        }
    }
}
