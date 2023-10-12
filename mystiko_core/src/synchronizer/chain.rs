use mystiko_dataloader::loader::{DataLoader, LoadOption};
use tokio::sync::RwLock;
use typed_builder::TypedBuilder;

#[derive(Debug)]
pub struct ChainSynchronizer<L: DataLoader> {
    chain_id: u64,
    loader: L,
    loading: RwLock<bool>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ChainSynchronizerOptions<L: DataLoader> {
    chain_id: u64,
    loader: L,
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
            loading: RwLock::new(false),
        }
    }

    pub async fn is_loading(&self) -> bool {
        *self.loading.read().await
    }

    pub async fn load(&self, options: &LoadOption) -> crate::Result<()> {
        if !self.set_loading(true).await {
            log::info!(
                "start loading data for chain_id={} with options={:?}",
                self.chain_id,
                options
            );
            let start_time = std::time::Instant::now();
            self.loader.load(options.clone()).await?;
            let loaded_block = self.loaded_block().await?;
            log::info!(
                "successfully loaded chain_id={} data to {} in {}ms",
                self.chain_id,
                loaded_block,
                start_time.elapsed().as_millis()
            );
        } else {
            log::warn!("skip loading data for chain_id={}", self.chain_id);
        }
        Ok(())
    }

    pub async fn loaded_block(&self) -> crate::Result<u64> {
        Ok(self.loader.chain_loaded_block(self.chain_id).await?.unwrap_or_default())
    }

    pub async fn contract_loaded_block(&self, address: &str) -> crate::Result<u64> {
        Ok(self
            .loader
            .contract_loaded_block(self.chain_id, address)
            .await?
            .unwrap_or_default())
    }

    async fn set_loading(&self, loading: bool) -> bool {
        let mut last_state = *self.loading.read().await;
        if last_state != loading {
            let mut writer = self.loading.write().await;
            last_state = *writer;
            if last_state != loading {
                *writer = loading;
            }
        }
        last_state
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
