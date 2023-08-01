use crate::data::chain::ChainData;
use crate::data::result::UnwrappedChainResult;
use crate::data::types::LoadedData;
use crate::error::DataLoaderError;
use crate::fetcher::types::{ChainFetchOption, ContractFetchOption, DataFetcher, FetchOption};
use crate::filter::ContractFilter;
use crate::handler::types::{DataHandler, HandleOption};
use crate::loader::listener::{
    LoadEvent, LoadFailureEvent, LoadSuccessEvent, LoaderEvent, LoaderListener, StartEvent, StopEvent,
};
use crate::loader::types::{ChainState, ContractState, StartOption, DEFAULT_LOAD_INTERVAL_MS, DEFAULT_MAX_BATCH_BLOCK};
use crate::validator::types::{DataValidator, ValidateOption};
use log::{error, warn};
use mystiko_config::wrapper::mystiko::MystikoConfig;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;

type Result<T> = anyhow::Result<T, DataLoaderError>;

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
    handler: Arc<H>,
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
    handler: Option<Arc<H>>,
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
        let load_interval_ms = options.load_interval_ms.unwrap_or(DEFAULT_LOAD_INTERVAL_MS);

        if !self.set_is_running(true).await {
            self.merge_contract_states(options.contract_filter.clone()).await?;
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
            warn!("loader is already running");
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

    async fn contracts(&self) -> HashMap<String, ContractState> {
        self.state.read().await.contract_states.clone()
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
                Ok(loaded_block) => {
                    self.emit_event(&LoaderEvent::LoadSuccessEvent(
                        LoadSuccessEvent::builder().end_block(loaded_block).build(),
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

    async fn execute_load(&self, start_block: u64, options: &StartOption) -> Result<u64> {
        let end_block = start_block + options.max_batch_block.unwrap_or(DEFAULT_MAX_BATCH_BLOCK);
        let chain_fetch_option = ChainFetchOption::builder()
            .config(self.config.clone())
            .chain_id(self.chain_id)
            .start_block(start_block)
            .end_block(end_block)
            .contract_filter(options.contract_filter.clone())
            .build();

        // todo filter state contract end_block > end_block?
        let mut fetch_option = FetchOption::Chain(&chain_fetch_option);
        let mut contracts_fetch_option;
        let mut retry_contracts = self.contracts().await;
        for fetcher in &self.fetchers {
            let mut chain_data = match self.fetch(fetcher, fetch_option.clone()).await {
                Err(e) => {
                    warn!("fetch meet error {:?}, try next fetcher", e);
                    continue;
                }
                Ok(d) => d,
            };

            if let Err(e) = self.validate(&mut chain_data).await {
                warn!("validate meet error {:?}, try next fetcher", e);
                continue;
            };

            if let Err(e) = self.handle(&mut chain_data).await {
                warn!("handle meet error {:?}, try next fetcher", e);
                continue;
            };

            let loaded_block = self.update_state(&chain_data).await;
            match self.build_retry_fetch_option(&mut chain_data, &mut retry_contracts, start_block, end_block) {
                Some(option) => {
                    contracts_fetch_option = option;
                    fetch_option = FetchOption::Contracts(&contracts_fetch_option);
                }
                None => {
                    return Ok(loaded_block);
                }
            };
        }

        error!("failed fetch from all fetchers");
        Err(DataLoaderError::LoaderRunError(
            "failed fetch from all fetchers".to_string(),
        ))
    }

    async fn fetch(&self, fetcher: &Arc<F>, option: FetchOption<'_>) -> Result<ChainData<R>> {
        let fetch_result = match fetcher.fetch(&option).await {
            Err(e) => {
                return Err(DataLoaderError::AnyhowError(e));
            }
            Ok(chain_data) => chain_data,
        };

        let unwrapped = UnwrappedChainResult::from(fetch_result);
        unwrapped.log_warning("fetch");
        Ok(unwrapped.result)
    }

    async fn validate(&self, data: &mut ChainData<R>) -> Result<()> {
        if !data.contracts_data.is_empty() {
            let validator_option = ValidateOption::builder().config(self.config.clone()).build();
            for validator in &self.validators {
                let validate_result = validator.validate(data, &validator_option).await?;
                let unwrapped: UnwrappedChainResult<Vec<String>> = UnwrappedChainResult::from(validate_result);
                unwrapped.log_warning("validate");

                let _ = unwrapped.contract_errors.iter().map(|c| {
                    data.contracts_data
                        .iter()
                        .position(|d| d.address == c.address)
                        .map(|i| data.contracts_data.remove(i));
                });
            }
        }
        Ok(())
    }

    async fn handle(&self, data: &mut ChainData<R>) -> Result<()> {
        if !data.contracts_data.is_empty() {
            let handler_option = HandleOption::builder().config(self.config.clone()).build();
            let handle_result = self.handler.handle(data, &handler_option).await?;
            let unwrapped: UnwrappedChainResult<Vec<String>> = UnwrappedChainResult::from(handle_result);
            unwrapped.log_warning("handle");

            let _ = unwrapped.contract_errors.iter().map(|c| {
                data.contracts_data
                    .iter()
                    .position(|d| d.address == c.address)
                    .map(|i| data.contracts_data.remove(i));
            });
        }
        Ok(())
    }

    fn build_retry_fetch_option(
        &self,
        data: &mut ChainData<R>,
        retry_contracts: &mut HashMap<String, ContractState>,
        start_block: u64,
        end_block: u64,
    ) -> Option<Vec<ContractFetchOption>> {
        data.contracts_data.iter().for_each(|d| {
            retry_contracts.remove(&d.address);
        });

        // todo filter state contract end_block > end_block?
        if !retry_contracts.is_empty() {
            let contract_fetch_options = retry_contracts
                .keys()
                .map(|key| {
                    ContractFetchOption::builder()
                        .config(self.config.clone())
                        .chain_id(self.chain_id)
                        .address(key.clone())
                        .start_block(start_block)
                        .end_block(end_block)
                        .build()
                })
                .collect::<Vec<_>>();
            Some(contract_fetch_options)
        } else {
            None
        }
    }

    async fn update_state(&self, chain_data: &ChainData<R>) -> u64 {
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

            state.loaded_block
        } else {
            self.loaded_block().await
        }
    }

    async fn merge_contract_states(&self, filter: Option<Arc<Box<dyn ContractFilter>>>) -> Result<()> {
        let init_states = build_contract_states(self.chain_id, self.config.clone(), filter)?;
        let mut current_states = self.state.write().await;
        init_states.iter().for_each(|(addr, init_state)| {
            if !current_states.contract_states.contains_key(addr) {
                if current_states.loaded_block > init_state.loaded_block {
                    current_states.loaded_block = init_state.loaded_block;
                }
                current_states.contract_states.insert(addr.clone(), init_state.clone());
            }
        });
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
            handler: None,
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

    pub fn handler(self, handler: H) -> Self {
        self.shared_handler(Arc::new(handler))
    }

    pub fn shared_handler(mut self, handler: Arc<H>) -> Self {
        self.handler = Some(handler);
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
        if self.fetchers.is_empty() || self.validators.is_empty() || self.handler.is_none() {
            return Err(DataLoaderError::LoaderInitError(
                "fetchers, validators cannot be empty".to_string(),
            ));
        }

        let handle = self
            .handler
            .ok_or_else(|| DataLoaderError::LoaderInitError("handler cannot be None".to_string()))?;

        if let Some(config) = self.config {
            let state = ChainState {
                loaded_block: self.initial_block,
                is_running: false,
                is_loading: false,
                contract_states: HashMap::new(),
            };
            Ok(ChainDataLoader {
                config,
                chain_id: self.chain_id,
                fetchers: self.fetchers,
                validators: self.validators,
                handler: handle,
                listeners: self.listeners,
                state: RwLock::new(state),
                _phantom: std::marker::PhantomData,
            })
        } else {
            Err(DataLoaderError::LoaderInitError("config is required".to_string()))
        }
    }
}

fn build_contract_states(
    chain_id: u64,
    cfg: Arc<MystikoConfig>,
    filter: Option<Arc<Box<dyn ContractFilter>>>,
) -> Result<HashMap<String, ContractState>> {
    match cfg.find_chain(chain_id) {
        None => Err(DataLoaderError::LoaderInitError("chain not exist".to_string())),
        Some(c) => {
            let mut h = HashMap::new();
            c.contracts_with_disabled().iter().for_each(|contract| {
                let should_filter = match &filter {
                    Some(f) => f.filter(chain_id, contract),
                    None => false,
                };
                if !should_filter {
                    h.insert(
                        contract.address().to_string(),
                        ContractState::builder().loaded_block(contract.start_block()).build(),
                    );
                }
            });
            Ok(h)
        }
    }
}
