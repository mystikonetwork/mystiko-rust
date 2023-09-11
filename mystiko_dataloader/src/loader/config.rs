use crate::data::LoadedData;
use crate::fetcher::{
    create_provider_pool_from_config, DataFetcher, DataPackerFetcherV1, EtherscanFetcher, IndexerFetcher,
    ProviderFetcher,
};
use crate::handler::DataHandler;
use crate::loader::ChainDataFetcher;
use crate::validator::rule::create_rule_validator_by_types;
use crate::validator::DataValidator;
use anyhow::Error as AnyhowError;
use mystiko_config::MystikoConfig;
use mystiko_ethers::Providers;
use mystiko_protos::loader::v1::{
    FetcherConfig, FetcherType, LoaderConfig, RuleValidatorCheckerType, RuleValidatorConfig, ValidatorConfig,
    ValidatorType,
};
use std::sync::Arc;
use thiserror::Error;
use typed_builder::TypedBuilder;

pub type DataLoaderConfigResult<T> = anyhow::Result<T, DataLoaderConfigError>;
type ChainDataFetcherFromConfig<R> = ChainDataFetcher<R, Box<dyn DataFetcher<R>>>;

#[derive(Error, Debug)]
pub enum DataLoaderConfigError {
    #[error("not supported fetcher type {0}")]
    FetcherTypeError(i32),
    #[error("not supported validator type {0}")]
    ValidatorTypeError(i32),
    #[error("not supported rule validator checker type {0}")]
    RuleValidatorCheckerTypeError(i32),
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct LoaderConfigOptions<
    R,
    P = Box<dyn Providers>,
    F = Box<dyn DataFetcher<R>>,
    V = Box<dyn DataValidator<R>>,
    H = Box<dyn DataHandler<R>>,
> {
    pub chain_id: u64,
    pub config: LoaderConfig,
    pub handler: Arc<H>,
    #[builder(default, setter(strip_option))]
    pub providers: Option<Arc<P>>,
    #[builder(default, setter(skip))]
    pub fetchers: Vec<Arc<ChainDataFetcher<R, F>>>,
    #[builder(default, setter(skip))]
    pub validators: Vec<Arc<V>>,
    #[builder(default, setter(skip))]
    _phantom: std::marker::PhantomData<R>,
}

impl<R, P, F, V, H> LoaderConfigOptions<R, P, F, V, H>
where
    R: LoadedData + 'static,
    P: Providers + 'static,
    F: DataFetcher<R>,
    V: DataValidator<R>,
    H: DataHandler<R>,
{
    pub fn fetcher_types(&self) -> DataLoaderConfigResult<Vec<FetcherType>> {
        if self.config.fetchers.is_empty() {
            if self.fetchers.is_empty() {
                Ok(vec![FetcherType::Provider])
            } else {
                Ok(vec![])
            }
        } else {
            let mut fetcher_types = vec![];
            for fetcher in self.config.fetchers.iter() {
                let fetcher_type =
                    FetcherType::from_i32(*fetcher).ok_or(DataLoaderConfigError::FetcherTypeError(*fetcher))?;
                fetcher_types.push(fetcher_type);
            }
            Ok(fetcher_types)
        }
    }

    pub fn validator_types(&self) -> DataLoaderConfigResult<Vec<ValidatorType>> {
        let mut validator_types = vec![];
        for validator in self.config.validators.iter() {
            let validator_type =
                ValidatorType::from_i32(*validator).ok_or(DataLoaderConfigError::ValidatorTypeError(*validator))?;
            validator_types.push(validator_type);
        }
        Ok(validator_types)
    }

    pub fn rule_validator_checker_types(
        &self,
        rule_config: &RuleValidatorConfig,
    ) -> DataLoaderConfigResult<Vec<RuleValidatorCheckerType>> {
        let mut checker_types = vec![];
        for checker in rule_config.checkers.iter() {
            let checker_type = RuleValidatorCheckerType::from_i32(*checker)
                .ok_or(DataLoaderConfigError::RuleValidatorCheckerTypeError(*checker))?;
            checker_types.push(checker_type);
        }
        Ok(checker_types)
    }

    pub async fn build_mystiko_config(&self) -> DataLoaderConfigResult<Arc<MystikoConfig>> {
        let mystiko_config = MystikoConfig::from_options(self.config.mystiko_config_options.clone()).await?;
        Ok(Arc::new(mystiko_config))
    }

    pub fn build_providers(
        &self,
        mystiko_config: Arc<MystikoConfig>,
    ) -> DataLoaderConfigResult<Arc<Box<dyn Providers>>> {
        let provider_cfg = self
            .config
            .fetcher_config
            .clone()
            .and_then(|fetcher_config| fetcher_config.provider);
        let providers = create_provider_pool_from_config(provider_cfg, mystiko_config.clone());
        Ok(Arc::new(Box::new(providers) as Box<dyn Providers>))
    }

    pub fn build_fetchers(
        &self,
        mystiko_config: Arc<MystikoConfig>,
        providers: Arc<P>,
    ) -> DataLoaderConfigResult<Vec<Arc<ChainDataFetcherFromConfig<R>>>> {
        let mut fetchers: Vec<Arc<ChainDataFetcherFromConfig<R>>> = vec![];
        let fetcher_config: FetcherConfig = self.config.fetcher_config.clone().into();
        for fetcher_type in self.fetcher_types()? {
            match fetcher_type {
                FetcherType::Unspecified => {
                    return Err(DataLoaderConfigError::FetcherTypeError(fetcher_type as i32));
                }
                FetcherType::Packer => {
                    let skip_validation = match &fetcher_config.packer {
                        None => false,
                        Some(c) => c.skip_validation.unwrap_or(false),
                    };
                    let packer = DataPackerFetcherV1::from(mystiko_config.clone());
                    let fetcher = ChainDataFetcher::builder()
                        .fetcher(Arc::new(Box::new(packer) as Box<dyn DataFetcher<R>>))
                        .skip_validation(skip_validation)
                        .build();
                    fetchers.push(Arc::new(fetcher));
                }
                FetcherType::Indexer => {
                    let skip_validation = match &fetcher_config.indexer {
                        None => false,
                        Some(c) => c.skip_validation.unwrap_or(false),
                    };
                    let indexer = IndexerFetcher::from_config(mystiko_config.clone(), fetcher_config.indexer.clone())?;
                    let fetcher = ChainDataFetcher::builder()
                        .fetcher(Arc::new(Box::new(indexer) as Box<dyn DataFetcher<R>>))
                        .skip_validation(skip_validation)
                        .build();
                    fetchers.push(Arc::new(fetcher));
                }
                FetcherType::Etherscan => {
                    let skip_validation = match &fetcher_config.etherscan {
                        None => false,
                        Some(c) => c.skip_validation.unwrap_or(false),
                    };
                    let etherscan =
                        EtherscanFetcher::from_config(mystiko_config.clone(), fetcher_config.etherscan.clone())?;
                    let fetcher = ChainDataFetcher::builder()
                        .fetcher(Arc::new(Box::new(etherscan) as Box<dyn DataFetcher<R>>))
                        .skip_validation(skip_validation)
                        .build();
                    fetchers.push(Arc::new(fetcher));
                }
                FetcherType::Provider => {
                    let skip_validation = match &fetcher_config.provider {
                        None => false,
                        Some(c) => c.skip_validation.unwrap_or(false),
                    };
                    let provider_fetcher =
                        ProviderFetcher::from_config(fetcher_config.provider.clone(), providers.clone());
                    let fetcher = ChainDataFetcher::builder()
                        .fetcher(Arc::new(Box::new(provider_fetcher) as Box<dyn DataFetcher<R>>))
                        .skip_validation(skip_validation)
                        .build();
                    fetchers.push(Arc::new(fetcher));
                }
            }
        }

        Ok(fetchers)
    }

    pub fn build_validators(
        &self,
        providers: Arc<Box<dyn Providers>>,
        handler: Arc<Box<dyn DataHandler<R>>>,
    ) -> DataLoaderConfigResult<Vec<Arc<Box<dyn DataValidator<R>>>>> {
        let mut validators = vec![];
        for validator_type in self.validator_types()? {
            match validator_type {
                ValidatorType::Unspecified => {
                    return Err(DataLoaderConfigError::ValidatorTypeError(validator_type as i32));
                }
                ValidatorType::Rule => {
                    let validator_config: ValidatorConfig = self.config.validator_config.clone().into();
                    let rule_cfg: RuleValidatorConfig = validator_config.rule.into();
                    let checker_types = self.rule_validator_checker_types(&rule_cfg)?;
                    let rule_validator =
                        create_rule_validator_by_types(handler.clone(), providers.clone(), &checker_types)
                            .map_err(|e| DataLoaderConfigError::AnyhowError(e.into()))?;
                    validators.push(Arc::new(Box::new(rule_validator) as Box<dyn DataValidator<R>>));
                }
            }
        }
        Ok(validators)
    }
}
