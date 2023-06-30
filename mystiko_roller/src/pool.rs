use crate::common::error::{Result, RollerError};
use crate::config::roller::ChainDataSource;
use crate::context::ContextTrait;
use crate::data::handler::DataHandler;
use crate::pull::handler::PullHandle;
use crate::rollup::handler::RollupHandle;
use mystiko_config::wrapper::contract::pool::PoolContractConfig;
use mystiko_server_utils::tx_manager::error::TxManagerError::GasPriceError;

use std::cmp::Ordering;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};
use tracing::{debug, error};

pub enum PoolAction {
    Init,
    Run,
}

pub struct Pool {
    pub index: String,
    context: Arc<dyn ContextTrait>,
    data: Arc<RwLock<DataHandler>>,
    pull: PullHandle,
    rollup: RollupHandle,
    empty_queue_counter: u64,
}

impl Pool {
    pub async fn new(index: usize, pool_contract: &PoolContractConfig, context: Arc<dyn ContextTrait + Send>) -> Pool {
        let data = DataHandler::new(context.cfg().chain.chain_id, pool_contract, Arc::clone(&context)).await;
        let data_rc = Arc::new(RwLock::new(data));
        let pull = PullHandle::new(pool_contract.address(), Arc::clone(&context), Arc::clone(&data_rc));
        let rollup = RollupHandle::new(pool_contract, Arc::clone(&context), Arc::clone(&data_rc)).await;

        Pool {
            index: index.to_string(),
            context,
            data: data_rc,
            pull,
            rollup,
            empty_queue_counter: 0,
        }
    }

    pub async fn init(self) -> Result<Self> {
        debug!("start");
        self.data.write().await.load_commitment_from_db().await?;
        Ok(self)
    }

    pub async fn run(mut self) -> Result<Self> {
        debug!("run");

        let chain_data_sources = self.context.cfg().get_data_sources();
        let check_time = Duration::from_secs(self.context.cfg().pull.check_interval_secs);
        for source in &chain_data_sources {
            sleep(check_time).await;
            match self.run_from_one_source(source).await {
                Err(e) => {
                    if self.should_failover(&e) {
                        error!("{:?} meet error {:?}, failover", source, e);
                        continue;
                    } else {
                        break;
                    }
                }
                Ok(_) => {
                    debug!("{:?} success", source);
                    break;
                }
            }
        }
        Ok(self)
    }

    pub async fn run_from_one_source(&mut self, source: &ChainDataSource) -> Result<()> {
        debug!("roller run new round");

        self.pull.pull(source).await?;
        self.check_commitment_queue().await?;
        self.rollup.rollup(source).await?;
        Ok(())
    }

    pub async fn check_commitment_queue(&mut self) -> Result<()> {
        debug!("check commitment queue");
        let queue_len = self.data.read().await.get_commitments_queue_count();
        let include_count = self.data.read().await.get_included_count();

        match queue_len.cmp(&include_count) {
            Ordering::Greater => {
                self.empty_queue_counter = 0;
                Ok(())
            }
            Ordering::Equal => {
                self.empty_queue_counter += 1;
                if self.empty_queue_counter > self.context.cfg().pull.max_empty_queue_count {
                    if let Ok(false) = self.rollup.is_chain_commitment_queue_empty().await {
                        self.data.write().await.reset_next_sync_block();
                        self.empty_queue_counter = 0;
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
