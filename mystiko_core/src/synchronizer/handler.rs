use crate::{
    Commitment, Contract, Database, FromContext, MystikoContext, MystikoError, Nullifier, SyncLoaderHandler,
    SynchronizerHandler,
};
use async_trait::async_trait;
use mystiko_config::{ChainConfig, MystikoConfig};
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::fetcher::{PACKER_FETCHER_NAME, PROVIDER_FETCHER_NAME, SEQUENCER_FETCHER_NAME};
use mystiko_dataloader::handler::{DataHandler, DatabaseHandler};
use mystiko_dataloader::loader::{
    ChainDataLoader, DataLoader, DataLoaderConfigError, FromConfig, LoadFetcherOption, LoadFetcherSkipOption,
    LoadOption, LoadValidatorOption, LoadValidatorSkipOption, LoaderConfigOptions,
};
use mystiko_dataloader::validator::rule::{
    RULE_COUNTER_CHECKER_NAME, RULE_INTEGRITY_CHECKER_NAME, RULE_MERKLE_TREE_CHECKER_NAME, RULE_SEQUENCE_CHECKER_NAME,
    RULE_VALIDATOR_NAME,
};
use mystiko_dataloader::{loader::ResetOptions as DataLoaderResetOptions, DataLoaderError};
use mystiko_ethers::Providers;
use mystiko_protos::core::synchronizer::v1::{
    ChainStatus, ContractStatus, ResetOptions, SyncOptions, SynchronizerStatus,
};
use mystiko_protos::loader::v1::{
    FetcherConfig, LoaderConfig, PackerFetcherConfig, ProviderFetcherChainConfig, ProviderFetcherConfig,
    RuleValidatorCheckerType, RuleValidatorConfig, SequencerFetcherConfig, ValidatorConfig,
};
use mystiko_protos::loader::v1::{FetcherType, ValidatorType};
use mystiko_storage::{StatementFormatter, Storage};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use typed_builder::TypedBuilder;

const DEFAULT_PROVIDER_FETCHER_DELAY_BLOCK_MULTIPLE: u64 = 3;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Synchronizer<L: DataLoader> {
    mystiko_config: Arc<MystikoConfig>,
    loaders: HashMap<u64, L>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SynchronizerOptions<F: StatementFormatter, S: Storage> {
    mystiko_config: Arc<MystikoConfig>,
    providers: Arc<Box<dyn Providers>>,
    db: Arc<Database<F, S>>,
    #[builder(default)]
    loader_config: Option<LoaderConfig>,
}

#[derive(Debug, Error)]
pub enum SynchronizerError {
    #[error("unsupported chain (id = {0})")]
    UnsupportedChainError(u64),
    #[error(transparent)]
    DataLoaderError(#[from] DataLoaderError),
    #[error(transparent)]
    DataLoaderConfigError(#[from] DataLoaderConfigError),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

#[async_trait]
impl<L> SynchronizerHandler<SyncOptions, SynchronizerStatus, ResetOptions> for Synchronizer<L>
where
    L: DataLoader,
{
    type Error = SynchronizerError;

    async fn chain_synced_block(&self, chain_id: u64) -> Result<Option<u64>, Self::Error> {
        let loader = self
            .loaders
            .get(&chain_id)
            .ok_or(SynchronizerError::UnsupportedChainError(chain_id))?;
        let result = loader.chain_loaded_block(chain_id).await?;
        Ok(result)
    }

    async fn contract_synced_block(&self, chain_id: u64, contract_address: &str) -> Result<Option<u64>, Self::Error> {
        let loader = self
            .loaders
            .get(&chain_id)
            .ok_or(SynchronizerError::UnsupportedChainError(chain_id))?;
        let result = loader.contract_loaded_block(chain_id, contract_address).await?;
        Ok(result)
    }

    async fn status(&self, with_contracts: bool) -> Result<SynchronizerStatus, Self::Error> {
        let tasks = self
            .loaders
            .iter()
            .map(|(chain_id, loader)| self.chain_status(chain_id, loader, with_contracts))
            .collect::<Vec<_>>();
        let chains = futures::future::try_join_all(tasks).await?;
        Ok(SynchronizerStatus::builder().chains(chains).build())
    }

    async fn sync(&self, sync_option: SyncOptions) -> Result<SynchronizerStatus, Self::Error> {
        let chains = if sync_option.chain_ids.is_empty() {
            self.loaders.keys().copied().collect::<Vec<_>>()
        } else {
            sync_option.chain_ids.clone()
        };

        let mut loader_tasks = Vec::new();
        for chain_id in chains {
            let loader = self
                .loaders
                .get(&chain_id)
                .ok_or_else(|| Self::Error::UnsupportedChainError(chain_id))?;
            loader_tasks.push(self.chain_sync(loader, &sync_option));
        }

        let results = futures::future::join_all(loader_tasks).await;
        let mut chains_status = vec![];
        for result in results {
            match result {
                Ok(status) => chains_status.push(status),
                Err(e) => return Err(e),
            }
        }
        Ok(SynchronizerStatus::builder().chains(chains_status).build())
    }

    async fn reset(&self, reset_options: ResetOptions) -> Result<(), Self::Error> {
        let loaders = self
            .loaders
            .iter()
            .filter_map(|(chain_id, loader)| {
                reset_options
                    .chains
                    .iter()
                    .find(|chain| chain.chain_id == *chain_id)
                    .map(|options| (loader, options))
            })
            .collect::<Vec<_>>();

        let tasks = loaders.iter().map(|(loader, options)| {
            let reset_options = DataLoaderResetOptions::builder()
                .chain_id(options.chain_id)
                .contract_addresses(options.contract_addresses.clone())
                .block_number(options.block_number)
                .build();
            loader.reset(reset_options)
        });
        let _ = futures::future::try_join_all(tasks).await?;
        Ok(())
    }
}

#[async_trait]
impl<F, S> FromContext<F, S> for Synchronizer<ChainDataLoader<FullData>>
where
    F: StatementFormatter + 'static,
    S: Storage + 'static,
{
    async fn from_context(context: &MystikoContext<F, S>) -> Result<Self, MystikoError> {
        let options = SynchronizerOptions::builder()
            .mystiko_config(context.config.clone())
            .providers(context.providers.clone())
            .db(context.db.clone())
            .loader_config(context.loader_config.clone())
            .build();
        Ok(Self::new(options).await?)
    }
}

impl Synchronizer<ChainDataLoader<FullData>> {
    pub async fn new<F, S>(options: SynchronizerOptions<F, S>) -> Result<Self, SynchronizerError>
    where
        F: StatementFormatter + 'static,
        S: Storage + 'static,
    {
        let collection = options.db.collection();
        let loader_handle = DatabaseHandler::<FullData, F, S, Contract, Commitment, Nullifier>::builder()
            .config(options.mystiko_config.clone())
            .collection(collection)
            .build();
        loader_handle.initialize().await?;
        let sync_handler = SyncLoaderHandler::builder()
            .db(options.db.clone())
            .raw(loader_handle)
            .build();
        let handler = Arc::new(Box::new(sync_handler) as Box<dyn DataHandler<FullData>>);

        let tasks = options.mystiko_config.chains().into_iter().map(|chain_cfg| {
            let chain_id = chain_cfg.chain_id();
            let loader_config = options
                .loader_config
                .clone()
                .unwrap_or_else(|| build_default_wallet_loader_config(chain_cfg));
            let option = LoaderConfigOptions::builder()
                .chain_id(chain_id)
                .config(loader_config.clone())
                .mystiko_config(options.mystiko_config.clone())
                .providers(options.providers.clone())
                .handler(handler.clone())
                .build();
            async move {
                let loader = ChainDataLoader::from_config(&option).await?;
                Ok((chain_id, loader)) as Result<_, SynchronizerError>
            }
        });
        let results = futures::future::try_join_all(tasks).await?;
        let loaders = results.into_iter().collect::<HashMap<_, _>>();
        Ok(Self::builder()
            .mystiko_config(options.mystiko_config)
            .loaders(loaders)
            .build())
    }
}

impl<L> Synchronizer<L>
where
    L: DataLoader,
{
    async fn chain_status(
        &self,
        chain_id: &u64,
        loader: &L,
        with_contracts: bool,
    ) -> Result<ChainStatus, SynchronizerError> {
        let chain_sync_block = loader.chain_loaded_block(*chain_id).await?.unwrap_or_default();
        let chain_target_block = loader.chain_target_block(*chain_id).await?.unwrap_or_default();
        let mut chain_status = ChainStatus::builder()
            .chain_id(*chain_id)
            .synced_block(chain_sync_block)
            .target_block(chain_target_block)
            .build();
        if with_contracts {
            let chain_cfg = self
                .mystiko_config
                .find_chain(*chain_id)
                .ok_or_else(|| SynchronizerError::UnsupportedChainError(*chain_id))?;
            let tasks: Vec<_> = chain_cfg
                .pool_contracts()
                .iter()
                .map(|pool| self.contract_status(chain_id, loader, pool.address()))
                .collect();
            let contracts = futures::future::try_join_all(tasks).await?;
            chain_status.contracts = contracts;
        }
        Ok(chain_status)
    }

    async fn contract_status(
        &self,
        chain_id: &u64,
        loader: &L,
        contract_addr: &str,
    ) -> Result<ContractStatus, SynchronizerError> {
        let contract_sync_block = loader
            .contract_loaded_block(*chain_id, contract_addr)
            .await?
            .unwrap_or_default();
        let contract_status = ContractStatus::builder()
            .contract_address(contract_addr)
            .synced_block(contract_sync_block)
            .build();
        Ok(contract_status)
    }

    async fn chain_sync(&self, loader: &L, sync_option: &SyncOptions) -> Result<ChainStatus, SynchronizerError> {
        let load_option = self.build_load_option(sync_option);
        let result = loader.load(load_option).await;
        match result {
            Ok(status) => Ok(ChainStatus::builder()
                .chain_id(status.chain_id)
                .synced_block(status.loaded_block)
                .target_block(status.target_block)
                .build()),
            Err(err) => Err(err.into()),
        }
    }

    fn build_load_option(&self, sync_option: &SyncOptions) -> LoadOption {
        let fetcher_option = self.build_load_fetcher_option(sync_option);
        let validator_option = self.build_load_validator_option(sync_option);
        LoadOption::builder()
            .fetcher(fetcher_option)
            .validator(validator_option)
            .build()
    }

    fn build_load_fetcher_option(&self, sync_option: &SyncOptions) -> LoadFetcherOption {
        let packer_fetcher_option = LoadFetcherSkipOption::builder()
            .skip_fetch(sync_option.disable_datapacker_fetcher)
            .skip_validation(sync_option.enable_datapacker_fetcher_validate.map(|enabled| !enabled))
            .build();
        let sequencer_fetcher_option = LoadFetcherSkipOption::builder()
            .skip_fetch(sync_option.disable_sequencer_fetcher)
            .skip_validation(sync_option.enable_sequencer_fetcher_validate.map(|enabled| !enabled))
            .build();
        let provider_fetcher_option = LoadFetcherSkipOption::builder()
            .skip_fetch(sync_option.disable_provider_fetcher)
            .skip_validation(sync_option.disable_provider_fetcher_validate)
            .build();

        let mut fetcher_skips = HashMap::new();
        fetcher_skips.insert(PACKER_FETCHER_NAME.to_string(), packer_fetcher_option);
        fetcher_skips.insert(SEQUENCER_FETCHER_NAME.to_string(), sequencer_fetcher_option);
        fetcher_skips.insert(PROVIDER_FETCHER_NAME.to_string(), provider_fetcher_option);

        let mut options = LoadFetcherOption::builder().skips(fetcher_skips).build();
        if let Some(timeout) = sync_option.fetcher_fetch_timeout_ms {
            options.fetch_timeout_ms = timeout;
        }
        if let Some(timeout) = sync_option.fetcher_query_loaded_block_timeout_ms {
            options.query_loaded_block_timeout_ms = timeout;
        }
        options
    }

    fn build_load_validator_option(&self, sync_option: &SyncOptions) -> LoadValidatorOption {
        let mut skip_checkers = HashMap::new();
        if let Some(disabled) = sync_option.disable_rule_validator_integrity_check {
            skip_checkers.insert(RULE_INTEGRITY_CHECKER_NAME.to_string(), disabled);
        }
        if let Some(disabled) = sync_option.disable_rule_validator_sequence_check {
            skip_checkers.insert(RULE_SEQUENCE_CHECKER_NAME.to_string(), disabled);
        }
        if let Some(disabled) = sync_option.disable_rule_validator_counter_check {
            skip_checkers.insert(RULE_COUNTER_CHECKER_NAME.to_string(), disabled);
        }
        if let Some(disabled) = sync_option.disable_rule_validator_tree_check {
            skip_checkers.insert(RULE_MERKLE_TREE_CHECKER_NAME.to_string(), disabled);
        }

        let rule_validator_option = LoadValidatorSkipOption::builder()
            .skip_validation(sync_option.disable_rule_validator)
            .skip_checkers(skip_checkers)
            .build();
        let validator_skips = HashMap::from([(RULE_VALIDATOR_NAME.to_string(), rule_validator_option)]);
        let mut options = LoadValidatorOption::builder().skips(validator_skips).build();
        if let Some(concurrency) = sync_option.validator_validate_concurrency {
            options.concurrency = concurrency as usize;
        }
        options
    }
}

fn build_default_wallet_loader_config(chain_config: &ChainConfig) -> LoaderConfig {
    let fetchers = HashMap::from([
        (1, FetcherType::Packer as i32),
        (2, FetcherType::Sequencer as i32),
        (3, FetcherType::Provider as i32),
    ]);

    let provider_chain_config = HashMap::from([(
        chain_config.chain_id(),
        ProviderFetcherChainConfig::builder()
            .delay_num_blocks(chain_config.event_delay_blocks() * DEFAULT_PROVIDER_FETCHER_DELAY_BLOCK_MULTIPLE)
            .build(),
    )]);

    let fetcher_config = FetcherConfig::builder()
        .packer(PackerFetcherConfig::builder().skip_validation(true).build())
        .sequencer(SequencerFetcherConfig::builder().skip_validation(true).build())
        .provider(ProviderFetcherConfig::builder().chains(provider_chain_config).build())
        .build();

    let validators = HashMap::from([(1, ValidatorType::Rule as i32)]);
    let validator_config = ValidatorConfig::builder()
        .rule({
            let checkers = HashMap::from([
                (1, RuleValidatorCheckerType::Integrity as i32),
                (2, RuleValidatorCheckerType::Sequence as i32),
                (3, RuleValidatorCheckerType::Counter as i32),
                (4, RuleValidatorCheckerType::Tree as i32),
            ]);
            RuleValidatorConfig::builder().checkers(checkers).build()
        })
        .build();
    LoaderConfig::builder()
        .fetchers(fetchers)
        .fetcher_config(fetcher_config)
        .validators(validators)
        .validator_config(validator_config)
        .build()
}
