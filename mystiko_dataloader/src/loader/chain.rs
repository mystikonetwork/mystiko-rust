use crate::data::ChainData;
use crate::data::LoadedData;
use crate::data::UnwrappedChainResult;
use crate::error::DataLoaderError;
use crate::fetcher::{ContractFetchOptions, DataFetcher, FetchOptions};
use crate::handler::{DataHandler, HandleOption};
use crate::loader::{DataLoader, DataLoaderResult, LoadOption, DEFAULT_DELAY_BLOCK, DEFAULT_VALIDATOR_CONCURRENCY};
use crate::validator::{DataValidator, ValidateOption};
use async_trait::async_trait;
use ethers_providers::Middleware;
use log::{error, warn};
use mystiko_config::wrapper::chain::ChainConfig;
use mystiko_config::wrapper::contract::ContractConfig;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_ethers::provider::factory::Provider;
use std::any::type_name;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, Default)]
pub struct ChainDataLoaderBuilder<
    R,
    F = Box<dyn DataFetcher<R>>,
    V = Box<dyn DataValidator<R>>,
    H = Box<dyn DataHandler<R>>,
> {
    config: Option<Arc<MystikoConfig>>,
    chain_id: u64,
    fetchers: Vec<Arc<F>>,
    validators: Vec<Arc<V>>,
    handler: Option<Arc<H>>,
    provider: Option<Arc<Provider>>,
    _phantom: std::marker::PhantomData<R>,
}

#[derive(Debug)]
pub struct ChainDataLoader<R, F = Box<dyn DataFetcher<R>>, V = Box<dyn DataValidator<R>>, H = Box<dyn DataHandler<R>>> {
    config: Arc<MystikoConfig>,
    chain_id: u64,
    fetchers: Vec<Arc<F>>,
    validators: Vec<Arc<V>>,
    handler: Arc<H>,
    provider: Arc<Provider>,
    _phantom: std::marker::PhantomData<R>,
}

#[derive(Debug, Clone, TypedBuilder)]
struct ChainLoadParams<'a> {
    pub cfg: &'a ChainConfig,
    pub start_block: u64,
    pub target_block: u64,
    pub option: &'a Option<LoadOption>,
}

#[async_trait]
impl<R, F, V, H> DataLoader for ChainDataLoader<R, F, V, H>
where
    R: 'static + LoadedData,
    F: 'static + DataFetcher<R>,
    V: 'static + DataValidator<R>,
    H: 'static + DataHandler<R>,
{
    async fn load(&self, options: Option<LoadOption>) -> DataLoaderResult<()> {
        let chain_cfg = self.build_chain_config().await?;
        let chain_start_block = self.build_chain_start_block(&chain_cfg).await?;
        let chain_target_block = self.build_chain_target_block(chain_start_block, &options).await?;
        let params = ChainLoadParams::builder()
            .cfg(&chain_cfg)
            .start_block(chain_start_block)
            .target_block(chain_target_block)
            .option(&options)
            .build();

        self.try_load(&params).await
    }
}

impl<R, F, V, H> ChainDataLoaderBuilder<R, F, V, H>
where
    R: 'static + LoadedData,
    F: 'static + DataFetcher<R>,
    V: 'static + DataValidator<R>,
    H: 'static + DataHandler<R>,
{
    pub fn new() -> Self {
        Self {
            config: None,
            chain_id: 0,
            fetchers: Vec::new(),
            validators: Vec::new(),
            handler: None,
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

    pub fn shared_provider(mut self, provider: Arc<Provider>) -> Self {
        self.provider = Some(provider);
        self
    }

    pub fn build(self) -> DataLoaderResult<ChainDataLoader<R, F, V, H>> {
        let config = self
            .config
            .ok_or_else(|| DataLoaderError::LoaderBuildError("config cannot be None".to_string()))?;

        if self.fetchers.is_empty() {
            return Err(DataLoaderError::LoaderBuildError(
                "fetchers cannot be empty".to_string(),
            ));
        }

        let handler = self
            .handler
            .ok_or_else(|| DataLoaderError::LoaderBuildError("handler cannot be None".to_string()))?;

        let provider = self
            .provider
            .ok_or_else(|| DataLoaderError::LoaderBuildError("provider cannot be None".to_string()))?;

        Ok(ChainDataLoader {
            config,
            chain_id: self.chain_id,
            fetchers: self.fetchers,
            validators: self.validators,
            handler,
            provider,
            _phantom: Default::default(),
        })
    }
}

impl<R, F, V, H> ChainDataLoader<R, F, V, H>
where
    R: 'static + LoadedData,
    F: 'static + DataFetcher<R>,
    V: 'static + DataValidator<R>,
    H: 'static + DataHandler<R>,
{
    async fn try_load(&self, params: &ChainLoadParams<'_>) -> DataLoaderResult<()> {
        let (chain_filter, contracts) = self.build_loading_contracts(params.cfg).await?;
        let contract_options = if chain_filter {
            Some(
                contracts
                    .iter()
                    .map(|c| ContractFetchOptions::builder().contract_config(c.clone()).build())
                    .collect(),
            )
        } else {
            None
        };

        let mut fetch_option = FetchOptions::builder()
            .config(self.config.clone())
            .chain_id(self.chain_id)
            .start_block(params.start_block)
            .target_block(params.target_block)
            .contract_options(contract_options)
            .build();

        for (index, fetcher) in self.fetchers.iter().enumerate() {
            let mut chain_data = match self.fetch(fetcher, &fetch_option).await {
                Err(e) => {
                    warn!(
                        "fetcher(index={:?}, name={:?}) raised error {:?}, try next fetcher",
                        index,
                        type_name_of_object::<F>(),
                        e
                    );
                    continue;
                }
                Ok(d) => d,
            };

            if let Err(e) = self.validate(&mut chain_data, params.option).await {
                warn!(
                    "validate fetcher(index={:?}, name={:?}) data raised error {:?}, try next fetcher",
                    index,
                    type_name_of_object::<F>(),
                    e
                );
                continue;
            };

            if let Err(e) = self.handle(&chain_data).await {
                warn!(
                    "handle fetcher(index={:?}, name={:?}) data raised error {:?}, try next fetcher",
                    index,
                    type_name_of_object::<F>(),
                    e
                );

                continue;
            };

            let retry_fetch_options = self
                .build_retry_fetch_options(&contracts, params.start_block, params.target_block)
                .await;
            match retry_fetch_options {
                Some(o) => fetch_option = o,
                None => return Ok(()),
            }
        }

        error!("failed to fetch data from all fetchers");
        Err(DataLoaderError::LoaderFetchersExhaustedError)
    }

    async fn fetch(&self, fetcher: &Arc<F>, option: &FetchOptions) -> DataLoaderResult<ChainData<R>> {
        let fetch_result = fetcher.fetch(option).await?;
        let unwrapped = UnwrappedChainResult::from(fetch_result);

        // Log warnings for any contract errors
        unwrapped.contract_errors.iter().for_each(|error| {
            warn!("fetch contract {:?} raised error {:?}", error.address, error.source);
        });

        Ok(unwrapped.result)
    }

    async fn validate(&self, data: &mut ChainData<R>, options: &Option<LoadOption>) -> DataLoaderResult<()> {
        if !data.contracts_data.is_empty() {
            let validator_option = ValidateOption::builder()
                .config(self.config.clone())
                .validate_concurrency(
                    options
                        .as_ref()
                        .map(|o| o.validator_concurrency)
                        .unwrap_or(DEFAULT_VALIDATOR_CONCURRENCY),
                )
                .build();
            for (index, validator) in self.validators.iter().enumerate() {
                let validate_result = validator.validate(data, &validator_option).await?;
                let unwrapped: UnwrappedChainResult<Vec<String>> = UnwrappedChainResult::from(validate_result);
                unwrapped.contract_errors.iter().for_each(|c| {
                    warn!(
                        "validator(index={:?}, name={:?}) contract {:?} raised error {:?}",
                        index,
                        type_name_of_object::<R>(),
                        c.address,
                        c.source
                    );
                    data.contracts_data
                        .iter()
                        .position(|d| d.address == c.address)
                        .map(|i| data.contracts_data.remove(i));
                });
            }
            Ok(())
        } else {
            warn!("fetcher contract data is empty");
            Err(DataLoaderError::LoaderEmptyValidateDataError)
        }
    }

    async fn handle(&self, data: &ChainData<R>) -> DataLoaderResult<()> {
        if !data.contracts_data.is_empty() {
            let handler_option = HandleOption::builder().config(self.config.clone()).build();
            let handle_result = self.handler.handle(data, &handler_option).await?;
            let unwrapped: UnwrappedChainResult<Vec<String>> = UnwrappedChainResult::from(handle_result);
            unwrapped.contract_errors.iter().for_each(|c| {
                warn!("handle contract {:?} raised error {:?}", c.address, c.source);
            });
            Ok(())
        } else {
            warn!("fetcher contract data all invalid");
            Err(DataLoaderError::LoaderEmptyHandleDataError)
        }
    }

    async fn build_chain_config(&self) -> DataLoaderResult<ChainConfig> {
        let chain_cfg = self
            .config
            .find_chain(self.chain_id)
            .ok_or_else(|| DataLoaderError::UnsupportedChainError(self.chain_id))?
            .clone();
        Ok(chain_cfg)
    }

    async fn build_loading_contracts(&self, chain: &ChainConfig) -> DataLoaderResult<(bool, Vec<ContractConfig>)> {
        if let Some(contracts) = self.handler.query_loading_contracts(self.chain_id).await? {
            if !contracts.is_empty() {
                return Ok((true, contracts));
            }
        } else {
            let contracts = chain.contracts_with_disabled();
            if !contracts.is_empty() {
                return Ok((false, contracts));
            }
        }

        Err(DataLoaderError::LoaderNoContractsError)
    }

    async fn build_chain_start_block(&self, chain_cfg: &ChainConfig) -> DataLoaderResult<u64> {
        Ok(self.build_chain_loaded_block(chain_cfg).await? + 1)
    }

    async fn build_chain_loaded_block(&self, chain_cfg: &ChainConfig) -> DataLoaderResult<u64> {
        let start_block = self
            .handler
            .query_chain_loaded_block(self.chain_id)
            .await?
            .unwrap_or_else(|| chain_cfg.start_block());
        Ok(start_block)
    }

    async fn build_chain_target_block(&self, start_block: u64, options: &Option<LoadOption>) -> DataLoaderResult<u64> {
        let delay_block = match options {
            None => DEFAULT_DELAY_BLOCK,
            Some(o) => o.delay_block.unwrap_or(DEFAULT_DELAY_BLOCK),
        };

        let latest = self.provider.get_block_number().await?.as_u64();
        if latest > delay_block && latest - delay_block >= start_block {
            Ok(latest - delay_block)
        } else {
            Err(DataLoaderError::LoaderLoadError("latest block too small".to_string()))
        }
    }

    async fn build_contract_start_block(&self, contract: &ContractConfig) -> DataLoaderResult<u64> {
        let start_block = self
            .handler
            .query_contract_loaded_block(self.chain_id, contract.address())
            .await?
            .unwrap_or(contract.start_block())
            + 1;
        Ok(start_block)
    }

    async fn build_retry_fetch_options(
        &self,
        contracts: &Vec<ContractConfig>,
        chain_start_block: u64,
        chain_target_block: u64,
    ) -> Option<FetchOptions> {
        let mut contract_options: Vec<ContractFetchOptions> = vec![];
        for contract_cfg in contracts {
            if let Ok(contract_start_block) = self.build_contract_start_block(contract_cfg).await {
                // skip contract that already fetch to chain_target_block
                if contract_start_block > chain_target_block {
                    continue;
                }

                let fetch_option = ContractFetchOptions::builder()
                    .contract_config(contract_cfg.clone())
                    .start_block(Some(contract_start_block))
                    .target_block(Some(chain_target_block))
                    .build();
                contract_options.push(fetch_option);
            }
        }

        if contract_options.is_empty() {
            None
        } else {
            let options = FetchOptions::builder()
                .config(self.config.clone())
                .chain_id(self.chain_id)
                .start_block(chain_start_block)
                .target_block(chain_target_block)
                .contract_options(Some(contract_options))
                .build();
            Some(options)
        }
    }
}

fn type_name_of_object<T>() -> &'static str {
    let fully_qualified_name = type_name::<T>();
    let struct_name = fully_qualified_name.split('<').next();
    let name = match struct_name {
        Some(n) => n.rsplit("::").next(),
        None => fully_qualified_name.rsplit("::").next(),
    };
    name.unwrap_or(fully_qualified_name)
}
