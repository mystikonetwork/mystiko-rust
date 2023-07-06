use crate::common::error::{Result, RollerError};
use crate::config::roller::ChainDataSource;
use crate::context::ContextTrait;
use crate::data::handler::DataHandler;
use crate::pull::handler::PullHandle;
use crate::rollup::handler::RollupHandle;
use mystiko_config::wrapper::contract::pool::PoolContractConfig;
use mystiko_server_utils::tx_manager::error::TxManagerError::GasPriceError;

use crate::chain::provider::ProviderStub;
use crate::chain::ChainDataGiver;
use mystiko_server_utils::tx_manager::config::TxManagerConfig;
use std::cmp::Ordering;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};
use tracing::error;

#[derive(Debug)]
pub enum PoolAction {
    Init,
    Run,
}

pub struct Pool {
    pub index: String,
    pub context: Arc<dyn ContextTrait>,
    pub data: Arc<RwLock<DataHandler>>,
    pull: PullHandle,
    rollup: RollupHandle,
    stub_provider: Arc<ProviderStub>,
}

impl Pool {
    pub async fn new(
        index: usize,
        pool_contract: PoolContractConfig,
        tx_manager_cfg: &TxManagerConfig,
        context: Arc<dyn ContextTrait + Send>,
    ) -> Pool {
        let data = DataHandler::new(context.cfg().chain.chain_id, &pool_contract, Arc::clone(&context)).await;
        let data_rc = Arc::new(RwLock::new(data));
        let pull = PullHandle::new(pool_contract.address(), Arc::clone(&context), Arc::clone(&data_rc));
        let rollup = RollupHandle::new(
            &pool_contract,
            tx_manager_cfg,
            Arc::clone(&context),
            Arc::clone(&data_rc),
        )
        .await;
        let stub_provider = Arc::new(ProviderStub::new(pool_contract.address(), context.provider()));
        Pool {
            index: index.to_string(),
            context,
            stub_provider,
            data: data_rc,
            pull,
            rollup,
        }
    }

    pub async fn init(self) -> Result<Pool> {
        self.data.write().await.load_commitment_from_db().await?;
        Ok(self)
    }

    pub async fn run(self) -> Result<Pool> {
        let chain_data_sources = self.context.cfg().chain.get_data_source_order();
        let check_time = Duration::from_secs(self.context.cfg().pull.check_interval_secs);
        println!("chain data sources: {:?}", chain_data_sources);
        for source in &chain_data_sources {
            let giver: Arc<dyn ChainDataGiver> = match source {
                ChainDataSource::Indexer => self.context.indexer().unwrap(),
                ChainDataSource::Explorer => self.context.chain_explorer().unwrap(),
                ChainDataSource::Provider => self.stub_provider.clone(),
            };

            if let Err(e) = self.run_from_one_source(giver).await {
                if self.should_failover(&e) {
                    error!("{:?} meet error {:?}, retry next data source", source, e);
                    sleep(check_time).await;
                    continue;
                } else {
                    error!("{:?} meet error {:?}, retry next time", source, e);
                    break;
                }
            } else {
                break;
            }
        }
        Ok(self)
    }

    pub async fn run_from_one_source<T: ChainDataGiver + ?Sized>(&self, giver: Arc<T>) -> Result<()> {
        self.pull.pull(giver.clone()).await?;
        self.check_commitment_queue().await?;
        self.rollup.rollup(giver).await
    }

    pub async fn check_commitment_queue(&self) -> Result<()> {
        let queue_len = self.data.read().await.get_commitments_queue_count();
        let include_count = self.data.read().await.get_included_count();
        match queue_len.cmp(&include_count) {
            Ordering::Greater => {
                self.data.write().await.set_empty_queue_counter(0);
                Ok(())
            }
            Ordering::Equal => {
                self.data.write().await.inc_empty_queue_counter();
                let empty_queue_counter = self.data.read().await.get_empty_queue_counter();
                if empty_queue_counter > self.context.cfg().pull.max_empty_queue_count {
                    if let Ok(false) = self.rollup.commitment_queue_check_by_transaction().await {
                        self.data.write().await.reset_next_sync_block();
                        self.data.write().await.set_empty_queue_counter(0);
                        return Err(RollerError::CommitmentMissing);
                    }
                }
                Ok(())
            }
            Ordering::Less => {
                panic!("unexpected check commitment queue");
            }
        }
    }

    fn should_failover(&self, err: &RollerError) -> bool {
        !matches!(
            err,
            RollerError::TxManagerError(GasPriceError(_))
                | RollerError::TokenPriceError(_)
                | RollerError::CircuitNotFound
        )
    }
}
