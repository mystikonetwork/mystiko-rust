use crate::SynchronizerHandler;
use async_trait::async_trait;
use mystiko_config::MystikoConfig;
use mystiko_dataloader::fetcher::{PACKER_FETCHER_NAME, PROVIDER_FETCHER_NAME, SEQUENCER_FETCHER_NAME};
use mystiko_dataloader::loader::{
    DataLoader, LoadFetcherOption, LoadFetcherSkipOption, LoadOption, LoadValidatorOption, LoadValidatorSkipOption,
};
use mystiko_dataloader::validator::rule::{
    RULE_COUNTER_CHECKER_NAME, RULE_INTEGRITY_CHECKER_NAME, RULE_MERKLE_TREE_CHECKER_NAME, RULE_SEQUENCE_CHECKER_NAME,
    RULE_VALIDATOR_NAME,
};
use mystiko_dataloader::DataLoaderError;
use mystiko_protos::core::synchronizer::v1::{ChainStatus, ContractStatus, SyncOptions, SynchronizerStatus};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Synchronizer<L: DataLoader> {
    mystiko_config: Arc<MystikoConfig>,
    chains: HashMap<u64, L>,
}

#[derive(Debug, Error)]
pub enum SynchronizerError {
    #[error("unsupported chain (id = {0})")]
    UnsupportedChainError(u64),
    #[error(transparent)]
    DataLoaderError(#[from] DataLoaderError),
}

#[async_trait]
impl<L> SynchronizerHandler<SyncOptions, SynchronizerStatus> for Synchronizer<L>
where
    L: DataLoader,
{
    type Error = SynchronizerError;

    async fn chain_synced_block(&self, chain_id: u64) -> Result<Option<u64>, Self::Error> {
        let loader = self
            .chains
            .get(&chain_id)
            .ok_or(SynchronizerError::UnsupportedChainError(chain_id))?;
        let result = loader.chain_loaded_block(chain_id).await?;
        Ok(result)
    }

    async fn contract_synced_block(&self, chain_id: u64, contract_address: &str) -> Result<Option<u64>, Self::Error> {
        let loader = self
            .chains
            .get(&chain_id)
            .ok_or(SynchronizerError::UnsupportedChainError(chain_id))?;
        let result = loader.contract_loaded_block(chain_id, contract_address).await?;
        Ok(result)
    }

    async fn status(&self, with_contracts: bool) -> Result<SynchronizerStatus, Self::Error> {
        let tasks = self
            .chains
            .iter()
            .map(|(chain_id, loader)| self.chain_status(chain_id, loader, with_contracts))
            .collect::<Vec<_>>();
        let chains = futures::future::try_join_all(tasks).await?;
        Ok(SynchronizerStatus::builder().chains(chains).build())
    }

    async fn sync(&self, sync_option: SyncOptions) -> Result<(), Self::Error> {
        let chains = if sync_option.chain_ids.is_empty() {
            self.chains.keys().copied().collect::<Vec<_>>()
        } else {
            sync_option.chain_ids.clone()
        };

        let loader_tasks: Vec<_> = chains
            .iter()
            .filter_map(|chain_id| {
                if let Some(loader) = self.chains.get(chain_id) {
                    Some(self.chain_sync(chain_id, loader, &sync_option))
                } else {
                    log::warn!("chain(id={:?}) not supported", chain_id);
                    None
                }
            })
            .collect();
        let _ = futures::future::join_all(loader_tasks).await;
        Ok(())
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
        let mut chain_status = ChainStatus::builder()
            .chain_id(*chain_id)
            .synced_block(chain_sync_block)
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

    async fn chain_sync(&self, chain_id: &u64, loader: &L, sync_option: &SyncOptions) {
        let load_option = self.build_load_option(sync_option);
        let result = loader.load(load_option).await;
        if let Err(err) = result {
            log::error!("chain(id={:?}) load error: {:?}", chain_id, err);
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
        fetcher_skips.insert(PACKER_FETCHER_NAME, packer_fetcher_option);
        fetcher_skips.insert(SEQUENCER_FETCHER_NAME, sequencer_fetcher_option);
        fetcher_skips.insert(PROVIDER_FETCHER_NAME, provider_fetcher_option);

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
            skip_checkers.insert(RULE_INTEGRITY_CHECKER_NAME, disabled);
        }
        if let Some(disabled) = sync_option.disable_rule_validator_sequence_check {
            skip_checkers.insert(RULE_SEQUENCE_CHECKER_NAME, disabled);
        }
        if let Some(disabled) = sync_option.disable_rule_validator_counter_check {
            skip_checkers.insert(RULE_COUNTER_CHECKER_NAME, disabled);
        }
        if let Some(disabled) = sync_option.disable_rule_validator_tree_check {
            skip_checkers.insert(RULE_MERKLE_TREE_CHECKER_NAME, disabled);
        }

        let rule_validator_option = LoadValidatorSkipOption::builder()
            .skip_validation(sync_option.disable_rule_validator)
            .skip_checkers(skip_checkers)
            .build();
        let validator_skips = HashMap::from([(RULE_VALIDATOR_NAME, rule_validator_option)]);
        let mut options = LoadValidatorOption::builder().skips(validator_skips).build();
        if let Some(concurrency) = sync_option.validator_validate_concurrency {
            options.concurrency = concurrency as usize;
        }
        options
    }
}
