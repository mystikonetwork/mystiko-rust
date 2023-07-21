use crate::data::types::LoadedData;
use crate::fetcher::types::DataFetcher;
use crate::filter::ContractFilter;
use crate::handler::types::DataHandler;
use crate::validator::types::DataValidator;
use anyhow::{Error, Result};
use mystiko_config::wrapper::mystiko::MystikoConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};
use typed_builder::TypedBuilder;

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
    pub contract_filter: Option<Arc<Box<dyn ContractFilter>>>,
}

#[derive(Debug)]
pub struct ChainDataLoader<R, F = Box<dyn DataFetcher<R>>, V = Box<dyn DataValidator<R>>, H = Box<dyn DataHandler<R>>> {
    config: Arc<MystikoConfig>,
    chain_id: u64,
    initial_block: u64,
    fetchers: Vec<Arc<F>>,
    validators: Vec<Arc<V>>,
    handlers: Vec<Arc<H>>,
    state: RwLock<ChainState>,
    _phantom: std::marker::PhantomData<R>,
}

#[derive(Debug, Default)]
pub struct ChainDataLoaderBuilder<
    R,
    F = Box<dyn DataFetcher<R>>,
    V = Box<dyn DataValidator<R>>,
    H = Box<dyn DataHandler<R>>,
> {
    config: Option<Arc<MystikoConfig>>,
    chain_id: u64,
    initial_block: u64,
    fetchers: Vec<Arc<F>>,
    validators: Vec<Arc<V>>,
    handlers: Vec<Arc<H>>,
    _phantom: std::marker::PhantomData<R>,
}

impl<R, F, V, H> ChainDataLoader<R, F, V, H>
where
    R: LoadedData,
    F: DataFetcher<R>,
    V: DataValidator<R>,
    H: DataHandler<R>,
{
    pub fn builder() -> ChainDataLoaderBuilder<R, F, V, H> {
        ChainDataLoaderBuilder::new()
    }

    pub async fn start(&self, _options: &StartOption) -> Result<()> {
        todo!()
    }

    pub async fn stop(&self) -> Result<()> {
        todo!()
    }

    pub async fn state(&self) -> RwLockReadGuard<ChainState> {
        self.state.read().await
    }
}

impl<R, F, V, H> ChainDataLoaderBuilder<R, F, V, H>
where
    R: LoadedData,
    F: DataFetcher<R>,
    V: DataValidator<R>,
    H: DataHandler<R>,
{
    pub fn new() -> Self {
        Self {
            config: None,
            chain_id: 0,
            initial_block: 0,
            fetchers: Vec::new(),
            validators: Vec::new(),
            handlers: Vec::new(),
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

    pub fn build(self) -> Result<ChainDataLoader<R, F, V, H>> {
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
                initial_block: self.initial_block,
                fetchers: self.fetchers,
                validators: self.validators,
                handlers: self.handlers,
                state: RwLock::new(state),
                _phantom: std::marker::PhantomData,
            })
        } else {
            Err(Error::msg("config is required"))
        }
    }
}
