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
    LoaderState, StartOption, DEFAULT_DELAY_BLOCK, DEFAULT_LOAD_INTERVAL_MS, DEFAULT_MAX_BATCH_BLOCK,
};
use crate::validator::types::{DataValidator, ValidateOption};
use ethers_providers::Middleware;
use log::{error, warn};
use mystiko_config::wrapper::contract::ContractConfig;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_ethers::provider::factory::Provider;
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
        let contracts = self.build_loading_contracts().await?;
        let start_block = self.build_chain_loaded_block(contracts.as_slice()).await? + 1;

        if !self.set_is_running(true).await {
            self.emit_event(&LoaderEvent::StartEvent(
                StartEvent::builder().start_block(start_block).build(),
            ))
            .await;

            loop {
                if !self.is_running().await {
                    break;
                }

                self.load(&contracts, options).await.unwrap_or_else(|e| {
                    warn!("load meet error {:?}", e);
                });

                if !self.is_running().await {
                    break;
                }
                sleep(Duration::from_millis(load_interval_ms)).await;
            }

            let loaded_block = self
                .build_chain_loaded_block(contracts.as_slice())
                .await
                .unwrap_or_else(|e| {
                    warn!("start build chain loaded block error {:?}", e);
                    0_u64
                });
            self.emit_event(&LoaderEvent::StopEvent(
                StopEvent::builder().loaded_block(loaded_block).build(),
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

    async fn load(&self, contracts: &[ContractConfig], options: &StartOption) -> Result<()> {
        let chain_start_block = self.build_chain_loaded_block(contracts).await? + 1;
        let chain_target_block = self.build_chain_target_block(chain_start_block, options).await?;

        if !self.set_is_loading(true).await {
            self.emit_event(&LoaderEvent::LoadEvent(
                LoadEvent::builder()
                    .start_block(chain_start_block)
                    .target_block(chain_target_block)
                    .build(),
            ))
            .await;

            let result = self
                .execute_load(contracts, chain_start_block, chain_target_block, options)
                .await;
            let chain_loaded_block = self.build_chain_loaded_block(contracts).await.unwrap_or_else(|e| {
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

            self.set_is_loading(false).await;
        }

        Ok(())
    }

    async fn execute_load(
        &self,
        contracts: &[ContractConfig],
        chain_start_block: u64,
        chain_target_block: u64,
        options: &StartOption,
    ) -> Result<()> {
        // todo support None to load all contracts
        let chain_fetch_option = ChainFetchOption::builder()
            .config(self.config.clone())
            .chain_id(self.chain_id)
            .start_block(chain_start_block)
            .target_block(chain_target_block)
            .contracts(Some(contracts.to_vec()))
            .build();

        let mut fetch_option = FetchOption::Chain(&chain_fetch_option);
        let mut contracts_fetch_option;
        let mut retry_contracts = contracts.to_vec();

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

            match self
                .build_retry_fetch_option(
                    &mut chain_data,
                    &mut retry_contracts,
                    chain_start_block,
                    chain_target_block,
                    options,
                )
                .await
            {
                Some(retry_fetch_option) => {
                    contracts_fetch_option = retry_fetch_option;
                    fetch_option = FetchOption::Contracts(&contracts_fetch_option);
                }
                None => {
                    return Ok(());
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

    async fn build_loading_contracts(&self) -> Result<Vec<ContractConfig>> {
        if let Some(contracts) = self.handler.loading_contracts(self.chain_id).await? {
            if !contracts.is_empty() {
                return Ok(contracts);
            }
        } else if let Some(chain) = self.config.find_chain(self.chain_id) {
            let contracts = chain.contracts_with_disabled();
            if !contracts.is_empty() {
                return Ok(contracts);
            }
        }

        Err(DataLoaderError::LoaderRunError("contracts cannot be empty".to_string()))
    }

    async fn build_chain_loaded_block(&self, contracts: &[ContractConfig]) -> Result<u64> {
        let start_block = self
            .handler
            .chain_loaded_block(self.chain_id)
            .await?
            .unwrap_or_else(|| {
                contracts
                    .iter()
                    .map(|c| c.start_block())
                    .min()
                    .ok_or(DataLoaderError::LoaderRunError(
                        "minimum chain loaded block is zero".to_string(),
                    ))
                    .unwrap()
            });

        Ok(start_block)
    }

    async fn build_chain_target_block(&self, start_block: u64, option: &StartOption) -> Result<u64> {
        let delay_block = option.delay_block.unwrap_or(DEFAULT_DELAY_BLOCK);
        let latest = self.provider.get_block_number().await?.as_u64();
        let latest = if latest > delay_block {
            latest - delay_block
        } else {
            return Err(LoaderRunError("latest block too small".to_string()));
        };

        let max_batch_block = option.max_batch_block.unwrap_or(DEFAULT_MAX_BATCH_BLOCK);
        let target_block = if latest > start_block + max_batch_block {
            start_block + max_batch_block
        } else {
            latest
        };
        Ok(target_block)
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

    fn build_contract_target_block(&self, start_block: u64, target_block: u64, option: &StartOption) -> u64 {
        let max_batch_block = option.max_batch_block.unwrap_or(DEFAULT_MAX_BATCH_BLOCK);
        if target_block > start_block + max_batch_block {
            start_block + max_batch_block
        } else {
            target_block
        }
    }

    async fn build_retry_fetch_option(
        &self,
        data: &mut ChainData<R>,
        retry_contracts: &mut Vec<ContractConfig>,
        chain_start_block: u64,
        chain_target_block: u64,
        start_option: &StartOption,
    ) -> Option<Vec<ContractFetchOption>> {
        // Removing contracts that already fetch to chain_target_block
        retry_contracts.retain(|r| {
            !data
                .contracts_data
                .iter()
                .any(|d| r.address() == d.address && d.end_block >= chain_target_block)
        });

        if !retry_contracts.is_empty() {
            let mut contract_fetch_options = Vec::new();

            for contract_cfg in retry_contracts {
                if let Ok(contract_start_block) = self.build_contract_start_block(contract_cfg).await {
                    if contract_start_block < chain_start_block {
                        error!(
                            "contract start block {} less than chain start block {}",
                            contract_start_block, chain_start_block
                        );
                    }

                    // skip contract that already fetch to chain_target_block
                    if contract_start_block > chain_target_block {
                        continue;
                    }

                    let contract_target_block =
                        self.build_contract_target_block(contract_start_block, chain_target_block, start_option);
                    let fetch_option = ContractFetchOption::builder()
                        .config(self.config.clone())
                        .chain_id(self.chain_id)
                        .address(contract_cfg.address().to_string())
                        .start_block(contract_start_block)
                        .target_block(contract_target_block)
                        .build();
                    contract_fetch_options.push(fetch_option);
                }
            }

            Some(contract_fetch_options)
        } else {
            None
        }
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

        Ok(ChainDataLoader {
            config,
            chain_id: self.chain_id,
            fetchers: self.fetchers,
            validators: self.validators,
            handler,
            listeners: self.listeners,
            provider,
            state: RwLock::new(state),
            _phantom: std::marker::PhantomData,
        })
    }
}
