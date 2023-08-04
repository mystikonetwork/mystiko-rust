use crate::data::chain::ChainData;
use crate::data::result::UnwrappedChainResult;
use crate::data::types::LoadedData;
use crate::error::DataLoaderError;
use crate::error::DataLoaderError::LoaderRunError;
use crate::fetcher::types::{ChainFetchOption, ContractFetchOption, DataFetcher, FetchOption};
use crate::handler::types::{DataHandler, HandleOption};
use crate::loader::listener::{
    LoadEvent, LoadFailureEvent, LoadSuccessEvent, LoaderEvent, LoaderListener, StartEvent, StopEvent,
};
use crate::loader::types::{
    LoadOption, LoaderState, ScheduleOption, DEFAULT_DELAY_BLOCK, DEFAULT_SCHEDULE_INTERVAL_MS,
};
use crate::validator::types::{DataValidator, ValidateOption};
use ethers_providers::Middleware;
use log::{error, warn};
use mystiko_config::wrapper::chain::ChainConfig;
use mystiko_config::wrapper::contract::ContractConfig;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_ethers::provider::factory::Provider;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::sleep;

type Result<T> = anyhow::Result<T, DataLoaderError>;

#[derive(Debug)]
pub struct ChainDataLoader<
    R: 'static,
    F: 'static = Box<dyn DataFetcher<R>>,
    V: 'static = Box<dyn DataValidator<R>>,
    H: 'static = Box<dyn DataHandler<R>>,
    L: 'static = Box<dyn LoaderListener>,
> {
    executor: Arc<ChainDataLoaderExecutor<R, F, V, H, L>>,
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
    pub async fn schedule(&self, options: ScheduleOption) -> Result<Option<JoinHandle<()>>> {
        if !self.set_is_running(true).await {
            let join_handle = self.run(options).await;
            match join_handle {
                Ok(h) => Ok(Some(h)),
                Err(e) => {
                    self.set_is_running(false).await;
                    Err(e)
                }
            }
        } else {
            warn!("loader is already running");
            Ok(None)
        }
    }

    pub async fn load(&self, options: Option<LoadOption>) -> Result<()> {
        let chain_cfg = self.executor.build_start_param().await?;
        self.executor.try_load(&chain_cfg, &options).await
    }

    async fn run(&self, options: ScheduleOption) -> Result<JoinHandle<()>> {
        let chain_cfg = self.executor.build_start_param().await?;
        let executor = self.executor.clone();
        let join_handle = tokio::spawn(async move { executor.start(chain_cfg, options.clone()).await });
        Ok(join_handle)
    }

    pub async fn stop_schedule(&self) {
        self.set_is_running(false).await;
    }

    pub async fn is_running(&self) -> bool {
        self.executor.is_running().await
    }

    pub async fn is_loading(&self) -> bool {
        self.executor.is_loading().await
    }

    async fn set_is_running(&self, is_running: bool) -> bool {
        self.executor.set_is_running(is_running).await
    }
}

#[derive(Debug)]
struct ChainDataLoaderExecutor<
    R: 'static,
    F: 'static = Box<dyn DataFetcher<R>>,
    V: 'static = Box<dyn DataValidator<R>>,
    H: 'static = Box<dyn DataHandler<R>>,
    L: 'static = Box<dyn LoaderListener>,
> {
    config: Arc<MystikoConfig>,
    chain_id: u64,
    fetchers: Vec<Arc<F>>,
    validators: Vec<Arc<V>>,
    handler: Arc<H>,
    listeners: Vec<Arc<L>>,
    provider: Arc<Provider>,
    state: RwLock<LoaderState>,
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
    fetchers: Vec<Arc<F>>,
    validators: Vec<Arc<V>>,
    handler: Option<Arc<H>>,
    listeners: Vec<Arc<L>>,
    provider: Option<Arc<Provider>>,
    _phantom: std::marker::PhantomData<R>,
}

impl<R, F, V, H, L> ChainDataLoaderExecutor<R, F, V, H, L>
where
    R: LoadedData,
    F: DataFetcher<R>,
    V: DataValidator<R>,
    H: DataHandler<R>,
    L: LoaderListener,
{
    async fn build_start_param(&self) -> Result<ChainConfig> {
        let chain_cfg = self
            .config
            .find_chain(self.chain_id)
            .ok_or_else(|| DataLoaderError::UnsupportedChainError(self.chain_id))?
            .clone();
        Ok(chain_cfg)
    }

    async fn start(&self, chain_cfg: ChainConfig, options: ScheduleOption) {
        let schedule_interval_ms = options.schedule_interval_ms.unwrap_or(DEFAULT_SCHEDULE_INTERVAL_MS);
        self.emit_event(&LoaderEvent::StartEvent(StartEvent::builder().build()))
            .await;

        loop {
            if !self.is_running().await {
                break;
            }

            self.try_load(&chain_cfg, &options.load_option)
                .await
                .unwrap_or_else(|e| {
                    warn!("try load meet error {:?}", e);
                });

            if !self.is_running().await {
                break;
            }

            sleep(Duration::from_millis(schedule_interval_ms)).await;
        }

        self.emit_event(&LoaderEvent::StopEvent(StopEvent::builder().build()))
            .await;
    }

    async fn is_running(&self) -> bool {
        self.state.read().await.is_running
    }

    async fn is_loading(&self) -> bool {
        self.state.read().await.is_loading
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

    async fn try_load(&self, chain_cfg: &ChainConfig, options: &Option<LoadOption>) -> Result<()> {
        if !self.set_is_loading(true).await {
            let result = self.load(chain_cfg, options).await;
            self.set_is_loading(false).await;
            result
        } else {
            Err(DataLoaderError::LoaderRunError("loader is already loading".to_string()))
        }
    }

    async fn load(&self, chain_cfg: &ChainConfig, options: &Option<LoadOption>) -> Result<()> {
        let chain_start_block = self.build_chain_start_block(chain_cfg).await?;
        let chain_target_block = self.build_chain_target_block(chain_start_block, options).await?;
        self.emit_event(&LoaderEvent::LoadEvent(
            LoadEvent::builder()
                .start_block(chain_start_block)
                .target_block(chain_target_block)
                .build(),
        ))
        .await;

        let result = self
            .execute_load(chain_cfg, chain_start_block, chain_target_block)
            .await;
        let chain_loaded_block = self.build_chain_loaded_block(chain_cfg).await.unwrap_or_else(|e| {
            warn!("load build chain loaded block meet error {:?}", e);
            0_u64
        });

        match result {
            Ok(_) => {
                self.emit_event(&LoaderEvent::LoadSuccessEvent(
                    LoadSuccessEvent::builder()
                        .start_block(chain_start_block)
                        .loaded_block(chain_loaded_block)
                        .build(),
                ))
                .await;
            }
            Err(e) => {
                self.emit_event(&LoaderEvent::LoadFailureEvent(
                    LoadFailureEvent::builder()
                        .start_block(chain_start_block)
                        .loaded_block(chain_loaded_block)
                        .load_error(e)
                        .build(),
                ))
                .await;
            }
        }
        Ok(())
    }

    async fn execute_load(
        &self,
        chain_cfg: &ChainConfig,
        chain_start_block: u64,
        chain_target_block: u64,
    ) -> Result<()> {
        let (chain_filter, mut contracts) = self.build_loading_contracts(chain_cfg).await?;
        let contracts_option = if chain_filter { Some(contracts.clone()) } else { None };
        let chain_fetch_option = ChainFetchOption::builder()
            .config(self.config.clone())
            .chain_id(self.chain_id)
            .start_block(chain_start_block)
            .target_block(chain_target_block)
            .contracts(contracts_option)
            .build();

        let mut fetch_option = FetchOption::Chain(&chain_fetch_option);
        let mut contracts_fetch_option;

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

            let retry_fetch_options = self.build_retry_fetch_option(&mut contracts, chain_target_block).await;
            if !retry_fetch_options.is_empty() {
                contracts_fetch_option = retry_fetch_options;
                fetch_option = FetchOption::Contracts(&contracts_fetch_option);
            } else {
                return Ok(());
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

        // Log warnings for any contract errors
        unwrapped.contract_errors.iter().for_each(|error| {
            warn!("fetch contract {:?} meet error {:?}", error.address, error.source);
        });

        Ok(unwrapped.result)
    }

    async fn validate(&self, data: &mut ChainData<R>) -> Result<()> {
        if !data.contracts_data.is_empty() {
            let validator_option = ValidateOption::builder().config(self.config.clone()).build();
            for validator in &self.validators {
                let validate_result = validator.validate(data, &validator_option).await?;
                let unwrapped: UnwrappedChainResult<Vec<String>> = UnwrappedChainResult::from(validate_result);
                unwrapped.contract_errors.iter().for_each(|c| {
                    warn!("validate contract {:?} meet error {:?}", c.address, c.source);
                    data.contracts_data
                        .iter()
                        .position(|d| d.address == c.address)
                        .map(|i| data.contracts_data.remove(i));
                });
            }
        } else {
            warn!("validate contract data is empty")
        }
        Ok(())
    }

    async fn handle(&self, data: &mut ChainData<R>) -> Result<()> {
        if !data.contracts_data.is_empty() {
            let handler_option = HandleOption::builder().config(self.config.clone()).build();
            let handle_result = self.handler.handle(data, &handler_option).await?;
            let unwrapped: UnwrappedChainResult<Vec<String>> = UnwrappedChainResult::from(handle_result);
            unwrapped.contract_errors.iter().for_each(|c| {
                warn!("handle contract {:?} meet error {:?}", c.address, c.source);
                data.contracts_data
                    .iter()
                    .position(|d| d.address == c.address)
                    .map(|i| data.contracts_data.remove(i));
            });
        }
        Ok(())
    }

    async fn build_loading_contracts(&self, chain: &ChainConfig) -> Result<(bool, Vec<ContractConfig>)> {
        if let Some(contracts) = self.handler.loading_contracts(self.chain_id).await? {
            if !contracts.is_empty() {
                return Ok((true, contracts));
            }
        } else {
            let contracts = chain.contracts_with_disabled();
            if !contracts.is_empty() {
                return Ok((false, contracts));
            }
        }

        Err(DataLoaderError::LoaderRunError("contracts cannot be empty".to_string()))
    }

    async fn build_chain_start_block(&self, chain_cfg: &ChainConfig) -> Result<u64> {
        Ok(self.build_chain_loaded_block(chain_cfg).await? + 1)
    }

    async fn build_chain_loaded_block(&self, chain_cfg: &ChainConfig) -> Result<u64> {
        let start_block = self
            .handler
            .chain_loaded_block(self.chain_id)
            .await?
            .unwrap_or_else(|| chain_cfg.start_block());
        Ok(start_block)
    }

    async fn build_chain_target_block(&self, start_block: u64, option: &Option<LoadOption>) -> Result<u64> {
        let delay_block = match option {
            None => DEFAULT_DELAY_BLOCK,
            Some(o) => o.delay_block.unwrap_or(DEFAULT_DELAY_BLOCK),
        };

        let latest = self.provider.get_block_number().await?.as_u64();
        let latest = if latest > delay_block && latest - delay_block >= start_block {
            latest - delay_block
        } else {
            return Err(LoaderRunError("latest block too small".to_string()));
        };

        Ok(latest)
    }

    async fn build_contract_start_block(&self, contract: &ContractConfig) -> Result<u64> {
        let start_block = self
            .handler
            .contract_loaded_block(self.chain_id, contract.address())
            .await?
            .unwrap_or(contract.start_block())
            + 1;
        Ok(start_block)
    }

    async fn build_retry_fetch_option(
        &self,
        contracts: &mut Vec<ContractConfig>,
        chain_target_block: u64,
    ) -> Vec<ContractFetchOption> {
        let mut contract_fetch_options = Vec::new();
        for contract_cfg in contracts {
            if let Ok(contract_start_block) = self.build_contract_start_block(contract_cfg).await {
                // skip contract that already fetch to chain_target_block
                if contract_start_block > chain_target_block {
                    continue;
                }

                let fetch_option = ContractFetchOption::builder()
                    .config(self.config.clone())
                    .chain_id(self.chain_id)
                    .address(contract_cfg.address().to_string())
                    .start_block(contract_start_block)
                    .target_block(chain_target_block)
                    .build();
                contract_fetch_options.push(fetch_option);
            }
        }

        contract_fetch_options
    }

    async fn emit_event(&self, event: &LoaderEvent) {
        for listener in &self.listeners {
            listener.callback(event).await.unwrap_or_else(|e| {
                warn!("emit event meet error {:?}", e);
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
            fetchers: Vec::new(),
            validators: Vec::new(),
            handler: None,
            listeners: Vec::new(),
            provider: None,
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

    pub fn shared_provider(mut self, provider: Arc<Provider>) -> Self {
        self.provider = Some(provider);
        self
    }

    pub fn build(self) -> Result<ChainDataLoader<R, F, V, H, L>> {
        let config = self
            .config
            .ok_or_else(|| DataLoaderError::LoaderInitError("config cannot be None".to_string()))?;

        if self.fetchers.is_empty() {
            return Err(DataLoaderError::LoaderInitError("fetchers cannot be empty".to_string()));
        }

        let handler = self
            .handler
            .ok_or_else(|| DataLoaderError::LoaderInitError("handler cannot be None".to_string()))?;

        let provider = self
            .provider
            .ok_or_else(|| DataLoaderError::LoaderInitError("provider cannot be None".to_string()))?;

        let state = LoaderState {
            is_running: false,
            is_loading: false,
        };

        let executor = ChainDataLoaderExecutor {
            config,
            chain_id: self.chain_id,
            fetchers: self.fetchers,
            validators: self.validators,
            handler,
            listeners: self.listeners,
            provider,
            state: RwLock::new(state),
            _phantom: Default::default(),
        };

        Ok(ChainDataLoader {
            executor: Arc::new(executor),
            _phantom: std::marker::PhantomData,
        })
    }
}
