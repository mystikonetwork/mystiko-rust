use crate::data::raw::RawData;
use crate::fetcher::types::DataFetcher;
use crate::filter::ContractFilter;
use crate::handler::types::DataHandler;
use crate::validator::types::DataValidator;
use anyhow::{Error, Result};
use mystiko_config::wrapper::mystiko::MystikoConfig;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct ChainDataLoaderState {
    pub loaded_block: u64,
    pub is_running: bool,
    pub is_loading: bool,
}

#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct ChainDataLoaderStartOption {
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
    state: RwLock<ChainDataLoaderState>,
    _phantom: std::marker::PhantomData<R>,
}

#[derive(Debug)]
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
    R: RawData,
    F: DataFetcher<R>,
    V: DataValidator<R>,
    H: DataHandler<R>,
{
    pub fn builder() -> ChainDataLoaderBuilder<R, F, V, H> {
        ChainDataLoaderBuilder::new()
    }

    pub async fn start(&mut self, _options: &ChainDataLoaderStartOption) -> Result<()> {
        todo!()
    }

    pub async fn stop(&mut self) -> Result<()> {
        todo!()
    }

    pub async fn state(&self) -> RwLockReadGuard<ChainDataLoaderState> {
        self.state.read().await
    }
}

impl<R, F, V, H> ChainDataLoaderBuilder<R, F, V, H>
where
    R: RawData,
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

    pub fn add_fetcher(mut self, fetcher: F) -> Self {
        self.fetchers.push(Arc::new(fetcher));
        self
    }

    pub fn add_fetchers(mut self, fetchers: Vec<F>) -> Self {
        self.fetchers.extend(fetchers.into_iter().map(|f| Arc::new(f)));
        self
    }

    pub fn add_validator(mut self, validator: V) -> Self {
        self.validators.push(Arc::new(validator));
        self
    }

    pub fn add_validators(mut self, validators: Vec<V>) -> Self {
        self.validators.extend(validators.into_iter().map(|v| Arc::new(v)));
        self
    }

    pub fn add_handler(mut self, handler: H) -> Self {
        self.handlers.push(Arc::new(handler));
        self
    }

    pub fn add_handlers(mut self, handlers: Vec<H>) -> Self {
        self.handlers.extend(handlers.into_iter().map(|h| Arc::new(h)));
        self
    }

    pub fn build(self) -> Result<ChainDataLoader<R, F, V, H>> {
        if let Some(config) = self.config {
            let state = ChainDataLoaderState {
                loaded_block: self.initial_block,
                is_running: false,
                is_loading: false,
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
