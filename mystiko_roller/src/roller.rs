use crate::context::Context;
use crate::pool::Pool;
use anyhow::Result;
use std::sync::Arc;
use std::thread;
use tracing::info;

pub struct Roller {
    context: Arc<Context>,
}

impl Roller {
    pub async fn new() -> Result<Roller> {
        let context = Context::new().await;
        Ok(Roller {
            context: Arc::new(context),
        })
    }

    pub async fn start(&self) {
        let mut thread_handles = vec![];
        let context = Arc::clone(&self.context);
        let mut thread_number = 0;

        let pool_contracts = self
            .context
            .core_cfg_parser()
            .pool_contracts(self.context.cfg().chain.chain_id);
        for pool_contract in pool_contracts {
            if pool_contract.address() != "0x83Ad3a5B2dE65b32a446e9B73640a8B8431D3eb7" {
                continue;
            }

            let thread_name = "T".to_string() + &thread_number.to_string();
            info!(
                "new thread {:?} for contract {:?}",
                thread_name,
                pool_contract.address()
            );

            let context = Arc::clone(&context); // Clone the context before moving it into the closure
            let new_thread = thread::Builder::new()
                .name(thread_name)
                .spawn(move || {
                    let runtime = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .unwrap();

                    runtime.block_on(async move {
                        let mut pool = Pool::new(&pool_contract, context).await;
                        pool.start().await;
                    });
                })
                .unwrap();
            thread_handles.push(new_thread);
            thread_number += 1;
        }

        for thread in thread_handles {
            thread.join().unwrap();
        }
    }
}
