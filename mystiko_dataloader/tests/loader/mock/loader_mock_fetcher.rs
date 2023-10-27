use async_trait::async_trait;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::data::LoadedData;
use mystiko_dataloader::data::{ChainResult, ContractResult};
use mystiko_dataloader::fetcher::{ChainLoadedBlockOptions, DataFetcher, FetchOptions, FetchResult, FetcherError};
use std::time::Duration;
use tokio::sync::RwLock;

pub struct MockFetcher<R>
where
    R: LoadedData,
{
    pub chain_id: u64,
    loaded_block: RwLock<Option<u64>>,
    fetch_result: RwLock<Option<ChainData<R>>>,
    fetch_sleep: RwLock<bool>,
}

impl<R> MockFetcher<R>
where
    R: LoadedData + Clone,
{
    pub fn new(chain_id: u64) -> Self {
        MockFetcher {
            chain_id,
            loaded_block: RwLock::new(Some(u64::MAX)),
            fetch_result: RwLock::new(None),
            fetch_sleep: RwLock::new(false),
        }
    }

    pub async fn set_loaded_block(&self, block: Option<u64>) {
        *self.loaded_block.write().await = block;
    }

    pub async fn set_fetch_result(&self, r: ChainData<R>) {
        *self.fetch_result.write().await = Some(r);
    }
    pub async fn set_fetch_error_result(&self) {
        *self.fetch_result.write().await = None;
    }
    pub async fn set_fetch_sleep(&self, fetch_sleep: bool) {
        *self.fetch_sleep.write().await = fetch_sleep;
    }
}

#[async_trait]
impl<R> DataFetcher<R> for MockFetcher<R>
where
    R: LoadedData + Clone,
{
    fn name(&self) -> &'static str {
        "mock_fetcher"
    }

    async fn fetch(&self, _option: &FetchOptions) -> FetchResult<R> {
        if *self.fetch_sleep.read().await {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        if let Some(ref result) = *self.fetch_result.read().await {
            let contract_results = result
                .contracts_data
                .clone()
                .into_iter()
                .map(|d| {
                    ContractResult::builder()
                        .address(d.address.clone())
                        .result(Ok(ContractData::builder()
                            .address(d.address)
                            .start_block(d.start_block)
                            .end_block(d.end_block)
                            .build()))
                        .build()
                })
                .collect::<Vec<_>>();

            Ok(ChainResult::builder()
                .chain_id(self.chain_id)
                .contract_results(contract_results)
                .build())
        } else {
            Err(anyhow::Error::msg("error".to_string()).into())
        }
    }

    async fn chain_loaded_block(&self, _options: &ChainLoadedBlockOptions) -> Result<u64, FetcherError> {
        if let Some(block) = *self.loaded_block.read().await {
            if block == 0 {
                tokio::time::sleep(Duration::from_millis(100)).await;
                Ok(u64::MAX)
            } else {
                Ok(block)
            }
        } else {
            Err(anyhow::Error::msg("error".to_string()).into())
        }
    }
}
