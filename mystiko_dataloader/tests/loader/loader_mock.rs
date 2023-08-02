use async_trait::async_trait;
use mystiko_config::wrapper::contract::ContractConfig;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::chain::ChainData;
use mystiko_dataloader::data::contract::ContractData;
use mystiko_dataloader::data::result::{ChainResult, ContractResult};
use mystiko_dataloader::data::types::{FullData, LoadedData};
use mystiko_dataloader::fetcher::types::{DataFetcher, FetchOption, FetchResult};
use mystiko_dataloader::filter::ContractFilter;
use mystiko_dataloader::handler::types::{DataHandler, HandleOption, HandleResult};
use mystiko_dataloader::loader::chain::{ChainDataLoader, ChainDataLoaderBuilder};
use mystiko_dataloader::loader::listener::{LoaderEvent, LoaderListener};
use mystiko_dataloader::loader::types::StartOption;
use mystiko_dataloader::validator::types::{DataValidator, ValidateOption, ValidateResult};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct MockContractFilter {
    contracts: HashMap<String, bool>,
}

impl MockContractFilter {
    pub fn new() -> Self {
        MockContractFilter {
            contracts: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.contracts.clear();
    }

    pub fn add_unfiltered_contract(&mut self, contract: &str) {
        self.contracts.insert(contract.to_string(), true);
    }

    pub fn add_unfiltered_contracts(&mut self, contracts: &[&str]) {
        contracts.iter().for_each(|&c| {
            self.contracts.insert(c.to_string(), true);
        });
    }
}

impl Default for MockContractFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractFilter for MockContractFilter {
    fn filter(&self, _chain_id: u64, contract_config: &ContractConfig) -> bool {
        !self.contracts.contains_key(contract_config.address())
    }
}

pub struct MockFetcher<R>
where
    R: LoadedData,
{
    pub chain_id: u64,
    pub result: RwLock<Option<ChainData<R>>>,
}

impl<R> MockFetcher<R>
where
    R: LoadedData + Clone,
{
    pub fn new(chain_id: u64) -> Self {
        MockFetcher {
            chain_id,
            result: RwLock::new(None),
        }
    }

    pub async fn set_result(&self, r: ChainData<R>) {
        *self.result.write().await = Some(r);
    }

    pub async fn set_error_result(&self) {
        *self.result.write().await = None;
    }
}

#[async_trait]
impl<R> DataFetcher<R> for MockFetcher<R>
where
    R: LoadedData + Clone,
{
    async fn fetch(&self, _option: &FetchOption) -> FetchResult<R> {
        if let Some(ref result) = *self.result.read().await {
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
            Err(anyhow::Error::msg("error".to_string()))
        }
    }
}

pub struct MockValidator {
    all_success: RwLock<bool>,
    result: RwLock<ValidateResult>,
}

impl Default for MockValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl MockValidator {
    pub fn new() -> Self {
        MockValidator {
            all_success: RwLock::new(true),
            result: RwLock::new(ValidateResult::Err(anyhow::Error::msg("validate error".to_string()))),
        }
    }

    pub async fn set_all_success(&self) {
        *self.all_success.write().await = true;
    }

    pub async fn set_result(&self, result: ValidateResult) {
        *self.all_success.write().await = false;
        *self.result.write().await = result;
    }
}

#[async_trait]
impl<R> DataValidator<R> for MockValidator
where
    R: LoadedData,
{
    async fn validate(&self, data: &ChainData<R>, _option: &ValidateOption) -> ValidateResult {
        if *self.all_success.read().await {
            let contract_results = data
                .contracts_data
                .iter()
                .map(|d| {
                    ContractResult::builder()
                        .address(d.address.clone())
                        .result(Ok(()))
                        .build()
                })
                .collect::<Vec<_>>();

            Ok(ChainResult::builder()
                .chain_id(data.chain_id)
                .contract_results(contract_results)
                .build())
        } else {
            let result = self.result.read().await;
            match &*result {
                Ok(result) => {
                    let contract_results = result
                        .contract_results
                        .iter()
                        .map(|d| match d.result {
                            Ok(_) => ContractResult::builder()
                                .address(d.address.clone())
                                .result(Ok(()))
                                .build(),
                            Err(_) => ContractResult::builder()
                                .address(d.address.clone())
                                .result(Err(anyhow::Error::msg("error".to_string())))
                                .build(),
                        })
                        .collect::<Vec<_>>();

                    Ok(ChainResult::builder()
                        .chain_id(result.chain_id)
                        .contract_results(contract_results)
                        .build())
                }
                Err(_) => Err(anyhow::Error::msg("handle error".to_string())),
            }
        }
    }
}

pub struct MockHandler {
    all_success: RwLock<bool>,
    result: RwLock<HandleResult>,
}

impl Default for MockHandler {
    fn default() -> Self {
        Self::new()
    }
}
impl MockHandler {
    pub fn new() -> Self {
        MockHandler {
            all_success: RwLock::new(true),
            result: RwLock::new(HandleResult::Err(anyhow::Error::msg("handle error".to_string()))),
        }
    }

    pub async fn set_all_success(&self) {
        *self.all_success.write().await = true;
    }

    pub async fn set_result(&self, result: HandleResult) {
        *self.all_success.write().await = false;
        *self.result.write().await = result;
    }
}

#[async_trait]
impl<R> DataHandler<R> for MockHandler
where
    R: LoadedData,
{
    async fn handle(&self, data: &ChainData<R>, _option: &HandleOption) -> HandleResult {
        if *self.all_success.read().await {
            let contract_results = data
                .contracts_data
                .iter()
                .map(|d| {
                    ContractResult::builder()
                        .address(d.address.clone())
                        .result(Ok(()))
                        .build()
                })
                .collect::<Vec<_>>();

            Ok(ChainResult::builder()
                .chain_id(data.chain_id)
                .contract_results(contract_results)
                .build())
        } else {
            let result = self.result.read().await;
            match &*result {
                Ok(result) => {
                    let contract_results = result
                        .contract_results
                        .iter()
                        .map(|d| match d.result {
                            Ok(_) => ContractResult::builder()
                                .address(d.address.clone())
                                .result(Ok(()))
                                .build(),
                            Err(_) => ContractResult::builder()
                                .address(d.address.clone())
                                .result(Err(anyhow::Error::msg("error".to_string())))
                                .build(),
                        })
                        .collect::<Vec<_>>();

                    Ok(ChainResult::builder()
                        .chain_id(result.chain_id)
                        .contract_results(contract_results)
                        .build())
                }
                Err(_) => Err(anyhow::Error::msg("handle error".to_string())),
            }
        }
    }
}

pub struct MockListener {
    pub event: RwLock<Vec<String>>,
}

impl MockListener {
    fn new() -> Self {
        MockListener {
            event: RwLock::new(vec![]),
        }
    }

    fn convert_event(&self, event: &LoaderEvent) -> String {
        match event {
            LoaderEvent::StartEvent(e) => format!("StartEvent-{}", e.start_block),
            LoaderEvent::StopEvent(e) => format!("StopEvent-{}", e.end_block),
            LoaderEvent::LoadEvent(e) => format!("LoadEvent-{}", e.start_block),
            LoaderEvent::LoadSuccessEvent(e) => format!("LoadSuccessEvent-{}", e.end_block),
            LoaderEvent::LoadFailureEvent(e) => {
                format!("LoadFailureEvent-{}-{}", e.end_block, e.load_error)
            }
        }
    }

    pub async fn drain_events(&self) -> Vec<String> {
        let mut event = self.event.write().await;
        event.drain(..).collect()
    }
}

impl Default for MockListener {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LoaderListener for MockListener {
    async fn callback(&self, event: &LoaderEvent) -> anyhow::Result<()> {
        let event_str = self.convert_event(event);
        self.event.write().await.push(event_str);
        Ok(())
    }
}

pub async fn loader_start(
    loader: Arc<ChainDataLoader<FullData, MockFetcher<FullData>, MockValidator, MockHandler, MockListener>>,
    option: StartOption,
) {
    let loader_clone1 = loader.clone();
    let loader_clone2 = loader.clone();
    let handle1 = tokio::spawn(async move { loader_clone1.start(&option).await });
    let handle2 = tokio::spawn(async move {
        let _ = sleep(std::time::Duration::from_millis(2_u64)).await;
        loader_clone2.stop().await
    });

    let result = futures::try_join!(handle1);
    assert!(result.is_ok());
    let result = futures::try_join!(handle2);
    assert!(result.is_ok());
}

pub async fn create_loader(
    batch: bool,
    fetch_result: bool,
    contract_address: &str,
    end_block: u64,
) -> ChainDataLoader<FullData, MockFetcher<FullData>, MockValidator, MockHandler, MockListener> {
    let chain_id = 1_u64;
    let builder: ChainDataLoaderBuilder<FullData, MockFetcher<FullData>, MockValidator, MockHandler, MockListener> =
        ChainDataLoaderBuilder::new();

    let fetcher = MockFetcher::new(chain_id);
    if fetch_result {
        fetcher
            .set_result(
                ChainData::builder()
                    .chain_id(chain_id)
                    .contracts_data(vec![ContractData::builder()
                        .address(contract_address.to_string())
                        .start_block(1_u64)
                        .end_block(end_block)
                        .build()])
                    .build(),
            )
            .await;
    }

    let validator = MockValidator::new();
    let handler = MockHandler::new();
    let listener = MockListener::default();
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();

    if batch {
        builder
            .chain_id(1)
            .initial_block(123_u64)
            .config(Arc::new(core_cfg))
            .add_fetchers(vec![fetcher])
            .add_validators(vec![validator])
            .handler(handler)
            .add_listeners(vec![listener])
            .build()
            .unwrap()
    } else {
        builder
            .chain_id(1)
            .initial_block(123_u64)
            .config(Arc::new(core_cfg))
            .add_fetcher(fetcher)
            .add_validator(validator)
            .handler(handler)
            .add_listener(listener)
            .build()
            .unwrap()
    }
}

pub async fn create_shared_loader(
    feature_count: usize,
    validator_count: usize,
    listener_count: usize,
) -> (
    Arc<ChainDataLoader<FullData, MockFetcher<FullData>, MockValidator, MockHandler, MockListener>>,
    Vec<Arc<MockFetcher<FullData>>>,
    Vec<Arc<MockValidator>>,
    Arc<MockHandler>,
    Vec<Arc<MockListener>>,
) {
    let chain_id = 1_u64;
    let builder: ChainDataLoaderBuilder<FullData, MockFetcher<FullData>, MockValidator, MockHandler, MockListener> =
        ChainDataLoaderBuilder::new();
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();

    let fetchers = (0..feature_count)
        .map(|_| Arc::new(MockFetcher::new(chain_id)))
        .collect::<Vec<_>>();
    let validators = (0..validator_count)
        .map(|_| Arc::new(MockValidator::new()))
        .collect::<Vec<_>>();
    let handler = Arc::new(MockHandler::new());
    let listeners = (0..listener_count)
        .map(|_| Arc::new(MockListener::default()))
        .collect::<Vec<_>>();

    let loader = builder
        .chain_id(chain_id)
        .initial_block(123_u64)
        .config(Arc::new(core_cfg))
        .add_shared_fetchers(fetchers.clone())
        .add_shared_validators(validators.clone())
        .shared_handler(handler.clone())
        .add_shared_listeners(listeners.clone())
        .build()
        .unwrap();

    (Arc::new(loader), fetchers, validators, handler, listeners)
}
