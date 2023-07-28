use async_trait::async_trait;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::chain::ChainData;
use mystiko_dataloader::data::contract::ContractData;
use mystiko_dataloader::data::types::{FullData, LoadedData};
use mystiko_dataloader::fetcher::types::{ChainFetchOption, ContractFetchOption, DataFetcher, FetchResult};
use mystiko_dataloader::handler::types::{DataHandler, HandleOption};
use mystiko_dataloader::loader::chain::{ChainDataLoader, ChainDataLoaderBuilder, ContractState, StartOption};
use mystiko_dataloader::loader::listener::{LoaderEvent, LoaderListener};
use mystiko_dataloader::validator::types::{DataValidator, ValidateOption};
use std::collections::HashMap;
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

    pub async fn set_result(&self, r: FetchResult<R>) {
        let mut result = self.result.write().await;
        *result = r;
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
        match *self.result.read().await {
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
        if *self.result.read().await {
            Ok(())
        } else {
            Err(anyhow::Error::msg("handler error".to_string()))
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

async fn loader_start(
    loader: Arc<ChainDataLoader<FullData, MockFetcher<FullData>, MockValidator, MockHandler, MockListener>>,
    option: StartOption,
) {
    let loader_clone1 = loader.clone();
    let loader_clone2 = loader.clone();
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
    batch: bool,
    fetch_result: bool,
    validator_result: Option<bool>,
    handler_result: bool,
    end_block: u64,
) -> ChainDataLoader<FullData, MockFetcher<FullData>, MockValidator, MockHandler, MockListener> {
    let builder: ChainDataLoaderBuilder<FullData, MockFetcher<FullData>, MockValidator, MockHandler, MockListener> =
        ChainDataLoaderBuilder::new();

    let fetcher_result = match fetch_result {
        true => FetchResult::Ok(vec![Ok(ContractData::builder()
            .address("0xAddress1".to_string())
            .start_block(1_u64)
            .end_block(end_block)
            .build())]),
        false => FetchResult::Err(anyhow::Error::msg("error".to_string())),
    };

    let fetcher = MockFetcher::new(fetcher_result);
    let validator = MockValidator::new(validator_result);
    let handler = MockHandler::new(handler_result);
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
            .add_handlers(vec![handler])
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
            .add_handler(handler)
            .add_listener(listener)
            .build()
            .unwrap()
    }
}

async fn create_shared_loader(
    feature_count: usize,
    validator_count: usize,
    handler_count: usize,
    listener_count: usize,
) -> (
    Arc<ChainDataLoader<FullData, MockFetcher<FullData>, MockValidator, MockHandler, MockListener>>,
    Vec<Arc<MockFetcher<FullData>>>,
    Vec<Arc<MockValidator>>,
    Vec<Arc<MockHandler>>,
    Vec<Arc<MockListener>>,
) {
    let builder: ChainDataLoaderBuilder<FullData, MockFetcher<FullData>, MockValidator, MockHandler, MockListener> =
        ChainDataLoaderBuilder::new();
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();

    let fetchers = (0..feature_count)
        .map(|_| {
            Arc::new(MockFetcher::new(FetchResult::Err(anyhow::Error::msg(
                "error".to_string(),
            ))))
        })
        .collect::<Vec<_>>();
    let validators = (0..validator_count)
        .map(|_| Arc::new(MockValidator::new(Some(true))))
        .collect::<Vec<_>>();
    let handlers = (0..handler_count)
        .map(|_| Arc::new(MockHandler::new(true)))
        .collect::<Vec<_>>();
    let listeners = (0..listener_count)
        .map(|_| Arc::new(MockListener::default()))
        .collect::<Vec<_>>();

    let loader = builder
        .chain_id(1)
        .initial_block(123_u64)
        .config(Arc::new(core_cfg))
        .add_shared_fetchers(fetchers.clone())
        .add_shared_validators(validators.clone())
        .add_shared_handlers(handlers.clone())
        .add_shared_listeners(listeners.clone())
        .build()
        .unwrap();

    (Arc::new(loader), fetchers, validators, handlers, listeners)
}

#[tokio::test]
async fn test_loader_start() {
    let end_block = 987_u64;
    let option = StartOption::builder()
        .load_interval_ms(10_u64)
        .max_batch_block(1000000_u64)
        .build();
    let loader = Arc::new(create_loader(false, true, Some(true), true, end_block).await);
    loader_start(loader.clone(), option).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 987);
    assert!(!state.is_loading);
    assert!(!loader.is_loading().await);
    assert!(!state.is_running);
    assert!(!loader.is_running().await);

    let mut contract_states = HashMap::new();
    contract_states.insert(
        "0xAddress1".to_string(),
        ContractState::builder().loaded_block(end_block).build(),
    );
    assert_eq!(state.contract_states, contract_states);
}

#[tokio::test]
async fn test_loader_start_batch_builder() {
    let end_block = 765_u64;
    let option = StartOption::builder()
        .load_interval_ms(10_u64)
        .max_batch_block(1000000_u64)
        .build();
    let loader = Arc::new(create_loader(true, false, Some(true), true, end_block).await);
    loader_start(loader.clone(), option).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!loader.is_loading().await);
    assert!(!state.is_running);
    assert!(!loader.is_running().await);
    assert_eq!(state.contract_states, HashMap::new());
}

#[tokio::test]
async fn test_loader_start_shared_fetcher() {
    let (loader, fetchers, _, _, listeners) = create_shared_loader(1, 1, 1, 1).await;
    let option = StartOption::builder()
        .load_interval_ms(10_u64)
        .max_batch_block(1000000_u64)
        .build();
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!state.is_running);

    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(state.contract_states, HashMap::new());
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-124".to_string(),
            "LoadEvent-124".to_string(),
            "LoadFailureEvent-123-loader run error failed fetch from all fetchers".to_string(),
            "StopEvent-123".to_string()
        ]
    );

    let end_block = 10_u64;
    fetchers[0]
        .set_result(FetchResult::Ok(vec![
            Ok(ContractData::builder()
                .address("0xAddress1".to_string())
                .start_block(1_u64)
                .end_block(end_block)
                .build()),
            Err(anyhow::Error::msg("error".to_string())),
        ]))
        .await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    let mut contract_states = HashMap::new();
    contract_states.insert(
        "0xAddress1".to_string(),
        ContractState::builder().loaded_block(end_block).build(),
    );
    assert_eq!(state.contract_states, contract_states);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-124".to_string(),
            "LoadEvent-124".to_string(),
            "LoadSuccessEvent-10".to_string(),
            "StopEvent-10".to_string()
        ]
    );

    let end_block2 = end_block + 10;
    fetchers[0]
        .set_result(FetchResult::Ok(vec![
            Ok(ContractData::builder()
                .address("0xAddress2".to_string())
                .start_block(1_u64)
                .end_block(end_block2)
                .build()),
            Err(anyhow::Error::msg("error".to_string())),
        ]))
        .await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    contract_states.insert(
        "0xAddress2".to_string(),
        ContractState::builder().loaded_block(end_block2).build(),
    );
    assert_eq!(state.contract_states, contract_states);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-11".to_string(),
            "LoadEvent-11".to_string(),
            "LoadSuccessEvent-10".to_string(),
            "StopEvent-10".to_string()
        ]
    );

    fetchers[0]
        .set_result(FetchResult::Err(anyhow::Error::msg("error".to_string())))
        .await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(state.contract_states, contract_states);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-11".to_string(),
            "LoadEvent-11".to_string(),
            "LoadFailureEvent-10-loader run error failed fetch from all fetchers".to_string(),
            "StopEvent-10".to_string()
        ]
    );

    let end_block3 = end_block2 - 2;
    fetchers[0]
        .set_result(FetchResult::Ok(vec![
            Ok(ContractData::builder()
                .address("0xAddress1".to_string())
                .start_block(1_u64)
                .end_block(end_block3)
                .build()),
            Ok(ContractData::builder()
                .address("0xAddress3".to_string())
                .start_block(1_u64)
                .end_block(end_block3)
                .build()),
        ]))
        .await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block3);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    let mut contract_states = HashMap::new();
    contract_states.insert(
        "0xAddress1".to_string(),
        ContractState::builder().loaded_block(end_block3).build(),
    );
    contract_states.insert(
        "0xAddress2".to_string(),
        ContractState::builder().loaded_block(end_block2).build(),
    );
    contract_states.insert(
        "0xAddress3".to_string(),
        ContractState::builder().loaded_block(end_block3).build(),
    );
    assert_eq!(state.contract_states, contract_states);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-11".to_string(),
            "LoadEvent-11".to_string(),
            "LoadSuccessEvent-18".to_string(),
            "StopEvent-18".to_string()
        ]
    );

    fetchers[0].set_result(FetchResult::Ok(vec![])).await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block3);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(state.contract_states, contract_states);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-19".to_string(),
            "LoadEvent-19".to_string(),
            "LoadSuccessEvent-18".to_string(),
            "StopEvent-18".to_string()
        ]
    );
}

#[tokio::test]
async fn test_loader_start_two_shared_fetcher() {
    let (loader, fetchers, _, _, listeners) = create_shared_loader(2, 1, 1, 1).await;

    let option = StartOption::builder()
        .load_interval_ms(10_u64)
        .max_batch_block(1000000_u64)
        .build();
    let end_block1 = 10_u64;
    let end_block2 = 20_u64;
    fetchers[0]
        .set_result(FetchResult::Ok(vec![Ok(ContractData::builder()
            .address("0xAddress1".to_string())
            .start_block(1_u64)
            .end_block(end_block1)
            .build())]))
        .await;
    fetchers[1]
        .set_result(FetchResult::Ok(vec![Ok(ContractData::builder()
            .address("0xAddress2".to_string())
            .start_block(1_u64)
            .end_block(end_block2)
            .build())]))
        .await;

    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block1);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    let mut contract_states = HashMap::new();
    contract_states.insert(
        "0xAddress1".to_string(),
        ContractState::builder().loaded_block(end_block1).build(),
    );
    assert_eq!(state.contract_states, contract_states);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-124".to_string(),
            "LoadEvent-124".to_string(),
            "LoadSuccessEvent-10".to_string(),
            "StopEvent-10".to_string()
        ]
    );
    fetchers[0].set_result(FetchResult::Ok(vec![])).await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block1);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    let mut contract_states = HashMap::new();
    contract_states.insert(
        "0xAddress1".to_string(),
        ContractState::builder().loaded_block(end_block1).build(),
    );
    assert_eq!(state.contract_states, contract_states);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-11".to_string(),
            "LoadEvent-11".to_string(),
            "LoadSuccessEvent-10".to_string(),
            "StopEvent-10".to_string()
        ]
    );

    fetchers[0]
        .set_result(FetchResult::Err(anyhow::Error::msg("error")))
        .await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block1);
    assert!(!state.is_loading);
    assert!(!state.is_running);

    let mut contract_states = HashMap::new();
    contract_states.insert(
        "0xAddress1".to_string(),
        ContractState::builder().loaded_block(end_block1).build(),
    );
    contract_states.insert(
        "0xAddress2".to_string(),
        ContractState::builder().loaded_block(end_block2).build(),
    );
    assert_eq!(state.contract_states, contract_states);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-11".to_string(),
            "LoadEvent-11".to_string(),
            "LoadSuccessEvent-10".to_string(),
            "StopEvent-10".to_string()
        ]
    );

    let end_block3 = end_block2 + 10;

    fetchers[0]
        .set_result(FetchResult::Ok(vec![Ok(ContractData::builder()
            .address("0xAddress1".to_string())
            .start_block(1_u64)
            .end_block(end_block3)
            .build())]))
        .await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block2);
    assert!(!state.is_loading);
    assert!(!state.is_running);

    let mut contract_states = HashMap::new();
    contract_states.insert(
        "0xAddress1".to_string(),
        ContractState::builder().loaded_block(end_block3).build(),
    );
    contract_states.insert(
        "0xAddress2".to_string(),
        ContractState::builder().loaded_block(end_block2).build(),
    );
    assert_eq!(state.contract_states, contract_states);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-11".to_string(),
            "LoadEvent-11".to_string(),
            "LoadSuccessEvent-20".to_string(),
            "StopEvent-20".to_string()
        ]
    );
}

#[tokio::test]
async fn test_loader_start_shared_validator() {
    let (loader, fetchers, validators, _, listeners) = create_shared_loader(1, 1, 1, 1).await;
    let end_block = 10_u64;
    let fetcher_result = FetchResult::Ok(vec![Ok(ContractData::builder()
        .address("0xAddress1".to_string())
        .start_block(1_u64)
        .end_block(end_block)
        .build())]);
    fetchers[0].set_result(fetcher_result).await;
    validators[0].set_result(None).await;

    let option = StartOption::builder()
        .load_interval_ms(10_u64)
        .max_batch_block(1000000_u64)
        .build();
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-124".to_string(),
            "LoadEvent-124".to_string(),
            "LoadFailureEvent-123-loader run error failed fetch from all fetchers".to_string(),
            "StopEvent-123".to_string()
        ]
    );

    validators[0].set_result(Some(false)).await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-124".to_string(),
            "LoadEvent-124".to_string(),
            "LoadFailureEvent-123-loader run error failed fetch from all fetchers".to_string(),
            "StopEvent-123".to_string()
        ]
    );

    validators[0].set_result(Some(true)).await;
    loader_start(loader.clone(), option).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-124".to_string(),
            "LoadEvent-124".to_string(),
            "LoadSuccessEvent-10".to_string(),
            "StopEvent-10".to_string()
        ]
    );
}

#[tokio::test]
async fn test_loader_start_two_shared_validator() {
    let (loader, fetchers, validators, _, listeners) = create_shared_loader(1, 2, 1, 1).await;
    fetchers[0].set_result(FetchResult::Ok(vec![])).await;
    validators[0].set_result(Some(true)).await;
    validators[1].set_result(Some(true)).await;

    let option = StartOption::builder()
        .load_interval_ms(10_u64)
        .max_batch_block(1000000_u64)
        .build();
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-124".to_string(),
            "LoadEvent-124".to_string(),
            "LoadSuccessEvent-123".to_string(),
            "StopEvent-123".to_string()
        ]
    );

    validators[0].set_result(Some(true)).await;
    validators[1].set_result(Some(false)).await;

    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-124".to_string(),
            "LoadEvent-124".to_string(),
            "LoadFailureEvent-123-loader run error failed fetch from all fetchers".to_string(),
            "StopEvent-123".to_string()
        ]
    );
}

#[tokio::test]
async fn test_loader_start_shared_handler() {
    let (loader, fetchers, _, handlers, listeners) = create_shared_loader(1, 1, 1, 1).await;

    let end_block = 10_u64;
    let fetcher_result = FetchResult::Ok(vec![Ok(ContractData {
        address: "0xAddress1".to_string(),
        start_block: 1_u64,
        end_block,
        data: None,
    })]);
    fetchers[0].set_result(fetcher_result).await;
    handlers[0].set_result(false).await;
    let option = StartOption {
        load_interval_ms: Some(10_u64),
        max_batch_block: Some(1000000_u64),
        contract_filter: None,
    };
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-124".to_string(),
            "LoadEvent-124".to_string(),
            "LoadFailureEvent-123-handler error".to_string(),
            "StopEvent-123".to_string()
        ]
    );

    handlers[0].set_result(true).await;
    loader_start(loader.clone(), option).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-124".to_string(),
            "LoadEvent-124".to_string(),
            "LoadSuccessEvent-10".to_string(),
            "StopEvent-10".to_string()
        ]
    );
}

#[tokio::test]
async fn test_loader_start_two_shared_handler() {
    let (loader, fetchers, _, handlers, listeners) = create_shared_loader(1, 1, 2, 1).await;
    let start_option = StartOption::builder()
        .load_interval_ms(10_u64)
        .max_batch_block(100000_u64)
        .build();

    let end_block = 10_u64;
    let fetcher_result = FetchResult::Ok(vec![Ok(ContractData::builder()
        .address("0xAddress1".to_string())
        .start_block(1_u64)
        .end_block(end_block)
        .build())]);
    fetchers[0].set_result(fetcher_result).await;
    handlers[0].set_result(true).await;
    handlers[1].set_result(true).await;
    loader_start(loader.clone(), start_option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-124".to_string(),
            "LoadEvent-124".to_string(),
            "LoadSuccessEvent-10".to_string(),
            "StopEvent-10".to_string()
        ]
    );

    handlers[0].set_result(false).await;
    handlers[1].set_result(true).await;
    loader_start(loader.clone(), start_option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-11".to_string(),
            "LoadEvent-11".to_string(),
            "LoadFailureEvent-10-handler error".to_string(),
            "StopEvent-10".to_string()
        ]
    );
}

#[tokio::test]
async fn test_add_shared_handler() {
    let builder: ChainDataLoaderBuilder<FullData, MockFetcher<FullData>, MockValidator, MockHandler, MockListener> =
        ChainDataLoaderBuilder::new();
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let loader = builder
        .chain_id(1)
        .initial_block(123_u64)
        .config(Arc::new(core_cfg))
        .add_shared_fetcher(Arc::new(MockFetcher::new(FetchResult::Err(anyhow::Error::msg(
            "error".to_string(),
        )))))
        .add_shared_validator(Arc::new(MockValidator::new(Some(true))))
        .add_shared_handler(Arc::new(MockHandler::new(true)))
        .add_shared_listener(Arc::new(MockListener::default()))
        .build()
        .unwrap();
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!state.is_running);
}
