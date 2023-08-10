use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::U64;
use ethers_providers::{MockError, MockProvider, Provider as EthersProvider, RetryClientBuilder, RetryPolicy};
use mystiko_config::wrapper::contract::ContractConfig;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::chain::ChainData;
use mystiko_dataloader::data::contract::ContractData;
use mystiko_dataloader::data::result::{ChainResult, ContractResult};
use mystiko_dataloader::data::types::{FullData, LoadedData};
use mystiko_dataloader::fetcher::types::{DataFetcher, FetchOptions, FetchResult};
use mystiko_dataloader::handler::types::{
    CommitmentQueryOption, ContractCommitment, ContractNullifier, DataHandler, HandleOption, HandleResult,
    NullifierQueryOption,
};
use mystiko_dataloader::loader::chain::{ChainDataLoader, ChainDataLoaderBuilder};
use mystiko_dataloader::loader::listener::{LoaderEvent, LoaderListener};
use mystiko_dataloader::loader::types::{LoadOption, ScheduleOption};
use mystiko_dataloader::validator::types::{DataValidator, ValidateOption, ValidateResult};
use mystiko_ethers::provider::factory::Provider;
use mystiko_ethers::provider::failover::FailoverProvider;
use mystiko_ethers::provider::wrapper::ProviderWrapper;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Once};
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;

static INIT: Once = Once::new();

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
    async fn fetch(&self, _option: &FetchOptions) -> FetchResult<R> {
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
            result: RwLock::new(ValidateResult::Err(anyhow::Error::msg("error".to_string()))),
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
                Err(_) => Err(anyhow::Error::msg("error".to_string())),
            }
        }
    }
}

pub struct MockHandler<R>
where
    R: LoadedData,
{
    contracts: RwLock<Vec<ContractConfig>>,
    chain_loaded_blocks_error: RwLock<bool>,
    contract_loaded_blocks_error: RwLock<HashMap<String, bool>>,
    all_success: RwLock<bool>,
    result: RwLock<HandleResult>,
    data: RwLock<HashMap<String, ContractData<R>>>,
}

impl<R> Default for MockHandler<R>
where
    R: LoadedData + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<R> MockHandler<R>
where
    R: LoadedData + Clone,
{
    pub fn new() -> Self {
        MockHandler {
            contracts: RwLock::new(vec![]),
            chain_loaded_blocks_error: RwLock::new(false),
            contract_loaded_blocks_error: RwLock::new(HashMap::new()),
            all_success: RwLock::new(true),
            result: RwLock::new(HandleResult::Err(anyhow::Error::msg("handle error".to_string()))),
            data: RwLock::new(HashMap::new()),
        }
    }

    pub async fn set_contracts(&self, chain_id: u64, contracts: HashSet<&str>, core_cfg: Arc<MystikoConfig>) {
        let c = core_cfg
            .find_chain(chain_id)
            .unwrap()
            .contracts_with_disabled()
            .into_iter()
            .filter(|c| contracts.contains(c.address()))
            .collect::<Vec<_>>();

        *self.contracts.write().await = c;
    }

    pub async fn set_chain_loaded_blocks_error(&self, b_error: bool) {
        *self.chain_loaded_blocks_error.write().await = b_error;
    }

    pub async fn set_contract_loaded_blocks_error(&self, contract_address: &str, b_error: bool) {
        self.contract_loaded_blocks_error
            .write()
            .await
            .insert(contract_address.to_string(), b_error);
    }

    pub async fn set_all_success(&self) {
        *self.all_success.write().await = true;
    }

    pub async fn set_result(&self, result: HandleResult) {
        *self.all_success.write().await = false;
        *self.result.write().await = result;
    }

    pub async fn drain_data(&self) -> HashMap<String, ContractData<R>> {
        let mut data_write = self.data.write().await;
        let mut drained_data = HashMap::new();
        data_write.drain().for_each(|(k, v)| {
            drained_data.insert(k, v);
        });
        drained_data
    }
}

#[async_trait]
impl<R> DataHandler<R> for MockHandler<R>
where
    R: LoadedData,
{
    async fn query_loading_contracts(&self, _chain_id: u64) -> Result<Option<Vec<ContractConfig>>> {
        if self.contracts.read().await.is_empty() {
            Ok(None)
        } else {
            Ok(Some(self.contracts.read().await.clone()))
        }
    }

    async fn query_chain_loaded_block(&self, _chain_id: u64) -> anyhow::Result<Option<u64>> {
        if *self.chain_loaded_blocks_error.read().await {
            return Err(anyhow::Error::msg("error".to_string()));
        }

        let data = self.data.read().await;
        let min_end_block = data.iter().map(|(_, v)| v.end_block).min();

        Ok(min_end_block)
    }

    async fn query_contract_loaded_block(&self, _chain_id: u64, contract_address: &str) -> anyhow::Result<Option<u64>> {
        if let Some(blocks) = self.contract_loaded_blocks_error.read().await.get(contract_address) {
            if *blocks {
                return Err(anyhow::Error::msg("error".to_string()));
            }
        }

        let end_block = self
            .data
            .read()
            .await
            .get(contract_address)
            .map(|d| d.end_block)
            .unwrap_or(0_u64);

        Ok(Some(end_block))
    }

    async fn query_commitments(
        &self,
        _chain_id: u64,
        _option: &CommitmentQueryOption,
    ) -> Result<Vec<ContractCommitment>> {
        Err(anyhow::Error::msg("query_commitments error".to_string()))
    }

    async fn query_nullifiers(&self, _chain_id: u64, _option: &NullifierQueryOption) -> Result<Vec<ContractNullifier>> {
        Err(anyhow::Error::msg("query_commitments error".to_string()))
    }

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

            let mut data_write = self.data.write().await;
            data.contracts_data.iter().for_each(|d| {
                data_write.insert(
                    d.address.clone(),
                    ContractData::builder()
                        .address(d.address.clone())
                        .start_block(d.start_block)
                        .end_block(d.end_block)
                        .build(),
                );
            });

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

                    let mut data_write = self.data.write().await;
                    data.contracts_data.iter().for_each(|d| {
                        data_write.insert(
                            d.address.clone(),
                            ContractData::builder()
                                .address(d.address.clone())
                                .start_block(d.start_block)
                                .end_block(d.end_block)
                                .build(),
                        );
                    });

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

    fn convert_event(&self, event: Arc<LoaderEvent>) -> String {
        match &*event {
            LoaderEvent::ScheduleEvent(_) => "ScheduleEvent".to_string(),
            LoaderEvent::LoadEvent(e) => format!("LoadEvent-{}-{}", e.start_block, e.target_block),
            LoaderEvent::LoadSuccessEvent(e) => format!("LoadSuccessEvent-{}-{}", e.start_block, e.loaded_block),
            LoaderEvent::LoadFailureEvent(e) => {
                format!("LoadFailureEvent-{}-{}-{}", e.start_block, e.loaded_block, e.load_error)
            }
            LoaderEvent::StopScheduleEvent(_) => "StopScheduleEvent".to_string(),
        }
    }

    pub async fn is_event_empty(&self) -> bool {
        tokio::time::sleep(std::time::Duration::from_millis(5_u64)).await;
        let events_guard = self.event.write().await;
        events_guard.is_empty()
    }

    pub async fn drain_events(&self) -> Vec<String> {
        loop {
            let mut events_guard = self.event.write().await;
            if !events_guard.is_empty() {
                return events_guard.drain(..).collect();
            }
            drop(events_guard);
            tokio::time::sleep(std::time::Duration::from_millis(2_u64)).await;
        }
    }
}

impl Default for MockListener {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LoaderListener for MockListener {
    async fn callback(&self, _chain_id: u64, event: Arc<LoaderEvent>) -> anyhow::Result<()> {
        let event_str = self.convert_event(event);
        self.event.write().await.push(event_str);
        Ok(())
    }
}

type ChainDataLoaderFullDataType =
    ChainDataLoader<FullData, MockFetcher<FullData>, MockValidator, MockHandler<FullData>, MockListener>;

#[derive(Clone, Copy, Debug)]
pub enum LoaderRunType {
    Schedule,
    Load,
}

pub async fn loader_run(run_type: LoaderRunType, loader: Arc<ChainDataLoaderFullDataType>, delay_block: Option<u64>) {
    match run_type {
        LoaderRunType::Schedule => {
            loader_schedule(loader.clone(), delay_block).await;
        }
        LoaderRunType::Load => {
            loader_load(loader.clone(), delay_block).await;
        }
    }
}

async fn loader_load(loader: Arc<ChainDataLoaderFullDataType>, delay_block: Option<u64>) {
    let load_option = delay_block.map(|d| LoadOption::builder().delay_block(d).build());
    let _ = loader.load(load_option).await;
}

async fn loader_schedule(loader: Arc<ChainDataLoaderFullDataType>, delay_block: Option<u64>) {
    INIT.call_once(|| {
        std::env::set_var("RUST_LOG", "trace");
        env_logger::init();
    });

    let load_option = match delay_block {
        None => LoadOption::builder().build(),
        Some(d) => LoadOption::builder().delay_block(d).build(),
    };
    let option = ScheduleOption::builder()
        .schedule_interval_ms(5_u64)
        .load_option(load_option)
        .build();
    let handle = loader.schedule(option).await.unwrap().unwrap();
    let mut running = false;
    for _ in 0..100 {
        let _ = sleep(std::time::Duration::from_millis(1_u64)).await;
        if loader.is_running().await {
            running = true;
            break;
        }
    }
    assert!(running);
    loader.stop_schedule().await;
    let result = futures::try_join!(handle);
    assert!(result.is_ok());
}

pub async fn create_loader(
    batch: bool,
    fetch_result: bool,
    contract_address: &str,
    end_block: u64,
) -> ChainDataLoader<FullData, MockFetcher<FullData>, MockValidator, MockHandler<FullData>, MockListener> {
    let chain_id = 1_u64;

    let builder: ChainDataLoaderBuilder<
        FullData,
        MockFetcher<FullData>,
        MockValidator,
        MockHandler<FullData>,
        MockListener,
    > = ChainDataLoaderBuilder::new();

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

    let (_, mock) = EthersProvider::mocked();
    let provider = create_mock_provider(&mock);
    let provider = Arc::new(provider);

    let block_number = U64::from(10000);
    mock.push(block_number).unwrap();

    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();

    if batch {
        builder
            .chain_id(1)
            .config(Arc::new(core_cfg))
            .add_fetchers(vec![fetcher])
            .add_validators(vec![validator])
            .handler(handler)
            .add_listeners(vec![listener])
            .shared_provider(provider)
            .build()
            .unwrap()
    } else {
        builder
            .chain_id(1)
            .config(Arc::new(core_cfg))
            .add_fetcher(fetcher)
            .add_validator(validator)
            .handler(handler)
            .add_listener(listener)
            .shared_provider(provider)
            .build()
            .unwrap()
    }
}

pub async fn create_shared_loader(
    chain_id: u64,
    feature_count: usize,
    validator_count: usize,
    listener_count: usize,
) -> (
    Arc<MystikoConfig>,
    Arc<ChainDataLoader<FullData, MockFetcher<FullData>, MockValidator, MockHandler<FullData>, MockListener>>,
    Vec<Arc<MockFetcher<FullData>>>,
    Vec<Arc<MockValidator>>,
    Arc<MockHandler<FullData>>,
    Vec<Arc<MockListener>>,
    Arc<MockProvider>,
) {
    let builder: ChainDataLoaderBuilder<
        FullData,
        MockFetcher<FullData>,
        MockValidator,
        MockHandler<FullData>,
        MockListener,
    > = ChainDataLoaderBuilder::new();
    let core_cfg = Arc::new(
        MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );

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

    let (_, mock) = EthersProvider::mocked();
    let provider = create_mock_provider(&mock);
    let provider = Arc::new(provider);

    let loader = builder
        .chain_id(chain_id)
        .config(core_cfg.clone())
        .add_shared_fetchers(fetchers.clone())
        .add_shared_validators(validators.clone())
        .shared_handler(handler.clone())
        .add_shared_listeners(listeners.clone())
        .shared_provider(provider)
        .build()
        .unwrap();

    (
        core_cfg.clone(),
        Arc::new(loader),
        fetchers,
        validators,
        handler,
        listeners,
        Arc::new(mock),
    )
}

#[derive(Debug, Default)]
struct MockProviderRetryPolicy;

impl RetryPolicy<MockError> for MockProviderRetryPolicy {
    fn should_retry(&self, _error: &MockError) -> bool {
        false
    }

    fn backoff_hint(&self, _error: &MockError) -> Option<Duration> {
        Duration::from_secs(10).into()
    }
}

fn create_mock_provider(provider: &MockProvider) -> Provider {
    let retry_provider_builder = RetryClientBuilder::default();
    let inner_provider = retry_provider_builder.build(provider.clone(), Box::<MockProviderRetryPolicy>::default());

    let mut provider_builder = FailoverProvider::dyn_rpc();
    provider_builder = provider_builder.add_provider(Box::new(inner_provider));
    Provider::new(ProviderWrapper::new(Box::new(provider_builder.build())))
}

pub fn contract_data_partial_eq(a: &HashMap<String, ContractData<FullData>>, b: &Vec<ContractData<FullData>>) -> bool {
    if a.len() != b.len() {
        println!("a.len() != b.len() {:?} {:?}", a.len(), b.len());
        return false;
    }

    for d in b.iter() {
        if let Some(v) = a.get(&d.address) {
            if v.address != d.address {
                println!("v.address != d.address");
                return false;
            }

            if v.start_block != d.start_block {
                println!("v.start_block != d.start_block");
                return false;
            }

            if v.end_block != d.end_block {
                println!("v.end_block != d.end_block");
                return false;
            }
        } else {
            println!("Address not found: {}", &d.address);
            return false;
        }
    }

    true
}

pub async fn events_check(run_type: LoaderRunType, listeners: &[Arc<MockListener>], events: Vec<String>) {
    let mut total_event = vec![];
    match run_type {
        LoaderRunType::Schedule => {
            total_event.push("ScheduleEvent".to_string());
            total_event.extend(events);
            total_event.push("StopScheduleEvent".to_string());
        }
        LoaderRunType::Load => {
            total_event.extend(events);
        }
    }

    for listener in listeners.iter() {
        assert_eq!(listener.drain_events().await, total_event.clone());
    }
}
