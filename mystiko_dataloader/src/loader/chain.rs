use crate::data::chain::ChainData;
use crate::data::types::LoadedData;
use crate::error::DataloaderError;
use crate::fetcher::types::{ChainFetchOption, DataFetcher};
use crate::filter::ContractFilter;
use crate::handler::types::{DataHandler, HandleOption};
use crate::loader::listener::{
    LoadEvent, LoadFailureEvent, LoadSuccessEvent, LoaderEvent, LoaderListener, StartEvent, StopEvent,
};
use crate::validator::types::{DataValidator, ValidateOption};
use log::error;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;
use typed_builder::TypedBuilder;

type Result<T> = anyhow::Result<T, DataloaderError>;

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct ContractState {
    pub loaded_block: u64,
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct ChainState {
    pub loaded_block: u64,
    pub is_running: bool,
    pub is_loading: bool,
    pub contract_states: HashMap<String, ContractState>,
}

#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct StartOption {
    #[builder(setter(strip_option))]
    pub load_interval_ms: Option<u64>,
    #[builder(setter(strip_option))]
    pub max_batch_block: Option<u64>,
    #[builder(default, setter(strip_option))]
    pub contract_filter: Option<Arc<Box<dyn ContractFilter>>>,
}

#[derive(Debug)]
pub struct ChainDataLoader<
    R,
    F = Box<dyn DataFetcher<R>>,
    V = Box<dyn DataValidator<R>>,
    H = Box<dyn DataHandler<R>>,
    L = Box<dyn LoaderListener>,
> {
    config: Arc<MystikoConfig>,
    chain_id: u64,
    fetchers: Vec<Arc<F>>,
    validators: Vec<Arc<V>>,
    handlers: Vec<Arc<H>>,
    listeners: Vec<Arc<L>>,
    state: RwLock<ChainState>,
    _phantom: std::marker::PhantomData<R>,
}

#[derive(Debug, Default)]
pub struct ChainDataLoaderBuilder<
    R,
    F = Box<dyn DataFetcher<R>>,
    V = Box<dyn DataValidator<R>>,
    H = Box<dyn DataHandler<R>>,
    L = Box<dyn LoaderListener>,
> {
    config: Option<Arc<MystikoConfig>>,
    chain_id: u64,
    initial_block: u64,
    fetchers: Vec<Arc<F>>,
    validators: Vec<Arc<V>>,
    handlers: Vec<Arc<H>>,
    listeners: Vec<Arc<L>>,
    _phantom: std::marker::PhantomData<R>,
}

impl<R, F, V, H, L> ChainDataLoader<R, F, V, H, L>
where
    R: LoadedData,
    F: DataFetcher<R>,
    V: DataValidator<R>,
    H: DataHandler<R>,
    L: LoaderListener,
{
    pub async fn start(&self, options: &StartOption) -> Result<()> {
        let load_interval_ms = options.load_interval_ms.unwrap_or(10000);

        if !self.set_is_running(true).await {
            self.emit_event(&LoaderEvent::StartEvent(
                StartEvent::builder().start_block(self.loaded_block().await + 1).build(),
            ))
            .await;

            loop {
                if !self.is_running().await {
                    break;
                }

                self.load(options).await;

                if !self.is_running().await {
                    break;
                }
                sleep(Duration::from_millis(load_interval_ms)).await;
            }

            self.emit_event(&LoaderEvent::StopEvent(
                StopEvent::builder().end_block(self.loaded_block().await).build(),
            ))
            .await;
        } else {
            error!("loader is already running");
        }

        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        self.set_is_running(false).await;
        Ok(())
    }

    pub async fn is_running(&self) -> bool {
        self.state.read().await.is_running
    }

    pub async fn is_loading(&self) -> bool {
        self.state.read().await.is_loading
    }

    pub async fn loaded_block(&self) -> u64 {
        self.state.read().await.loaded_block
    }

    pub async fn state(&self) -> ChainState {
        self.state.read().await.clone()
    }

    async fn set_is_running(&self, is_running: bool) -> bool {
        let mut last_state = self.state.read().await.is_running;
        if last_state != is_running {
            let mut writer = self.state.write().await;
            last_state = writer.is_running;
            if last_state != is_running {
                writer.is_running = is_running;
            }
        }
        last_state
    }

    async fn set_is_loading(&self, is_loading: bool) -> bool {
        let mut last_state = self.state.read().await.is_loading;
        if last_state != is_loading {
            let mut writer = self.state.write().await;
            last_state = writer.is_loading;
            if last_state != is_loading {
                writer.is_loading = is_loading;
            }
        }
        last_state
    }

    async fn load(&self, options: &StartOption) {
        if !self.set_is_loading(true).await {
            let start_block = self.loaded_block().await + 1;
            self.emit_event(&LoaderEvent::LoadEvent(
                LoadEvent::builder().start_block(start_block).build(),
            ))
            .await;
            let result = self.execute_load(start_block, options).await;
            match result {
                Ok(_) => {
                    self.emit_event(&LoaderEvent::LoadSuccessEvent(
                        LoadSuccessEvent::builder().end_block(self.loaded_block().await).build(),
                    ))
                    .await;
                }
                Err(e) => {
                    self.emit_event(&LoaderEvent::LoadFailureEvent(
                        LoadFailureEvent::builder()
                            .end_block(self.loaded_block().await)
                            .load_error(e)
                            .build(),
                    ))
                    .await;
                }
            }
            self.set_is_loading(false).await;
        }
    }

    async fn execute_load(&self, start_block: u64, options: &StartOption) -> Result<()> {
        let fetch_option_builder = ChainFetchOption::builder()
            .config(self.config.clone())
            .chain_id(self.chain_id)
            .start_block(start_block)
            .end_block(start_block + options.max_batch_block.unwrap_or(1000000));

        let fetch_option = if let Some(filter) = options.contract_filter.clone() {
            fetch_option_builder.contract_filter(filter).build()
        } else {
            fetch_option_builder.build()
        };

        let validator_option = ValidateOption::builder().config(self.config.clone()).build();
        let handler_option = HandleOption::builder().config(self.config.clone()).build();

        for fetcher in &self.fetchers {
            let chain_data = match fetcher.fetch_chain(&fetch_option).await {
                Err(_) => continue,
                Ok(chain_data) => chain_data,
            };

            // todo refetch failed contract?
            let validate_data = ChainData::builder()
                .chain_id(self.chain_id)
                .contracts_data(
                    chain_data
                        .into_iter()
                        .filter_map(|r| match r {
                            Ok(data) => Some(data),
                            Err(e) => {
                                error!("fetch meet error {:?}", e);
                                None
                            }
                        })
                        .collect::<Vec<_>>(),
                )
                .build();

            let mut validate_result = true;
            for validator in &self.validators {
                let result = validator.validate(&validate_data, &validator_option).await;
                if let Err(_) | Ok(false) = result {
                    validate_result = false;
                    break;
                }
            }

            if !validate_result {
                continue;
            }

            for handler in &self.handlers {
                handler.handle(&validate_data, &handler_option).await?;
            }

            return self.update_state(&validate_data).await;
        }

        error!("failed fetch from all fetchers");
        Err(DataloaderError::LoaderRunError(
            "failed fetch from all fetchers".to_string(),
        ))
    }

    async fn update_state(&self, chain_data: &ChainData<R>) -> Result<()> {
        if !chain_data.contracts_data.is_empty() {
            let mut state = self.state.write().await;
            chain_data.contracts_data.iter().for_each(|contract_data| {
                if let Some(contract_state) = state.contract_states.get_mut(&contract_data.address) {
                    contract_state.loaded_block = contract_data.end_block;
                } else {
                    state.contract_states.insert(
                        contract_data.address.clone(),
                        ContractState {
                            loaded_block: contract_data.end_block,
                        },
                    );
                }
            });

            if let Some((_, first_state)) = state.contract_states.iter().next() {
                let mut min_end_block = first_state.loaded_block;
                state.contract_states.iter().for_each(|(_, contract_state)| {
                    if contract_state.loaded_block < min_end_block {
                        min_end_block = contract_state.loaded_block;
                    }
                });
                state.loaded_block = min_end_block;
            }
        }
        Ok(())
    }

    async fn emit_event(&self, event: &LoaderEvent) {
        for listener in &self.listeners {
            listener.callback(event).await.unwrap_or_else(|e| {
                error!("emit event meet error {:?}", e);
            });
        }
    }
}

impl<R, F, V, H, L> ChainDataLoaderBuilder<R, F, V, H, L>
where
    R: LoadedData,
    F: DataFetcher<R>,
    V: DataValidator<R>,
    H: DataHandler<R>,
    L: LoaderListener,
{
    pub fn new() -> Self {
        Self {
            config: None,
            chain_id: 0,
            initial_block: 0,
            fetchers: Vec::new(),
            validators: Vec::new(),
            handlers: Vec::new(),
            listeners: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn config(mut self, config: Arc<MystikoConfig>) -> Self {
        self.config = Some(config);
        self
    }

    pub fn chain_id(mut self, chain_id: u64) -> Self {
        self.chain_id = chain_id;
        self
    }

    pub fn initial_block(mut self, initial_block: u64) -> Self {
        self.initial_block = initial_block;
        self
    }

    pub fn add_fetcher(self, fetcher: F) -> Self {
        self.add_shared_fetcher(Arc::new(fetcher))
    }

    pub fn add_fetchers(self, fetchers: Vec<F>) -> Self {
        self.add_shared_fetchers(fetchers.into_iter().map(Arc::new).collect())
    }

    pub fn add_shared_fetcher(mut self, fetcher: Arc<F>) -> Self {
        self.fetchers.push(fetcher);
        self
    }

    pub fn add_shared_fetchers(mut self, fetchers: Vec<Arc<F>>) -> Self {
        self.fetchers.extend(fetchers);
        self
    }

    pub fn add_validator(self, validator: V) -> Self {
        self.add_shared_validator(Arc::new(validator))
    }

    pub fn add_validators(self, validators: Vec<V>) -> Self {
        self.add_shared_validators(validators.into_iter().map(Arc::new).collect())
    }

    pub fn add_shared_validator(mut self, validator: Arc<V>) -> Self {
        self.validators.push(validator);
        self
    }

    pub fn add_shared_validators(mut self, validators: Vec<Arc<V>>) -> Self {
        self.validators.extend(validators);
        self
    }

    pub fn add_handler(self, handler: H) -> Self {
        self.add_shared_handler(Arc::new(handler))
    }

    pub fn add_handlers(self, handlers: Vec<H>) -> Self {
        self.add_shared_handlers(handlers.into_iter().map(Arc::new).collect())
    }

    pub fn add_shared_handler(mut self, handler: Arc<H>) -> Self {
        self.handlers.push(handler);
        self
    }

    pub fn add_shared_handlers(mut self, handlers: Vec<Arc<H>>) -> Self {
        self.handlers.extend(handlers);
        self
    }

    pub fn add_listener(self, listener: L) -> Self {
        self.add_shared_listener(Arc::new(listener))
    }

    pub fn add_listeners(self, listener: Vec<L>) -> Self {
        self.add_shared_listeners(listener.into_iter().map(Arc::new).collect())
    }

    pub fn add_shared_listener(mut self, listener: Arc<L>) -> Self {
        self.listeners.push(listener);
        self
    }

    pub fn add_shared_listeners(mut self, listeners: Vec<Arc<L>>) -> Self {
        self.listeners.extend(listeners);
        self
    }

    pub fn build(self) -> Result<ChainDataLoader<R, F, V, H, L>> {
        if self.fetchers.is_empty() || self.validators.is_empty() || self.handlers.is_empty() {
            return Err(DataloaderError::LoaderInitError(
                "fetchers, validators and handlers cannot be empty".to_string(),
            ));
        }

        if let Some(config) = self.config {
            let state = ChainState {
                loaded_block: self.initial_block,
                is_running: false,
                is_loading: false,
                contract_states: HashMap::<String, ContractState>::new(),
            };
            Ok(ChainDataLoader {
                config,
                chain_id: self.chain_id,
                fetchers: self.fetchers,
                validators: self.validators,
                handlers: self.handlers,
                listeners: self.listeners,
                state: RwLock::new(state),
                _phantom: std::marker::PhantomData,
            })
        } else {
            Err(DataloaderError::LoaderInitError("config is required".to_string()))
        }
    }
}
