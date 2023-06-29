use crate::common::error::Result;
use crate::context::Context;
use crate::context::ContextTrait;
use crate::pool::{Pool, PoolAction};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{error, info, Instrument};

pub struct Roller {
    round_check_sec: u64,
    pools: Vec<Pool>,
}

impl Roller {
    pub async fn new() -> Result<Roller> {
        let context = Arc::new(Context::new().await?);

        info!("starting roller {:?}", context.cfg().chain.chain_id);
        let pool_contracts = context.core_cfg_parser().pool_contracts(context.cfg().chain.chain_id);
        let mut pools = Vec::new();
        for (index, pool_contract) in pool_contracts.into_iter().enumerate() {
            info!("create pool instance t{:?} {:?}", index, pool_contract.address());
            let pool = Pool::new(
                index,
                &pool_contract,
                Arc::clone(&context) as Arc<dyn ContextTrait + Send>,
            )
            .await;
            pools.push(pool);
        }

        Ok(Roller {
            round_check_sec: context.cfg().pull.check_interval_secs,
            pools,
        })
    }

    pub async fn start(&mut self) {
        if let Err(e) = self.run(PoolAction::Init).await {
            error!("Failed to run init: {:?}", e);
            return;
        }

        loop {
            if let Err(e) = self.run(PoolAction::Run).await {
                error!("Failed to run: {:?}", e);
                break;
            }

            sleep(Duration::from_secs(self.round_check_sec)).await;
        }
    }

    async fn run(&mut self, action: PoolAction) -> Result<()> {
        let tasks: Vec<_> = self
            .pools
            .drain(..)
            .map(|pool| {
                let index = pool.index.clone();
                match action {
                    PoolAction::Init => tokio::spawn(pool.init().instrument(tracing::info_span!("", T = index))),
                    PoolAction::Run => tokio::spawn(pool.run().instrument(tracing::info_span!("", T = index))),
                }
            })
            .collect();
        let results = futures::future::try_join_all(tasks).await?;

        for result in results {
            match result {
                Ok(pool) => self.pools.push(pool),
                Err(e) => {
                    error!("Failed to run pool: {:?}", e);
                    return Err(e);
                }
            }
        }

        Ok(())
    }
}
