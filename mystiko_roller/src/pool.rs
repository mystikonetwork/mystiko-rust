use crate::context::ContextTrait;
use crate::data::handler::DataHandler;
use crate::pull::handler::PullHandle;
use crate::rollup::handler::RollupHandle;
use mystiko_config::wrapper::contract::pool::PoolContractConfig;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{self, Duration};
use tracing::{error, info};

pub struct Pool {
    context: Arc<dyn ContextTrait>,
    data: Arc<RwLock<DataHandler>>,
    pull: PullHandle,
    rollup: RollupHandle,
}

impl Pool {
    pub async fn new(pool_contract: &PoolContractConfig, context: Arc<dyn ContextTrait>) -> Pool {
        let data = DataHandler::new(context.cfg().chain.chain_id, pool_contract, Arc::clone(&context)).await;
        let data_rc = Arc::new(RwLock::new(data));
        let pull = PullHandle::new(pool_contract.address(), Arc::clone(&context), Arc::clone(&data_rc));
        let rollup = RollupHandle::new(pool_contract, Arc::clone(&context), Arc::clone(&data_rc)).await;

        Pool {
            context,
            data: data_rc,
            pull,
            rollup,
        }
    }

    pub async fn start(&mut self) {
        info!("start");
        self.data.write().await.load_commitment_from_db().await;
        self.run().await;
    }

    pub async fn run(&mut self) {
        let mut interval = time::interval(Duration::from_secs(self.context.cfg().pull.check_interval_secs));
        loop {
            interval.tick().await;
            self.run_once().await;
        }
    }

    pub async fn run_once(&mut self) {
        let result = self.pull.pull().await;
        if result.is_err() {
            error!("pull error: {:?}", result.err().unwrap());
        }

        let result = self.rollup.rollup().await;
        if result.is_err() {
            error!("rollup error: {:?}", result.err().unwrap());
        }
    }
}
