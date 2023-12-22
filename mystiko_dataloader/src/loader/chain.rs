use crate::data::ChainData;
use crate::data::LoadedData;
use crate::data::UnwrappedChainResult;
use crate::error::DataLoaderError;
use crate::fetcher::{ChainLoadedBlockOptions, ContractFetchOptions, DataFetcher, FetchOptions, FetcherOptions};
use crate::handler::{DataHandler, HandleOption};
use crate::loader::{
    DataLoader, DataLoaderConfigResult, DataLoaderResult, FromConfig, LoadFetcherOption, LoadOption, LoadStatus,
    LoadValidatorOption, LoaderConfigOptions, ResetOptions,
};
use crate::validator::{DataValidator, ValidateOption};
use async_trait::async_trait;
use mystiko_config::{ChainConfig, ContractConfig, MystikoConfig};
use std::cmp::max;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ChainDataLoader<R, H = Box<dyn DataHandler<R>>, F = Box<dyn DataFetcher<R>>, V = Box<dyn DataValidator<R>>> {
    config: Arc<MystikoConfig>,
    chain_id: u64,
    #[builder(default)]
    fetchers: Vec<FetcherWrapper<R, F>>,
    #[builder(default)]
    validators: Vec<Arc<V>>,
    handler: Arc<H>,
    #[builder(default)]
    _phantom: std::marker::PhantomData<R>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FetcherWrapper<R, F = Box<dyn DataFetcher<R>>> {
    pub(crate) fetcher: Arc<F>,
    #[builder(default)]
    pub(crate) options: FetcherOptions,
    #[builder(default)]
    _phantom: std::marker::PhantomData<R>,
}

#[derive(Debug, Clone, TypedBuilder)]
struct ChainLoadParams<'a> {
    pub(crate) cfg: &'a ChainConfig,
    pub(crate) option: &'a LoadOption,
}

#[derive(Debug, Clone, TypedBuilder)]
struct FetcherRunParams<F> {
    pub(crate) fetcher: Arc<F>,
    pub(crate) options: FetcherOptions,
    pub(crate) loaded_block: u64,
}

#[derive(Debug, Clone, TypedBuilder)]
struct ValidatorRunParams<V> {
    pub(crate) validator: Arc<V>,
}

#[derive(Debug, Clone, TypedBuilder)]
struct LoaderRunParams<'a, F, V> {
    pub(crate) params: &'a ChainLoadParams<'a>,
    pub(crate) fetchers: Vec<FetcherRunParams<F>>,
    pub(crate) validators: Vec<ValidatorRunParams<V>>,
}

#[async_trait]
impl<R, H, F, V> DataLoader for ChainDataLoader<R, H, F, V>
where
    R: LoadedData,
    H: DataHandler<R>,
    F: DataFetcher<R>,
    V: DataValidator<R>,
{
    async fn chain_loaded_block(&self, chain_id: u64) -> DataLoaderResult<Option<u64>> {
        Ok(self.handler.query_chain_loaded_block(chain_id).await?)
    }

    async fn contract_loaded_block(&self, chain_id: u64, contract_address: &str) -> DataLoaderResult<Option<u64>> {
        Ok(self
            .handler
            .query_contract_loaded_block(chain_id, contract_address)
            .await?)
    }

    async fn chain_target_block(&self, _chain_id: u64) -> DataLoaderResult<Option<u64>> {
        let mut fetchers = self.fetchers.iter().collect::<Vec<_>>();
        fetchers.sort_by_key(|f| f.options.target_block_priority);
        let load_options = LoadFetcherOption::builder().build();
        for fetcher in fetchers.iter().rev() {
            if let Ok(target_block) = self.query_loaded_blocks(fetcher, &load_options).await {
                return Ok(Some(target_block.loaded_block));
            }
        }
        Ok(None)
    }

    async fn load<O>(&self, options: O) -> DataLoaderResult<LoadStatus>
    where
        O: Into<LoadOption> + Send + Sync,
    {
        let options = options.into();
        let chain_cfg = self.build_chain_config().await?;
        let params = ChainLoadParams::builder().cfg(&chain_cfg).option(&options).build();

        self.try_load(&params).await
    }

    async fn reset(&self, options: ResetOptions) -> DataLoaderResult<()> {
        let _ = self.handler.reset(&options).await?;
        Ok(())
    }
}

#[async_trait]
impl<R, H> FromConfig<LoaderConfigOptions<R, H>> for ChainDataLoader<R, H>
where
    R: LoadedData + 'static,
    H: DataHandler<R> + 'static,
{
    async fn from_config(options: &LoaderConfigOptions<R, H>) -> DataLoaderConfigResult<Self> {
        options.validate_config()?;

        let mystiko_config = options.build_mystiko_config().await?;
        let providers = match &options.providers {
            None => options.build_providers(mystiko_config.clone())?,
            Some(p) => p.clone(),
        };

        let fetchers = if options.fetchers.is_empty() {
            options
                .build_fetchers(options.chain_id, mystiko_config.clone(), providers.clone())
                .await?
        } else {
            options
                .fetchers
                .iter()
                .map(|f| FetcherWrapper::builder().fetcher(f.clone()).build())
                .collect()
        };

        let validators = if options.validators.is_empty() {
            options.build_validators(providers.clone(), options.handler.clone())?
        } else {
            options.validators.clone()
        };

        let loader = ChainDataLoader::builder()
            .config(mystiko_config)
            .chain_id(options.chain_id)
            .fetchers(fetchers)
            .validators(validators)
            .handler(options.handler.clone())
            .build();
        Ok(loader)
    }
}

impl<R, H, F, V> ChainDataLoader<R, H, F, V>
where
    R: LoadedData,
    H: DataHandler<R>,
    F: DataFetcher<R>,
    V: DataValidator<R>,
{
    async fn try_load(&self, params: &ChainLoadParams<'_>) -> DataLoaderResult<LoadStatus> {
        let tasks = self
            .fetchers
            .iter()
            .filter(|f| !self.skip_fetcher(f.fetcher.name(), &params.option.fetcher))
            .map(|f| self.query_loaded_blocks(f, &params.option.fetcher))
            .collect::<Vec<_>>();

        if tasks.is_empty() {
            log::warn!("no fetcher to load data");
            return Err(DataLoaderError::LoaderNoFetchersError);
        }

        let results = futures::future::join_all(tasks).await;
        let fetchers: Vec<_> = results.into_iter().filter_map(|r| r.ok()).collect();
        if fetchers.is_empty() {
            log::error!("failed to query loaded blocks from all fetchers");
            return Err(DataLoaderError::QueryLoadedBlocksError);
        }

        let validators = self
            .validators
            .iter()
            .filter(|v| !self.skip_validator(v.name(), &params.option.validator))
            .map(|v| ValidatorRunParams::builder().validator(v.clone()).build())
            .collect();

        let run_params = LoaderRunParams::builder()
            .params(params)
            .fetchers(fetchers)
            .validators(validators)
            .build();
        self.run_load(&run_params).await
    }

    async fn run_load(&self, run_params: &LoaderRunParams<'_, F, V>) -> DataLoaderResult<LoadStatus> {
        let contracts = self.build_loading_contracts(run_params.params.cfg).await?;
        let mut loaded = None;
        let mut target_block = 0;
        for fetcher_params in run_params.fetchers.iter() {
            let name = fetcher_params.fetcher.name();
            target_block = max(target_block, fetcher_params.loaded_block);
            let fetch_option = self.build_fetch_options(&contracts, fetcher_params).await?;
            if let Some(fetch_option) = &fetch_option {
                if loaded.is_none() {
                    loaded = Some(false);
                }

                let mut chain_data = match self
                    .fetch(fetcher_params, &run_params.params.option.fetcher, fetch_option)
                    .await
                {
                    Err(e) => {
                        log::warn!("fetch fetcher(name={:?}) failed: {:?}", name, e);
                        None
                    }
                    Ok(d) => Some(d),
                };

                if let Some(chain_data) = chain_data.as_mut() {
                    let skip_validation = self.skip_validation(fetcher_params, &run_params.params.option.fetcher);
                    let mut invalid = false;
                    if !skip_validation {
                        if let Err(e) = self.validate(run_params, chain_data).await {
                            log::warn!("validate fetcher(name={:?}) data failed: {:?}", name, e);
                            invalid = true;
                        };
                    }
                    if !invalid {
                        if let Err(e) = self.handle(chain_data).await {
                            log::warn!("handle fetcher(name={:?}) data failed: {:?}", name, e);
                        } else {
                            loaded = Some(true);
                        }
                    }
                }
            }
        }

        if let Some(false) = loaded {
            log::error!("failed to load data from all fetchers");
            return Err(DataLoaderError::LoaderFetchersExhaustedError);
        }

        let loaded_block = self.chain_loaded_block(self.chain_id).await?;
        Ok(LoadStatus::builder()
            .chain_id(self.chain_id)
            .loaded_block(loaded_block.unwrap_or(0))
            .target_block(target_block)
            .build())
    }

    async fn query_loaded_blocks(
        &self,
        fetcher_wrapper: &FetcherWrapper<R, F>,
        options: &LoadFetcherOption,
    ) -> DataLoaderResult<FetcherRunParams<F>> {
        let option = ChainLoadedBlockOptions::builder()
            .chain_id(self.chain_id)
            .config(self.config.clone())
            .build();
        let timeout_duration = Duration::from_millis(options.query_loaded_block_timeout_ms);
        let result = timeout(timeout_duration, fetcher_wrapper.fetcher.chain_loaded_block(&option)).await;
        let name = fetcher_wrapper.fetcher.name();
        match result {
            Ok(Ok(block)) => Ok(FetcherRunParams::builder()
                .fetcher(fetcher_wrapper.fetcher.clone())
                .options(fetcher_wrapper.options.clone())
                .loaded_block(block)
                .build()),
            Ok(Err(e)) => {
                log::warn!("query_loaded_blocks of fetcher(name={:?}) failed: {:?}", name, e);
                Err(e.into())
            }
            Err(_) => {
                log::warn!(
                    "query_loaded_blocks of fetcher(name={:?}) timed out after {:?} ms",
                    name,
                    options.query_loaded_block_timeout_ms
                );
                Err(DataLoaderError::QueryLoadedBlocksTimeoutError(
                    name.to_string(),
                    options.query_loaded_block_timeout_ms,
                ))
            }
        }
    }

    async fn fetch(
        &self,
        params: &FetcherRunParams<F>,
        load_fetcher_options: &LoadFetcherOption,
        fetch_options: &FetchOptions,
    ) -> DataLoaderResult<ChainData<R>> {
        let duration_ms = Duration::from_millis(load_fetcher_options.fetch_timeout_ms);

        let fetch_result = timeout(duration_ms, params.fetcher.fetch(fetch_options)).await;
        match fetch_result {
            Ok(Ok(result)) => {
                let unwrapped = UnwrappedChainResult::from(result);
                unwrapped.contract_errors.iter().for_each(|error| {
                    log::warn!("fetch contract {:?} failed: {:?}", error.address, error.source);
                });
                Ok(unwrapped.result)
            }
            Ok(Err(e)) => Err(e.into()),
            Err(_) => Err(DataLoaderError::FetchTimeoutError(
                params.fetcher.name().to_string(),
                load_fetcher_options.fetch_timeout_ms,
            )),
        }
    }

    async fn validate(&self, run_param: &LoaderRunParams<'_, F, V>, data: &mut ChainData<R>) -> DataLoaderResult<()> {
        if !data.contracts_data.is_empty() {
            for validator_params in run_param.validators.iter() {
                let skip_checkers = run_param
                    .params
                    .option
                    .validator
                    .skips
                    .get(validator_params.validator.name())
                    .map_or_else(HashMap::new, |o| o.skip_checkers.clone());
                let validator_option = ValidateOption::builder()
                    .config(self.config.clone())
                    .validate_concurrency(run_param.params.option.validator.concurrency)
                    .skip_checkers(skip_checkers)
                    .build();
                let validate_result = validator_params.validator.validate(data, &validator_option).await?;
                let unwrapped: UnwrappedChainResult<Vec<String>> = UnwrappedChainResult::from(validate_result);
                unwrapped.contract_errors.iter().for_each(|c| {
                    log::warn!(
                        "validator(name={:?}) contract {:?} failed: {:?}",
                        validator_params.validator.name(),
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
            log::warn!("fetcher contract data is empty");
            Err(DataLoaderError::LoaderEmptyValidateDataError)
        }
    }

    async fn handle(&self, data: &ChainData<R>) -> DataLoaderResult<()> {
        if !data.contracts_data.is_empty() {
            let handler_option = HandleOption::builder().config(self.config.clone()).build();
            let handle_result = self.handler.handle(data, &handler_option).await?;
            let unwrapped: UnwrappedChainResult<Vec<String>> = UnwrappedChainResult::from(handle_result);
            unwrapped.contract_errors.iter().for_each(|c| {
                log::warn!("handle contract {:?} failed: {:?}", c.address, c.source);
            });
            Ok(())
        } else {
            log::warn!("fetcher contract data all invalid");
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

    async fn build_loading_contracts(&self, chain: &ChainConfig) -> DataLoaderResult<Vec<ContractConfig>> {
        if let Some(contracts) = self.handler.query_loading_contracts(self.chain_id).await? {
            return Ok(contracts);
        } else {
            let contracts = chain.contracts();
            if !contracts.is_empty() {
                return Ok(contracts);
            }
        }

        Err(DataLoaderError::LoaderNoContractsError)
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

    async fn build_fetch_options(
        &self,
        contracts: &Vec<ContractConfig>,
        fetcher: &FetcherRunParams<F>,
    ) -> DataLoaderResult<Option<FetchOptions>> {
        let mut contract_options: Vec<ContractFetchOptions> = vec![];
        for contract_cfg in contracts {
            let contract_start_block = self.build_contract_start_block(contract_cfg).await?;
            // skip contract that already fetch to fetcher.loaded_block
            if contract_start_block > fetcher.loaded_block {
                continue;
            }
            let fetch_option = ContractFetchOptions::builder()
                .contract_config(contract_cfg.clone())
                .start_block(Some(contract_start_block))
                .build();
            contract_options.push(fetch_option);
        }
        let start_block = contract_options.iter().filter_map(|s| s.start_block).min();
        if let Some(start_block) = start_block {
            let options = FetchOptions::builder()
                .config(self.config.clone())
                .chain_id(self.chain_id)
                .start_block(start_block)
                .target_block(fetcher.loaded_block)
                .contract_options(Some(contract_options))
                .build();
            Ok(Some(options))
        } else {
            Ok(None)
        }
    }

    fn skip_fetcher(&self, fetcher_name: &str, load_fetcher_option: &LoadFetcherOption) -> bool {
        load_fetcher_option
            .skips
            .get(fetcher_name)
            .and_then(|option| option.skip_fetch)
            .unwrap_or_default()
    }

    fn skip_validator(&self, validator_name: &str, load_validator_option: &LoadValidatorOption) -> bool {
        load_validator_option
            .skips
            .get(validator_name)
            .and_then(|option| option.skip_validation)
            .unwrap_or_default()
    }

    fn skip_validation(&self, run_param: &FetcherRunParams<F>, load_fetcher_option: &LoadFetcherOption) -> bool {
        load_fetcher_option
            .skips
            .get(run_param.fetcher.name())
            .and_then(|option| option.skip_validation)
            .unwrap_or(run_param.options.skip_validation)
    }
}
