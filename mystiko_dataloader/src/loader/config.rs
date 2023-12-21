use crate::data::LoadedData;
use crate::fetcher::{
    create_provider_pool_from_config, DataFetcher, DataPackerFetcherV1, EtherscanFetcher, FetcherOptions,
    ProviderFetcher, SequencerFetcher,
};
use crate::handler::DataHandler;
use crate::loader::FetcherWrapper;
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

#[derive(Error, Debug)]
pub enum DataLoaderConfigError {
    #[error("not supported fetcher type {0}")]
    FetcherTypeError(i32),
    #[error("fetcher config not exist for fetcher type {0}")]
    FetcherConfigNotExistError(i32),
    #[error("not supported validator type {0}")]
    ValidatorTypeError(i32),
    #[error("not supported rule validator checker type {0}")]
    RuleValidatorCheckerTypeError(i32),
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
}

#[derive(TypedBuilder)]
pub struct LoaderConfigOptions<R, H = Box<dyn DataHandler<R>>> {
    pub chain_id: u64,
    pub config: LoaderConfig,
    pub handler: Arc<H>,
    #[builder(default, setter(strip_option))]
    pub mystiko_config: Option<Arc<MystikoConfig>>,
    #[builder(default, setter(strip_option))]
    pub providers: Option<Arc<Box<dyn Providers>>>,
    #[builder(default)]
    pub fetchers: Vec<Arc<Box<dyn DataFetcher<R>>>>,
    #[builder(default)]
    pub validators: Vec<Arc<Box<dyn DataValidator<R>>>>,
    #[builder(default, setter(skip))]
    _phantom: std::marker::PhantomData<R>,
}

impl<R, H> LoaderConfigOptions<R, H>
where
    R: LoadedData + 'static,
    H: DataHandler<R> + 'static,
{
    pub(crate) fn validate_config(&self) -> DataLoaderConfigResult<()> {
        self.validate_etherscan_fetcher_config()
    }

    pub(crate) fn fetcher_types(&self) -> DataLoaderConfigResult<Vec<FetcherType>> {
        if self.config.fetchers.is_empty() {
            if self.fetchers.is_empty() {
                Ok(vec![FetcherType::Provider])
            } else {
                Ok(vec![])
            }
        } else {
            let mut fetcher_types = vec![];
            for fetcher in self.config.fetchers().iter() {
                let fetcher_type =
                    FetcherType::from_i32(*fetcher).ok_or(DataLoaderConfigError::FetcherTypeError(*fetcher))?;
                fetcher_types.push(fetcher_type);
            }
            Ok(fetcher_types)
        }
    }

    pub(crate) fn validator_types(&self) -> DataLoaderConfigResult<Vec<ValidatorType>> {
        let mut validator_types = vec![];
        for validator in self.config.validators().iter() {
            let validator_type =
                ValidatorType::from_i32(*validator).ok_or(DataLoaderConfigError::ValidatorTypeError(*validator))?;
            validator_types.push(validator_type);
        }
        Ok(validator_types)
    }

    pub(crate) fn rule_validator_checker_types(
        &self,
        rule_config: &RuleValidatorConfig,
    ) -> DataLoaderConfigResult<Vec<RuleValidatorCheckerType>> {
        let mut checker_types = vec![];
        for checker in rule_config.checkers().iter() {
            let checker_type = RuleValidatorCheckerType::from_i32(*checker)
                .ok_or(DataLoaderConfigError::RuleValidatorCheckerTypeError(*checker))?;
            checker_types.push(checker_type);
        }
        Ok(checker_types)
    }

    pub(crate) async fn build_mystiko_config(&self) -> DataLoaderConfigResult<Arc<MystikoConfig>> {
        if let Some(mystiko_config) = &self.mystiko_config {
            Ok(mystiko_config.clone())
        } else {
            let mystiko_config = MystikoConfig::from_options(self.config.mystiko_config_options.clone()).await?;
            Ok(Arc::new(mystiko_config))
        }
    }

    pub(crate) fn build_providers(
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

    pub(crate) async fn build_fetchers(
        &self,
        chain_id: u64,
        mystiko_config: Arc<MystikoConfig>,
        providers: Arc<Box<dyn Providers>>,
    ) -> DataLoaderConfigResult<Vec<FetcherWrapper<R>>> {
        let mut fetchers: Vec<FetcherWrapper<R>> = vec![];
        let fetcher_config: FetcherConfig = self.config.fetcher_config.clone().into();
        for fetcher_type in self.fetcher_types()?.iter() {
            match fetcher_type {
                FetcherType::Unspecified => {
                    return Err(DataLoaderConfigError::FetcherTypeError(*fetcher_type as i32));
                }
                FetcherType::Packer => {
                    let packer = DataPackerFetcherV1::from(mystiko_config.clone());
                    let (skip_validation, target_block_priority) = if let Some(c) = &fetcher_config.packer {
                        (c.skip_validation.unwrap_or(false), c.target_block_priority.unwrap_or(0))
                    } else {
                        (false, 0)
                    };
                    fetchers.push(
                        FetcherWrapper::builder()
                            .fetcher(Arc::new(Box::new(packer) as Box<dyn DataFetcher<R>>))
                            .options(
                                FetcherOptions::builder()
                                    .skip_validation(skip_validation)
                                    .target_block_priority(target_block_priority)
                                    .build(),
                            )
                            .build(),
                    );
                }
                FetcherType::Sequencer => {
                    let sequencer = SequencerFetcher::<R, mystiko_sequencer_client::v1::SequencerClient>::from_config(
                        mystiko_config.clone(),
                        fetcher_config.sequencer.clone(),
                    )
                    .await?;
                    let (skip_validation, target_block_priority) = if let Some(c) = &fetcher_config.sequencer {
                        (c.skip_validation.unwrap_or(false), c.target_block_priority.unwrap_or(0))
                    } else {
                        (false, 0)
                    };
                    fetchers.push(
                        FetcherWrapper::builder()
                            .fetcher(Arc::new(Box::new(sequencer) as Box<dyn DataFetcher<R>>))
                            .options(
                                FetcherOptions::builder()
                                    .skip_validation(skip_validation)
                                    .target_block_priority(target_block_priority)
                                    .build(),
                            )
                            .build(),
                    );
                }
                FetcherType::Etherscan => {
                    let etherscan = EtherscanFetcher::from_config(
                        chain_id,
                        mystiko_config.clone(),
                        fetcher_config.etherscan.clone(),
                    )?;
                    let (skip_validation, target_block_priority) = if let Some(c) = &fetcher_config.etherscan {
                        (c.skip_validation.unwrap_or(false), c.target_block_priority.unwrap_or(0))
                    } else {
                        (false, 0)
                    };
                    fetchers.push(
                        FetcherWrapper::builder()
                            .fetcher(Arc::new(Box::new(etherscan) as Box<dyn DataFetcher<R>>))
                            .options(
                                FetcherOptions::builder()
                                    .skip_validation(skip_validation)
                                    .target_block_priority(target_block_priority)
                                    .build(),
                            )
                            .build(),
                    );
                }
                FetcherType::Provider => {
                    let provider_fetcher =
                        ProviderFetcher::from_config(fetcher_config.provider.clone(), providers.clone());
                    let (skip_validation, target_block_priority) = if let Some(c) = &fetcher_config.provider {
                        (c.skip_validation.unwrap_or(false), c.target_block_priority.unwrap_or(0))
                    } else {
                        (false, 0)
                    };
                    fetchers.push(
                        FetcherWrapper::builder()
                            .fetcher(Arc::new(Box::new(provider_fetcher) as Box<dyn DataFetcher<R>>))
                            .options(
                                FetcherOptions::builder()
                                    .skip_validation(skip_validation)
                                    .target_block_priority(target_block_priority)
                                    .build(),
                            )
                            .build(),
                    );
                }
            }
        }

        Ok(fetchers)
    }

    pub(crate) fn build_validators(
        &self,
        providers: Arc<Box<dyn Providers>>,
        handler: Arc<H>,
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

    fn validate_etherscan_fetcher_config(&self) -> DataLoaderConfigResult<()> {
        if !self.config.fetchers().contains(&(FetcherType::Etherscan as i32)) {
            return Ok(());
        }

        let etherscan_config = self.config.fetcher_config.as_ref().and_then(|c| c.etherscan.as_ref());

        if let Some(e) = etherscan_config {
            if e.chains.get(&self.chain_id).is_none() {
                return Err(DataLoaderConfigError::FetcherConfigNotExistError(
                    FetcherType::Etherscan as i32,
                ));
            }
        }

        Ok(())
    }
}
