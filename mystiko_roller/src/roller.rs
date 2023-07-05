use crate::common::env::{load_roller_config_path, load_roller_run_mod};
use crate::common::error::Result;
use crate::config::roller::create_tx_manager_config;
use crate::context::Context;
use crate::context::ContextTrait;
use crate::pool::{Pool, PoolAction};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{error, info, Instrument};

pub struct Roller {
    pub round: u64,
    pub stop: bool,
    round_check_sec: u64,
    pools: Vec<Pool>,
}

impl Roller {
    pub async fn new(run_mod: &str, cfg_path: &str) -> Result<Roller> {
        let context = Arc::new(Context::new(run_mod, cfg_path).await?);
        info!("starting roller {:?}", context.cfg().chain.chain_id);
        let pool_contracts = context.core_cfg_parser().pool_contracts(context.cfg().chain.chain_id);
        let mut pools = Vec::new();
        let tx_manager_cfg = create_tx_manager_config(run_mod, cfg_path);
        for (index, pool_contract) in pool_contracts.into_iter().enumerate() {
            info!("create pool instance t{:?} {:?}", index, pool_contract.address());
            let pool = Pool::new(
                index,
                pool_contract,
                &tx_manager_cfg,
                Arc::clone(&context) as Arc<dyn ContextTrait + Send>,
            )
            .await;
            pools.push(pool);
        }

        Ok(Roller {
            round_check_sec: context.cfg().pull.check_interval_secs,
            round: 0,
            stop: false,
            pools,
        })
    }

    pub async fn start(&mut self) {
        if self.pools.is_empty() {
            return;
        }

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
            self.round += 1;
            if self.stop {
                info!("roller stopped");
                break;
            }
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

pub async fn run_roller() {
    let run_mod = load_roller_run_mod();
    let cfg_path = load_roller_config_path();
    let roller = Roller::new(&run_mod, &cfg_path).await;
    match roller {
        Ok(mut r) => {
            r.start().await;
        }
        Err(e) => {
            error!("roller start failed {:?}", e);
        }
    }
}
