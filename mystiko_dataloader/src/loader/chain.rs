use crate::data::chain::ChainData;
use crate::data::types::LoadedData;
use crate::error::{DataloaderError, Result};
use crate::fetcher::types::{ChainFetchOption, DataFetcher};
use crate::filter::ContractFilter;
use crate::handler::types::{DataHandler, HandleOption};
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
    async fn update_state(&self, chain_data: &ChainData<R>) -> Result<()> {
        if chain_data.contracts_data.is_empty() {
            return Ok(());
        }

        let mut state = self.state.write().await;
        let mut min_end_block: u64 = chain_data.contracts_data[0].end_block;
        for contract_data in &chain_data.contracts_data {
            min_end_block = min_end_block.min(contract_data.end_block);

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
        }

        state.loaded_block = min_end_block;
        Ok(())
    }

    async fn run(&self, options: &StartOption) -> Result<()> {
        let start_block = self.state.read().await.loaded_block;
        let fetch_option = ChainFetchOption {
            config: Arc::clone(&self.config),
            chain_id: self.chain_id,
            start_block,
            end_block: start_block + options.max_batch_block.unwrap_or(1000000),
            contract_filter: options.contract_filter.clone(),
        };

        let validator_option = ValidateOption {
            config: Arc::clone(&self.config),
        };

        let handler_option = HandleOption {
            config: Arc::clone(&self.config),
        };

        for fetcher in &self.fetchers {
            let chain_data = match fetcher.fetch_chain(&fetch_option).await {
                Err(_) => continue,
                Ok(chain_data) => chain_data,
            };

            // todo refetch failed contract?
            let validate_data = ChainData {
                chain_id: self.chain_id,
                contracts_data: chain_data
                    .into_iter()
                    .filter_map(|r| match r {
                        Ok(data) => Some(data),
                        Err(e) => {
                            error!("fetch meet error {:?}", e);
                            None
                        }
                    })
                    .collect(),
            };

            for validator in &self.validators {
                let result = validator.validate(&validate_data, &validator_option).await;
                if let Err(_) | Ok(false) = result {
                    break;
                }

                for handler in &self.handlers {
                    handler.handle(&validate_data, &handler_option).await?;
                }

                return self.update_state(&validate_data).await;
            }
        }

        Err(DataloaderError::LoaderRunError(
            "failed fetch from all fetchers".to_string(),
        ))
    }

    pub async fn start(&self, options: &StartOption) -> Result<()> {
        let load_interval_ms = options.load_interval_ms.unwrap_or(10000);
        self.state.write().await.loaded_block = self.initial_block;
        self.state.write().await.is_loading = true;
        loop {
            if !self.state.read().await.is_loading {
                break;
            }

            self.state.write().await.is_running = true;
            self.run(options).await.unwrap_or_else(|e| {
                error!("run error: {:?}", e);
            });
            self.state.write().await.is_running = false;

            if !self.state.read().await.is_loading {
                break;
            }
            sleep(Duration::from_millis(load_interval_ms)).await;
        }

        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        self.state.write().await.is_loading = false;
        Ok(())
    }

    pub async fn state(&self) -> ChainState {
        self.state.read().await.clone()
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
                initial_block: self.initial_block,
                fetchers: self.fetchers,
                validators: self.validators,
                handlers: self.handlers,
                state: RwLock::new(state),
                _phantom: std::marker::PhantomData,
            })
        } else {
            Err(DataloaderError::LoaderInitError("config is required".to_string()))
        }
    }
}
