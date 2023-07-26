use async_trait::async_trait;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::chain::ChainData;
use mystiko_dataloader::data::contract::ContractData;
use mystiko_dataloader::data::types::{FullData, LoadedData};
use mystiko_dataloader::fetcher::types::{ChainFetchOption, ContractFetchOption, DataFetcher, FetchResult};
use mystiko_dataloader::handler::types::{DataHandler, HandleOption};
use mystiko_dataloader::loader::chain::{ChainDataLoader, ChainDataLoaderBuilder, StartOption};
use mystiko_dataloader::validator::types::{DataValidator, ValidateOption};
use std::sync::Arc;
use std::thread::sleep;
use tokio::sync::RwLock;

pub struct MockFetcher<R>
where
    R: LoadedData,
{
    pub result: RwLock<FetchResult<R>>,
}

impl<R> MockFetcher<R>
where
    R: LoadedData + Clone,
{
    pub fn new(result: FetchResult<R>) -> Self {
        MockFetcher {
            result: RwLock::new(result),
        }
    }

    pub async fn set_result(&self, result: FetchResult<R>) {
        *self.result.write().await = result;
    }
}

#[async_trait]
impl<R> DataFetcher<R> for MockFetcher<R>
where
    R: LoadedData + Clone,
{
    async fn fetch_chain(&self, _option: &ChainFetchOption) -> FetchResult<R> {
        let data = self.result.read().await;
        match *data {
            Ok(ref result) => {
                let data = result
                    .iter()
                    .map(|r| match r {
                        Ok(ref contract_data) => Ok(contract_data.clone()),
                        Err(_) => Err(anyhow::Error::msg("error".to_string())),
                    })
                    .collect();
                Ok(data)
            }
            Err(_) => Err(anyhow::Error::msg("error".to_string())),
        }
    }

    async fn fetch_contracts(&self, _options: &[ContractFetchOption]) -> FetchResult<R> {
        let data = self.result.read().await;
        match *data {
            Ok(ref result) => {
                let data = result
                    .iter()
                    .map(|r| match r {
                        Ok(ref contract_data) => Ok(contract_data.clone()),
                        Err(_) => Err(anyhow::Error::msg("error".to_string())),
                    })
                    .collect();
                Ok(data)
            }
            Err(_) => Err(anyhow::Error::msg("error".to_string())),
        }
    }
}

pub struct MockValidator {
    pub result: RwLock<Option<bool>>,
}

impl MockValidator {
    pub fn new(result: Option<bool>) -> Self {
        MockValidator {
            result: RwLock::new(result),
        }
    }

    pub async fn set_result(&self, result: Option<bool>) {
        *self.result.write().await = result;
    }
}

#[async_trait]
impl<R> DataValidator<R> for MockValidator
where
    R: LoadedData,
{
    async fn validate(&self, _data: &ChainData<R>, _option: &ValidateOption) -> anyhow::Result<bool> {
        match self.result.read().await.clone() {
            Some(result) => Ok(result),
            None => Err(anyhow::Error::msg("error".to_string())),
        }
    }
}

pub struct MockHandler {
    pub result: RwLock<bool>,
}

impl MockHandler {
    pub fn new(result: bool) -> Self {
        MockHandler {
            result: RwLock::new(result),
        }
    }

    pub async fn set_result(&self, result: bool) {
        *self.result.write().await = result;
    }
}

#[async_trait]
impl<R> DataHandler<R> for MockHandler
where
    R: LoadedData,
{
    async fn handle(&self, _data: &ChainData<R>, _option: &HandleOption) -> anyhow::Result<()> {
        if self.result.read().await.clone() {
            Ok(())
        } else {
            Err(anyhow::Error::msg("error".to_string()))
        }
    }
}

async fn loader_run(
    loader: Arc<ChainDataLoader<FullData, MockFetcher<FullData>, MockValidator, MockHandler>>,
    option: StartOption,
) {
    let loader_clone1 = Arc::clone(&loader);
    let loader_clone2 = Arc::clone(&loader);
    let handle1 = tokio::spawn(async move { loader_clone1.start(&option).await });
    let handle2 = tokio::spawn(async move {
        sleep(std::time::Duration::from_millis(20_u64));
        loader_clone2.stop().await
    });

    let result = futures::try_join!(handle1);
    assert!(result.is_ok());
    let result = futures::try_join!(handle2);
    assert!(result.is_ok());
}

async fn create_loader(
    fetch_result: bool,
    validator_result: Option<bool>,
    handler_result: bool,
    end_block: u64,
) -> ChainDataLoader<FullData, MockFetcher<FullData>, MockValidator, MockHandler> {
    let builder: ChainDataLoaderBuilder<FullData, MockFetcher<FullData>, MockValidator, MockHandler> =
        ChainDataLoaderBuilder::new();

    let fetcher_result = match fetch_result {
        true => FetchResult::Ok(vec![Ok(ContractData {
            address: "".to_string(),
            start_block: 1_u64,
            end_block,
            data: None,
        })]),
        false => FetchResult::Err(anyhow::Error::msg("error".to_string())),
    };

    let fetcher = MockFetcher::new(fetcher_result);
    let validator = MockValidator::new(validator_result);
    let handler = MockHandler::new(handler_result);
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();

    builder
        .chain_id(1)
        .initial_block(123_u64)
        .config(Arc::new(core_cfg))
        .add_fetcher(fetcher)
        .add_validator(validator)
        .add_handler(handler)
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_loader_start() {
    let end_block = 10_u64;
    let option = StartOption {
        load_interval_ms: Some(10_u64),
        max_batch_block: Some(1000000_u64),
        contract_filter: None,
    };
    let loader = Arc::new(create_loader(true, Some(true), true, end_block).await);
    loader_run(Arc::clone(&loader), option).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 10);
    assert_eq!(state.is_loading, false);
    assert_eq!(state.is_running, false);
}

#[tokio::test]
async fn test_loader_start_shared_fetcher() {
    let end_block = 10_u64;
    let builder: ChainDataLoaderBuilder<FullData, MockFetcher<FullData>, MockValidator, MockHandler> =
        ChainDataLoaderBuilder::new();

    let fetcher_result = FetchResult::Err(anyhow::Error::msg("error".to_string()));
    let fetcher = Arc::new(MockFetcher::new(fetcher_result));
    let validator = Arc::new(MockValidator::new(Some(true)));
    let handler = Arc::new(MockHandler::new(true));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();

    let loader = builder
        .chain_id(1)
        .initial_block(123_u64)
        .config(Arc::new(core_cfg))
        .add_shared_fetcher(Arc::clone(&fetcher))
        .add_shared_validator(Arc::clone(&validator))
        .add_shared_handler(Arc::clone(&handler))
        .build()
        .unwrap();
    let loader = Arc::new(loader);
    let option = StartOption {
        load_interval_ms: Some(10_u64),
        max_batch_block: Some(1000000_u64),
        contract_filter: None,
    };
    loader_run(Arc::clone(&loader), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert_eq!(state.is_loading, false);
    assert_eq!(state.is_running, false);

    fetcher
        .set_result(FetchResult::Ok(vec![Err(anyhow::Error::msg("error".to_string()))]))
        .await;
    loader_run(Arc::clone(&loader), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert_eq!(state.is_loading, false);
    assert_eq!(state.is_running, false);

    fetcher
        .set_result(FetchResult::Ok(vec![
            Ok(ContractData {
                address: "".to_string(),
                start_block: 1_u64,
                end_block,
                data: None,
            }),
            Err(anyhow::Error::msg("error".to_string())),
        ]))
        .await;
    loader_run(Arc::clone(&loader), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert_eq!(state.is_loading, false);
    assert_eq!(state.is_running, false);

    let end_block = 20_u64;
    fetcher
        .set_result(FetchResult::Ok(vec![
            Ok(ContractData {
                address: "".to_string(),
                start_block: 1_u64,
                end_block,
                data: None,
            }),
            Err(anyhow::Error::msg("error".to_string())),
        ]))
        .await;
    loader_run(Arc::clone(&loader), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert_eq!(state.is_loading, false);
    assert_eq!(state.is_running, false);
}

#[tokio::test]
async fn test_loader_start_shared_validator() {
    let end_block = 10_u64;
    let builder: ChainDataLoaderBuilder<FullData, MockFetcher<FullData>, MockValidator, MockHandler> =
        ChainDataLoaderBuilder::new();

    let fetcher_result = FetchResult::Ok(vec![Ok(ContractData {
        address: "".to_string(),
        start_block: 1_u64,
        end_block,
        data: None,
    })]);
    let fetcher = Arc::new(MockFetcher::new(fetcher_result));
    let validator = Arc::new(MockValidator::new(None));
    let handler = Arc::new(MockHandler::new(true));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();

    let loader = builder
        .chain_id(1)
        .initial_block(123_u64)
        .config(Arc::new(core_cfg))
        .add_shared_fetcher(Arc::clone(&fetcher))
        .add_shared_validator(Arc::clone(&validator))
        .add_shared_handler(Arc::clone(&handler))
        .build()
        .unwrap();
    let loader = Arc::new(loader);
    let option = StartOption {
        load_interval_ms: Some(10_u64),
        max_batch_block: Some(1000000_u64),
        contract_filter: None,
    };
    loader_run(Arc::clone(&loader), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert_eq!(state.is_loading, false);
    assert_eq!(state.is_running, false);

    validator.set_result(Some(false)).await;
    loader_run(Arc::clone(&loader), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert_eq!(state.is_loading, false);
    assert_eq!(state.is_running, false);

    validator.set_result(Some(true)).await;
    loader_run(Arc::clone(&loader), option).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert_eq!(state.is_loading, false);
    assert_eq!(state.is_running, false);
}

#[tokio::test]
async fn test_loader_start_shared_handler() {
    let end_block = 10_u64;
    let builder: ChainDataLoaderBuilder<FullData, MockFetcher<FullData>, MockValidator, MockHandler> =
        ChainDataLoaderBuilder::new();

    let fetcher_result = FetchResult::Ok(vec![Ok(ContractData {
        address: "".to_string(),
        start_block: 1_u64,
        end_block,
        data: None,
    })]);
    let fetcher = Arc::new(MockFetcher::new(fetcher_result));
    let validator = Arc::new(MockValidator::new(Some(true)));
    let handler = Arc::new(MockHandler::new(false));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();

    let loader = builder
        .chain_id(1)
        .initial_block(123_u64)
        .config(Arc::new(core_cfg))
        .add_shared_fetcher(Arc::clone(&fetcher))
        .add_shared_validator(Arc::clone(&validator))
        .add_shared_handler(Arc::clone(&handler))
        .build()
        .unwrap();
    let loader = Arc::new(loader);
    let option = StartOption {
        load_interval_ms: Some(10_u64),
        max_batch_block: Some(1000000_u64),
        contract_filter: None,
    };
    loader_run(Arc::clone(&loader), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert_eq!(state.is_loading, false);
    assert_eq!(state.is_running, false);

    handler.set_result(true).await;
    loader_run(Arc::clone(&loader), option).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert_eq!(state.is_loading, false);
    assert_eq!(state.is_running, false);
}
