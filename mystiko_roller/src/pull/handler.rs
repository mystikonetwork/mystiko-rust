use crate::chain::ChainDataGiver;
use crate::common::error::Result;
use crate::config::roller::PullConfig;
use crate::context::ContextTrait;
use crate::data::handler::DataHandler;
use std::cmp::Ordering;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

pub struct PullHandle {
    pub chain_id: u64,
    pub contract_address: String,
    cfg: PullConfig,
    data: Arc<RwLock<DataHandler>>,
}

impl PullHandle {
    pub fn new(contract_address: &str, context: Arc<dyn ContextTrait + Send>, data: Arc<RwLock<DataHandler>>) -> Self {
        let cfg = context.cfg().pull.clone();
        PullHandle {
            chain_id: context.cfg().chain.chain_id,
            contract_address: contract_address.to_string(),
            cfg,
            data,
        }
    }

    async fn pull_queued_commitments<T: ChainDataGiver + ?Sized>(
        &self,
        giver: Arc<T>,
        start: u64,
        end: u64,
    ) -> Result<()> {
        let cms = giver
            .get_queued_commitments(self.chain_id, &self.contract_address, start, end)
            .await?;
        self.data.write().await.insert_commitments(cms.as_slice()).await?;
        Ok(())
    }

    async fn pull_from_chain_data_giver<T: ChainDataGiver + ?Sized>(&self, giver: Arc<T>) -> Result<()> {
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

    pub async fn pull<T: ChainDataGiver + ?Sized>(&self, giver: Arc<T>) -> Result<()> {
        self.pull_from_chain_data_giver(giver).await
    }
}
