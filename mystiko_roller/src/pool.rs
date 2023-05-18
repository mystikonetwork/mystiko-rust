use crate::context::Context;
use crate::data::data::DataHandle;
use crate::pull::handle::PullHandle;
use crate::rollup::handle::RollupHandle;
use mystiko_config::wrapper::contract::pool::PoolContractConfig;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use tokio::time::{self, Duration};
use tracing::{error, info};

pub struct Pool {
    context: Arc<Context>,
    data: Rc<RefCell<DataHandle>>,
    pull: PullHandle,
    rollup: RollupHandle,
}

impl Pool {
    pub async fn new(pool_contract: &PoolContractConfig, context: Arc<Context>) -> Pool {
        let data = DataHandle::new(pool_contract, Arc::clone(&context)).await;
        let data_rc = Rc::new(RefCell::new(data));
        let pull = PullHandle::new(pool_contract.address(), Arc::clone(&context), Rc::clone(&data_rc));
        let rollup = RollupHandle::new(pool_contract, Arc::clone(&context), Rc::clone(&data_rc)).await;

        Pool {
            context,
            data: data_rc,
            pull,
            rollup,
        }
    }

    pub async fn start(&mut self) {
        info!("start");
        self.data.borrow_mut().load_commitment_from_db().await;
        self.run().await;
    }

    pub async fn run(&mut self) {
        let mut interval = time::interval(Duration::from_secs(self.context.cfg().pull.check_interval_secs));
        loop {
            interval.tick().await;
            self.do_rollup().await;
        }
    }

    pub async fn do_rollup(&mut self) {
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
