use async_trait::async_trait;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::chain::ChainData;
use mystiko_dataloader::data::contract::ContractData;
use mystiko_dataloader::data::types::{FullData, LoadedData};
use mystiko_dataloader::fetcher::types::{ChainFetchOption, ContractFetchOption, DataFetcher, FetchResult};
use mystiko_dataloader::handler::types::{DataHandler, HandleOption};
use mystiko_dataloader::loader::chain::{ChainDataLoader, ChainDataLoaderBuilder, ContractState, StartOption};
use mystiko_dataloader::loader::listener::{LoaderEvent, Loaderlistener};
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
    pub event: RwLock<Vec<LoaderEvent>>,
}

impl MockListener {
    fn new() -> Self {
        MockListener {
            event: RwLock::new(vec![]),
        }
    }

    pub async fn get_events(&self) -> Vec<LoaderEvent> {
        self.event.read().await.clone()
    }

    pub async fn clear(&self) {
        self.event.write().await.clear();
    }
}

impl Default for MockListener {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Loaderlistener for MockListener {
    async fn callback(&self, event: &LoaderEvent) -> anyhow::Result<()> {
        self.event.write().await.push((*event).clone());
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
        true => FetchResult::Ok(vec![Ok(ContractData {
            address: "0xAddress1".to_string(),
            start_block: 1_u64,
            end_block,
            data: None,
        })]),
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
    let option = StartOption {
        load_interval_ms: Some(10_u64),
        max_batch_block: Some(1000000_u64),
        contract_filter: None,
    };
    let loader = Arc::new(create_loader(false, true, Some(true), true, end_block).await);
    loader_start(loader.clone(), option).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 987);
    assert!(!state.is_loading);
    assert!(!loader.is_loading().await);
    assert!(!state.is_running);
    assert!(!loader.is_running().await);
    assert_eq!(state.recent_error, None);
    assert_eq!(loader.recent_error().await, None);
    let mut contract_states = HashMap::new();
    contract_states.insert(
        "0xAddress1".to_string(),
        ContractState {
            loaded_block: end_block,
        },
    );
    assert_eq!(state.contract_states, contract_states);
}

#[tokio::test]
async fn test_loader_start_batch_builder() {
    let end_block = 765_u64;
    let option = StartOption {
        load_interval_ms: Some(10_u64),
        max_batch_block: Some(1000000_u64),
        contract_filter: None,
    };
    let loader = Arc::new(create_loader(true, false, Some(true), true, end_block).await);
    loader_start(loader.clone(), option).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!loader.is_loading().await);
    assert!(!state.is_running);
    assert!(!loader.is_running().await);
    assert!(state.recent_error.is_some());
    assert!(loader.recent_error().await.is_some());
    assert_eq!(state.contract_states, HashMap::new());
}

#[tokio::test]
async fn test_loader_start_shared_fetcher() {
    let (loader, fetchers, _, _, listeners) = create_shared_loader(1, 1, 1, 1).await;
    let option = StartOption {
        load_interval_ms: Some(10_u64),
        max_batch_block: Some(1000000_u64),
        contract_filter: None,
    };
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert!(state.recent_error.is_none());

    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert!(state.recent_error.unwrap().contains("failed fetch from all fetchers"));
    assert_eq!(state.contract_states, HashMap::new());
    assert_eq!(
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadFailureEvent,
            LoaderEvent::StopEvent
        ]
    );

    listeners[0].clear().await;
    let end_block = 10_u64;
    fetchers[0]
        .set_result(FetchResult::Ok(vec![
            Ok(ContractData {
                address: "0xAddress1".to_string(),
                start_block: 1_u64,
                end_block,
                data: None,
            }),
            Err(anyhow::Error::msg("error".to_string())),
        ]))
        .await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert!(state.recent_error.unwrap().contains("failed fetch from all fetchers"));
    let mut contract_states = HashMap::new();
    contract_states.insert(
        "0xAddress1".to_string(),
        ContractState {
            loaded_block: end_block,
        },
    );
    assert_eq!(state.contract_states, contract_states);
    assert_eq!(
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadSuccessEvent,
            LoaderEvent::StopEvent
        ]
    );

    listeners[0].clear().await;
    let end_block2 = end_block + 10;
    fetchers[0]
        .set_result(FetchResult::Ok(vec![
            Ok(ContractData {
                address: "0xAddress2".to_string(),
                start_block: 1_u64,
                end_block: end_block2,
                data: None,
            }),
            Err(anyhow::Error::msg("error".to_string())),
        ]))
        .await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert!(state.recent_error.unwrap().contains("failed fetch from all fetchers"));
    contract_states.insert(
        "0xAddress2".to_string(),
        ContractState {
            loaded_block: end_block2,
        },
    );
    assert_eq!(state.contract_states, contract_states);
    assert_eq!(
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadSuccessEvent,
            LoaderEvent::StopEvent
        ]
    );

    listeners[0].clear().await;
    fetchers[0]
        .set_result(FetchResult::Err(anyhow::Error::msg("error".to_string())))
        .await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert!(state.recent_error.unwrap().contains("failed fetch from all fetchers"));
    assert_eq!(state.contract_states, contract_states);
    assert_eq!(
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadFailureEvent,
            LoaderEvent::StopEvent
        ]
    );

    let end_block3 = end_block2 - 2;
    listeners[0].clear().await;
    fetchers[0]
        .set_result(FetchResult::Ok(vec![
            Ok(ContractData {
                address: "0xAddress1".to_string(),
                start_block: 1_u64,
                end_block: end_block3,
                data: None,
            }),
            Ok(ContractData {
                address: "0xAddress3".to_string(),
                start_block: 1_u64,
                end_block: end_block3,
                data: None,
            }),
        ]))
        .await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block3);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert!(state.recent_error.unwrap().contains("failed fetch from all fetchers"));
    let mut contract_states = HashMap::new();
    contract_states.insert(
        "0xAddress1".to_string(),
        ContractState {
            loaded_block: end_block3,
        },
    );
    contract_states.insert(
        "0xAddress2".to_string(),
        ContractState {
            loaded_block: end_block2,
        },
    );
    contract_states.insert(
        "0xAddress3".to_string(),
        ContractState {
            loaded_block: end_block3,
        },
    );
    assert_eq!(state.contract_states, contract_states);
    assert_eq!(
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadSuccessEvent,
            LoaderEvent::StopEvent
        ]
    );

    listeners[0].clear().await;
    fetchers[0].set_result(FetchResult::Ok(vec![])).await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block3);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert!(state.recent_error.unwrap().contains("failed fetch from all fetchers"));
    assert_eq!(state.contract_states, contract_states);
    assert_eq!(
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadSuccessEvent,
            LoaderEvent::StopEvent
        ]
    );
}

#[tokio::test]
async fn test_loader_start_two_shared_fetcher() {
    let (loader, fetchers, _, _, listeners) = create_shared_loader(2, 1, 1, 1).await;
    let option = StartOption {
        load_interval_ms: Some(10_u64),
        max_batch_block: Some(1000000_u64),
        contract_filter: None,
    };
    let end_block1 = 10_u64;
    let end_block2 = 20_u64;
    fetchers[0]
        .set_result(FetchResult::Ok(vec![Ok(ContractData {
            address: "0xAddress1".to_string(),
            start_block: 1_u64,
            end_block: end_block1,
            data: None,
        })]))
        .await;
    fetchers[1]
        .set_result(FetchResult::Ok(vec![Ok(ContractData {
            address: "0xAddress2".to_string(),
            start_block: 1_u64,
            end_block: end_block2,
            data: None,
        })]))
        .await;

    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block1);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(state.recent_error, None);
    let mut contract_states = HashMap::new();
    contract_states.insert(
        "0xAddress1".to_string(),
        ContractState {
            loaded_block: end_block1,
        },
    );
    assert_eq!(state.contract_states, contract_states);
    assert_eq!(
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadSuccessEvent,
            LoaderEvent::StopEvent
        ]
    );

    listeners[0].clear().await;
    fetchers[0].set_result(FetchResult::Ok(vec![])).await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block1);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(state.recent_error, None);
    let mut contract_states = HashMap::new();
    contract_states.insert(
        "0xAddress1".to_string(),
        ContractState {
            loaded_block: end_block1,
        },
    );
    assert_eq!(state.contract_states, contract_states);
    assert_eq!(
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadSuccessEvent,
            LoaderEvent::StopEvent
        ]
    );

    listeners[0].clear().await;
    fetchers[0]
        .set_result(FetchResult::Err(anyhow::Error::msg("error")))
        .await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block1);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(state.recent_error, None);
    let mut contract_states = HashMap::new();
    contract_states.insert(
        "0xAddress1".to_string(),
        ContractState {
            loaded_block: end_block1,
        },
    );
    contract_states.insert(
        "0xAddress2".to_string(),
        ContractState {
            loaded_block: end_block2,
        },
    );
    assert_eq!(state.contract_states, contract_states);
    assert_eq!(
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadSuccessEvent,
            LoaderEvent::StopEvent
        ]
    );

    let end_block3 = end_block2 + 10;
    listeners[0].clear().await;
    fetchers[0]
        .set_result(FetchResult::Ok(vec![Ok(ContractData {
            address: "0xAddress1".to_string(),
            start_block: 1_u64,
            end_block: end_block3,
            data: None,
        })]))
        .await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block2);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(state.recent_error, None);
    let mut contract_states = HashMap::new();
    contract_states.insert(
        "0xAddress1".to_string(),
        ContractState {
            loaded_block: end_block3,
        },
    );
    contract_states.insert(
        "0xAddress2".to_string(),
        ContractState {
            loaded_block: end_block2,
        },
    );
    assert_eq!(state.contract_states, contract_states);
    assert_eq!(
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadSuccessEvent,
            LoaderEvent::StopEvent
        ]
    );
}

#[tokio::test]
async fn test_loader_start_shared_validator() {
    let (loader, fetchers, validators, _, listeners) = create_shared_loader(1, 1, 1, 1).await;
    let end_block = 10_u64;
    let fetcher_result = FetchResult::Ok(vec![Ok(ContractData {
        address: "0xF20B03c02234F968AdB56cCd2bA2e3F44359A820".to_string(),
        start_block: 1_u64,
        end_block,
        data: None,
    })]);
    fetchers[0].set_result(fetcher_result).await;
    validators[0].set_result(None).await;
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
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadFailureEvent,
            LoaderEvent::StopEvent
        ]
    );

    listeners[0].clear().await;
    validators[0].set_result(Some(false)).await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadFailureEvent,
            LoaderEvent::StopEvent
        ]
    );

    listeners[0].clear().await;
    validators[0].set_result(Some(true)).await;
    loader_start(loader.clone(), option).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadSuccessEvent,
            LoaderEvent::StopEvent
        ]
    );
}

#[tokio::test]
async fn test_loader_start_two_shared_validator() {
    let (loader, fetchers, validators, _, listeners) = create_shared_loader(1, 2, 1, 1).await;
    fetchers[0].set_result(FetchResult::Ok(vec![])).await;
    validators[0].set_result(Some(true)).await;
    validators[1].set_result(Some(true)).await;

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
    assert_eq!(state.recent_error, None);
    assert_eq!(
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadSuccessEvent,
            LoaderEvent::StopEvent
        ]
    );

    listeners[0].clear().await;
    validators[0].set_result(Some(true)).await;
    validators[1].set_result(Some(false)).await;
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
    assert!(state.recent_error.unwrap().contains("failed fetch from all fetchers"));
    assert_eq!(
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadFailureEvent,
            LoaderEvent::StopEvent
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
    assert!(state.recent_error.unwrap().contains("handler error"));
    assert_eq!(
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadFailureEvent,
            LoaderEvent::StopEvent
        ]
    );

    listeners[0].clear().await;
    handlers[0].set_result(true).await;
    loader_start(loader.clone(), option).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert!(state.recent_error.unwrap().contains("handler error"));
    assert_eq!(
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadSuccessEvent,
            LoaderEvent::StopEvent
        ]
    );
}

#[tokio::test]
async fn test_loader_start_two_shared_handler() {
    let (loader, fetchers, _, handlers, listeners) = create_shared_loader(1, 1, 2, 1).await;

    let end_block = 10_u64;
    let fetcher_result = FetchResult::Ok(vec![Ok(ContractData {
        address: "0xAddress1".to_string(),
        start_block: 1_u64,
        end_block,
        data: None,
    })]);
    fetchers[0].set_result(fetcher_result).await;
    handlers[0].set_result(true).await;
    handlers[1].set_result(true).await;
    let option = StartOption {
        load_interval_ms: Some(10_u64),
        max_batch_block: Some(1000000_u64),
        contract_filter: None,
    };
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(state.recent_error, None);
    assert_eq!(
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadSuccessEvent,
            LoaderEvent::StopEvent
        ]
    );

    listeners[0].clear().await;
    handlers[0].set_result(false).await;
    handlers[1].set_result(true).await;
    let option = StartOption {
        load_interval_ms: Some(10_u64),
        max_batch_block: Some(1000000_u64),
        contract_filter: None,
    };
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert!(state.recent_error.unwrap().contains("handler error"));
    assert_eq!(
        listeners[0].get_events().await,
        vec![
            LoaderEvent::StartEvent,
            LoaderEvent::LoadEvent,
            LoaderEvent::LoadFailureEvent,
            LoaderEvent::StopEvent
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
