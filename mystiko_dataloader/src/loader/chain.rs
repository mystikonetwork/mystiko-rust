use crate::data::ChainData;
use crate::data::LoadedData;
use crate::data::UnwrappedChainResult;
use crate::error::DataLoaderError;
use crate::fetcher::{ChainLoadedBlockOptions, ContractFetchOptions, DataFetcher, FetchOptions, FetcherOptions};
use crate::handler::{DataHandler, HandleOption};
use crate::loader::{
    DataLoader, DataLoaderConfigResult, DataLoaderResult, LoadFetcherOption, LoadOption, LoadValidatorOption,
    LoaderConfigOptions,
};
use crate::validator::{DataValidator, ValidateOption};
use async_trait::async_trait;
use log::{error, warn};
use mystiko_config::{ChainConfig, ContractConfig, MystikoConfig};
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
    #[builder(default = vec![])]
    fetchers: Vec<Arc<F>>,
    #[builder(default = HashMap::new())]
    fetcher_options: HashMap<usize, FetcherOptions>,
    #[builder(default = vec![])]
    validators: Vec<Arc<V>>,
    handler: Arc<H>,
    #[builder(default = Default::default())]
    _phantom: std::marker::PhantomData<R>,
}

#[derive(Debug, Clone, TypedBuilder)]
struct ChainLoadParams<'a> {
    pub(crate) cfg: &'a ChainConfig,
    pub(crate) start_block: u64,
    pub(crate) option: &'a LoadOption,
}

#[derive(Debug, Clone, TypedBuilder)]
struct FetcherRunParams<F> {
    pub(crate) index: usize,
    pub(crate) fetcher: Arc<F>,
    pub(crate) loaded_block: u64,
}

#[derive(Debug, Clone, TypedBuilder)]
struct LoaderRunParams<'a, F> {
    pub(crate) params: &'a ChainLoadParams<'a>,
    pub(crate) target_block: u64,
    pub(crate) fetchers: Vec<FetcherRunParams<F>>,
}

#[async_trait]
impl<R, H, F, V> DataLoader for ChainDataLoader<R, H, F, V>
where
    R: LoadedData,
    H: DataHandler<R>,
    F: DataFetcher<R>,
    V: DataValidator<R>,
{
    async fn load<O>(&self, options: O) -> DataLoaderResult<()>
    where
        O: Into<LoadOption> + Send + Sync,
    {
        let options = options.into();
        let chain_cfg = self.build_chain_config().await?;
        let chain_start_block = self.build_chain_start_block(&chain_cfg).await?;
        let params = ChainLoadParams::builder()
            .cfg(&chain_cfg)
            .start_block(chain_start_block)
            .option(&options)
            .build();

        self.try_load(&params).await
    }
}

#[async_trait]
pub trait FromConfig<'a, T>: Sized {
    async fn from_config(item: &'a T) -> DataLoaderConfigResult<Self>;
}

#[async_trait]
impl<'a, R, H> FromConfig<'a, LoaderConfigOptions<R, H>> for ChainDataLoader<R, H>
where
    R: LoadedData + 'static,
    H: DataHandler<R> + 'static,
{
    async fn from_config(options: &'a LoaderConfigOptions<R, H>) -> DataLoaderConfigResult<Self> {
        options.validate_config()?;

        let mystiko_config = options.build_mystiko_config().await?;
        let providers = match &options.providers {
            None => options.build_providers(mystiko_config.clone())?,
            Some(p) => p.clone(),
        };

        let (fetchers, fetcher_options) = if options.fetchers.is_empty() {
            options.build_fetchers(mystiko_config.clone(), providers.clone())?
        } else {
            (options.fetchers.clone(), HashMap::new())
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
            .fetcher_options(fetcher_options)
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
    async fn try_load(&self, params: &ChainLoadParams<'_>) -> DataLoaderResult<()> {
        let mut tasks = vec![];
        for (index, fetcher) in self.fetchers.iter().enumerate() {
            tasks.push(self.query_loaded_blocks(index, fetcher, &params.option.fetcher));
        }

        let results = futures::future::join_all(tasks).await;
        let fetchers: Vec<_> = results.into_iter().filter_map(|r| r.ok()).collect();
        let target_block = fetchers
            .iter()
            .map(|f| f.loaded_block)
            .max()
            .ok_or(DataLoaderError::QueryLoadedBlocksError)?;
        let run_params = LoaderRunParams::builder()
            .params(params)
            .target_block(target_block)
            .fetchers(fetchers)
            .build();

        self.run_load(&run_params).await
    }

    async fn run_load(&self, run_params: &LoaderRunParams<'_, F>) -> DataLoaderResult<()> {
        let (contract_options, contracts) = self.build_loading_contracts(run_params.params.cfg).await?;
        let mut fetch_option = FetchOptions::builder()
            .config(self.config.clone())
            .chain_id(self.chain_id)
            .start_block(run_params.params.start_block)
            .target_block(run_params.target_block)
            .contract_options(contract_options)
            .build();

        for param in run_params.fetchers.iter() {
            fetch_option.target_block = param.loaded_block;
            if let Some(contracts_options) = fetch_option.contract_options.as_mut() {
                for contract_options in contracts_options.iter_mut() {
                    if contract_options.target_block.is_some() {
                        contract_options.target_block = Some(param.loaded_block);
                    }
                }
            }
            let mut chain_data = match self
                .fetch(&run_params.params.option.fetcher, param, &fetch_option)
                .await
            {
                Err(e) => {
                    warn!("fetch fetcher(index={:?}) raised error: {:?}", param.index, e);
                    continue;
                }
                Ok(d) => d,
            };

            let skip_validation = self
                .fetcher_options
                .get(&param.index)
                .map(|o| o.skip_validation)
                .unwrap_or(false);
            if !skip_validation {
                if let Err(e) = self
                    .validate(&mut chain_data, &run_params.params.option.validator)
                    .await
                {
                    warn!("validate fetcher(index={:?}) data raised error: {:?}", param.index, e);
                    continue;
                };
            }

            if let Err(e) = self.handle(&chain_data).await {
                warn!("handle fetcher(index={:?}) data raised error: {:?}", param.index, e);
                continue;
            };

            let retry_fetch_options = self.build_retry_fetch_options(&contracts, run_params, param).await;
            if let Ok(r) = retry_fetch_options {
                match r {
                    Some(o) => fetch_option = o,
                    None => return Ok(()),
                }
            }
        }

        error!("failed to load data from all fetchers");
        Err(DataLoaderError::LoaderFetchersExhaustedError)
    }

    async fn query_loaded_blocks(
        &self,
        index: usize,
        fetcher: &Arc<F>,
        options: &LoadFetcherOption,
    ) -> DataLoaderResult<FetcherRunParams<F>> {
        let option = ChainLoadedBlockOptions::builder()
            .chain_id(self.chain_id)
            .config(self.config.clone())
            .build();
        let timeout_duration = Duration::from_millis(options.query_loaded_block_timeout_ms);
        let result = timeout(timeout_duration, fetcher.chain_loaded_block(&option)).await;
        match result {
            Ok(Ok(block)) => Ok(FetcherRunParams::builder()
                .index(index)
                .fetcher(fetcher.clone())
                .loaded_block(block)
                .build()),
            Ok(Err(e)) => {
                warn!(
                    "query_loaded_blocks of fetcher(index={:?}) raised error: {:?}",
                    index, e
                );
                Err(e.into())
            }
            Err(_) => {
                warn!(
                    "query_loaded_blocks of fetcher(index={:?}) timed out after {:?} ms",
                    index, options.query_loaded_block_timeout_ms
                );
                Err(DataLoaderError::QueryLoadedBlocksTimeoutError(
                    index,
                    options.query_loaded_block_timeout_ms,
                ))
            }
        }
    }

    async fn fetch(
        &self,
        load_fetcher_options: &LoadFetcherOption,
        run_param: &FetcherRunParams<F>,
        fetch_options: &FetchOptions,
    ) -> DataLoaderResult<ChainData<R>> {
        let duration_ms = Duration::from_millis(load_fetcher_options.fetch_timeout_ms);

        let fetch_result = timeout(duration_ms, run_param.fetcher.fetch(fetch_options)).await;
        match fetch_result {
            Ok(Ok(result)) => {
                let unwrapped = UnwrappedChainResult::from(result);
                unwrapped.contract_errors.iter().for_each(|error| {
                    warn!("fetch contract {:?} raised error: {:?}", error.address, error.source);
                });
                Ok(unwrapped.result)
            }
            Ok(Err(e)) => Err(e.into()),
            Err(_) => Err(DataLoaderError::FetchTimeoutError(
                run_param.index,
                load_fetcher_options.fetch_timeout_ms,
            )),
        }
    }

    async fn validate(&self, data: &mut ChainData<R>, options: &LoadValidatorOption) -> DataLoaderResult<()> {
        if !data.contracts_data.is_empty() {
            let validator_option = ValidateOption::builder()
                .config(self.config.clone())
                .validate_concurrency(options.concurrency)
                .build();
            for (index, validator) in self.validators.iter().enumerate() {
                let validate_result = validator.validate(data, &validator_option).await?;
                let unwrapped: UnwrappedChainResult<Vec<String>> = UnwrappedChainResult::from(validate_result);
                unwrapped.contract_errors.iter().for_each(|c| {
                    warn!(
                        "validator(index={:?}) contract {:?} raised error: {:?}",
                        index, c.address, c.source
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
                warn!("handle contract {:?} raised error: {:?}", c.address, c.source);
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

    async fn build_loading_contracts(
        &self,
        chain: &ChainConfig,
    ) -> DataLoaderResult<(Option<Vec<ContractFetchOptions>>, Vec<ContractConfig>)> {
        if let Some(contracts) = self.handler.query_loading_contracts(self.chain_id).await? {
            if !contracts.is_empty() {
                let contract_options = contracts
                    .iter()
                    .map(|c| ContractFetchOptions::builder().contract_config(c.clone()).build())
                    .collect();
                return Ok((Some(contract_options), contracts));
            }
        } else {
            let contracts = chain.contracts_with_disabled();
            if !contracts.is_empty() {
                return Ok((None, contracts));
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
        run_params: &LoaderRunParams<'_, F>,
        fetcher: &FetcherRunParams<F>,
    ) -> DataLoaderResult<Option<FetchOptions>> {
        let mut contract_options: Vec<ContractFetchOptions> = vec![];
        for contract_cfg in contracts {
            if let Ok(contract_start_block) = self.build_contract_start_block(contract_cfg).await {
                // skip contract that already fetch to chain_target_block
                if contract_start_block > run_params.target_block {
                    continue;
                }

                if contract_start_block - 1 > fetcher.loaded_block {
                    warn!(
                        "contract {:?} start_block {:?} > fetcher {:?} loaded_block {:?} ",
                        contract_cfg.address(),
                        contract_start_block,
                        fetcher.index,
                        fetcher.loaded_block
                    );
                    return Err(DataLoaderError::FetcherLoadedBlockSmallerError(
                        fetcher.loaded_block,
                        contract_start_block,
                    ));
                }

                let fetch_option = ContractFetchOptions::builder()
                    .contract_config(contract_cfg.clone())
                    .start_block(Some(contract_start_block))
                    .target_block(Some(run_params.target_block))
                    .build();
                contract_options.push(fetch_option);
            }
        }

        if contract_options.is_empty() {
            Ok(None)
        } else {
            let options = FetchOptions::builder()
                .config(self.config.clone())
                .chain_id(self.chain_id)
                .start_block(run_params.params.start_block)
                .target_block(run_params.target_block)
                .contract_options(Some(contract_options))
                .build();
            Ok(Some(options))
        }
    }
}
