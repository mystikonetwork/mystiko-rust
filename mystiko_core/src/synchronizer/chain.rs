use mystiko_config::MystikoConfig;
use mystiko_dataloader::loader::{DataLoader, LoadOption};
use mystiko_protos::core::synchronizer::v1::{SynchronizerChainStatus, SynchronizerContractStatus};
use std::sync::Arc;
use tokio::sync::RwLock;
use typed_builder::TypedBuilder;

#[derive(Debug)]
pub struct ChainSynchronizer<L: DataLoader> {
    chain_id: u64,
    loader: L,
    config: Arc<MystikoConfig>,
    syncing: RwLock<bool>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ChainSynchronizerOptions<L: DataLoader> {
    chain_id: u64,
    loader: L,
    config: Arc<MystikoConfig>,
}

impl<L> ChainSynchronizer<L>
where
    L: DataLoader,
{
    pub fn new<O>(options: O) -> Self
    where
        O: Into<ChainSynchronizerOptions<L>>,
    {
        let options = options.into();
        Self {
            chain_id: options.chain_id,
            loader: options.loader,
            config: options.config,
            syncing: RwLock::new(false),
        }
    }

    pub async fn is_syncing(&self) -> bool {
        *self.syncing.read().await
    }

    pub async fn sync(&self, options: &LoadOption) -> crate::Result<()> {
        if !self.set_syncing(true).await {
            log::info!(
                "start syncing data for chain_id={} with options={:?}",
                self.chain_id,
                options
            );
            let start_time = std::time::Instant::now();
            self.loader.load(options.clone()).await?;
            let loaded_block = self.synced_block().await?;
            log::info!(
                "successfully synced chain_id={} data to {} in {}ms",
                self.chain_id,
                loaded_block,
                start_time.elapsed().as_millis()
            );
        } else {
            log::warn!("skip syncing data for chain_id={}", self.chain_id);
        }
        Ok(())
    }

    pub async fn synced_block(&self) -> crate::Result<u64> {
        Ok(self.loader.chain_loaded_block(self.chain_id).await?.unwrap_or_default())
    }

    pub async fn contract_synced_block(&self, address: &str) -> crate::Result<u64> {
        Ok(self
            .loader
            .contract_loaded_block(self.chain_id, address)
            .await?
            .unwrap_or_default())
    }

    pub async fn status(&self) -> crate::Result<SynchronizerChainStatus> {
        Ok(SynchronizerChainStatus::builder()
            .chain_id(self.chain_id)
            .syncing(self.is_syncing().await)
            .synced_block(self.synced_block().await?)
            .build())
    }

    pub async fn status_with_contracts(&self) -> crate::Result<SynchronizerChainStatus> {
        let mut status = self.status().await?;
        if let Some(chain_config) = self.config.find_chain(self.chain_id) {
            let contracts_futures = chain_config
                .contracts()
                .into_iter()
                .map(|contract_config| self.contract_synced_block_with_address(contract_config.address().to_string()))
                .collect::<Vec<_>>();
            status.contracts = futures::future::try_join_all(contracts_futures)
                .await?
                .into_iter()
                .map(|(contract_loaded_block, contract_address)| {
                    SynchronizerContractStatus::builder()
                        .address(contract_address)
                        .synced_block(contract_loaded_block)
                        .build()
                })
                .collect();
        }
        Ok(status)
    }

    async fn set_syncing(&self, syncing: bool) -> bool {
        let mut last_state = *self.syncing.read().await;
        if last_state != syncing {
            let mut writer = self.syncing.write().await;
            last_state = *writer;
            if last_state != syncing {
                *writer = syncing;
            }
        }
        last_state
    }

    async fn contract_synced_block_with_address(&self, address: String) -> crate::Result<(u64, String)> {
        Ok((self.contract_synced_block(&address).await?, address))
    }
}

impl<L> From<ChainSynchronizerOptions<L>> for ChainSynchronizer<L>
where
    L: DataLoader,
{
    fn from(options: ChainSynchronizerOptions<L>) -> Self {
        Self::new(options)
    }
}
