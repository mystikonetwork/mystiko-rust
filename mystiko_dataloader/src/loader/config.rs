use crate::data::LoadedData;
use crate::fetcher::{
    create_provider_pool_from_config, DataFetcher, DataPackerFetcherV1, EtherscanFetcher, IndexerFetcher,
    ProviderFetcher,
};
use crate::handler::DataHandler;
use crate::loader::DataLoaderResult;
use crate::validator::rule::create_rule_validator_by_types;
use crate::validator::DataValidator;
use crate::DataLoaderError;
use mystiko_config::MystikoConfig;
use mystiko_ethers::Providers;
use mystiko_protos::loader::v1::{
    FetcherConfig, FetcherType, LoaderConfig, RuleValidatorCheckerType, RuleValidatorConfig, ValidatorConfig,
    ValidatorType,
};
use std::sync::Arc;
use typed_builder::TypedBuilder;

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
    #[builder(default)]
    pub providers: Option<Arc<P>>,
    #[builder(default)]
    pub fetchers: Vec<Arc<F>>,
    #[builder(default)]
    pub validators: Vec<Arc<V>>,
    #[builder(default)]
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
    pub fn fetcher_types(&self) -> DataLoaderResult<Vec<FetcherType>> {
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
                    FetcherType::from_i32(*fetcher).ok_or(DataLoaderError::FetcherTypeError(*fetcher))?;
                fetcher_types.push(fetcher_type);
            }
            Ok(fetcher_types)
        }
    }

    pub fn validator_types(&self) -> DataLoaderResult<Vec<ValidatorType>> {
        let mut validator_types = vec![];
        for validator in self.config.validators.iter() {
            let validator_type =
                ValidatorType::from_i32(*validator).ok_or(DataLoaderError::ValidatorTypeError(*validator))?;
            validator_types.push(validator_type);
        }
        Ok(validator_types)
    }

    pub fn rule_validator_checker_types(
        &self,
        rule_config: &RuleValidatorConfig,
    ) -> DataLoaderResult<Vec<RuleValidatorCheckerType>> {
        let mut checker_types = vec![];
        for checker in rule_config.checkers.iter() {
            let checker_type = RuleValidatorCheckerType::from_i32(*checker)
                .ok_or(DataLoaderError::RuleValidatorCheckerTypeError(*checker))?;
            checker_types.push(checker_type);
        }
        Ok(checker_types)
    }

    pub async fn build_mystiko_config(&self) -> DataLoaderResult<Arc<MystikoConfig>> {
        let mystiko_config = MystikoConfig::from_options(self.config.mystiko_config_options.clone()).await?;
        Ok(Arc::new(mystiko_config))
    }

    pub fn build_providers(&self, mystiko_config: Arc<MystikoConfig>) -> DataLoaderResult<Arc<Box<dyn Providers>>> {
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
        chain_id: u64,
        mystiko_config: Arc<MystikoConfig>,
        providers: Arc<P>,
    ) -> DataLoaderResult<Vec<Arc<Box<dyn DataFetcher<R>>>>> {
        let mut fetchers = vec![];
        let fetcher_config: FetcherConfig = self.config.fetcher_config.clone().into();
        for fetcher_type in self.fetcher_types()? {
            match fetcher_type {
                FetcherType::Unspecified => {
                    return Err(DataLoaderError::FetcherTypeError(fetcher_type as i32));
                }
                FetcherType::Packer => {
                    let packer = DataPackerFetcherV1::from(mystiko_config.clone());
                    fetchers.push(Arc::new(Box::new(packer) as Box<dyn DataFetcher<R>>));
                }
                FetcherType::Indexer => {
                    let indexer = IndexerFetcher::from_config(mystiko_config.clone(), fetcher_config.indexer.clone())?;
                    fetchers.push(Arc::new(Box::new(indexer) as Box<dyn DataFetcher<R>>));
                }
                FetcherType::Etherscan => {
                    let etherscan = EtherscanFetcher::from_config(
                        chain_id,
                        mystiko_config.clone(),
                        fetcher_config.etherscan.clone(),
                    )?;
                    fetchers.push(Arc::new(Box::new(etherscan) as Box<dyn DataFetcher<R>>));
                }
                FetcherType::Provider => {
                    let provider_fetcher =
                        ProviderFetcher::from_config(fetcher_config.provider.clone(), providers.clone());
                    fetchers.push(Arc::new(Box::new(provider_fetcher) as Box<dyn DataFetcher<R>>));
                }
            }
        }

        Ok(fetchers)
    }

    pub fn build_validators(
        &self,
        providers: Arc<Box<dyn Providers>>,
        handler: Arc<Box<dyn DataHandler<R>>>,
    ) -> DataLoaderResult<Vec<Arc<Box<dyn DataValidator<R>>>>> {
        let mut validators = vec![];
        for validator_type in self.validator_types()? {
            match validator_type {
                ValidatorType::Unspecified => {
                    return Err(DataLoaderError::ValidatorTypeError(validator_type as i32));
                }
                ValidatorType::Rule => {
                    let validator_config: ValidatorConfig = self.config.validator_config.clone().into();
                    let rule_cfg: RuleValidatorConfig = validator_config.rule.into();
                    let checker_types = self.rule_validator_checker_types(&rule_cfg)?;
                    let rule_validator =
                        create_rule_validator_by_types(handler.clone(), providers.clone(), &checker_types)?;
                    validators.push(Arc::new(Box::new(rule_validator) as Box<dyn DataValidator<R>>));
                }
            }
        }
        Ok(validators)
    }
}
