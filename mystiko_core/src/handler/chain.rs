use crate::error::MystikoError;
use crate::handler::contract::{to_document_contract, ContractHandler};
use crate::types::Result;
use async_trait::async_trait;
use ethers_providers::Quorum;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_config::wrapper::provider::ProviderConfig;
use mystiko_database::database::Database;
use mystiko_database::document::chain::{Chain, ChainColumn, Provider};
use mystiko_database::document::contract::Contract;
use mystiko_ethers::provider::factory::{ProvidersOptions, HTTP_REGEX, WS_REGEX};
use mystiko_ethers::provider::pool::ChainProvidersOptions;
use mystiko_ethers::provider::types::{ProviderOptions, QuorumProviderOptions};
use mystiko_protos::core::document::v1::Chain as ProtoChain;
use mystiko_protos::core::document::v1::Provider as ProtoProvider;
use mystiko_protos::core::handler::v1::{UpdateChainOptions, UpdateProviderOptions};
use mystiko_protos::storage::v1::{QueryFilter, SubFilter};
use mystiko_storage::document::Document;
use mystiko_storage::formatter::types::StatementFormatter;
use mystiko_storage::storage::Storage;
use mystiko_types::ProviderType;
use std::sync::Arc;
use std::time::Duration;

pub const DEFAULT_PROVIDER_TIMEOUT_MS: u32 = 5000;
pub const DEFAULT_PROVIDER_MAX_TRY_COUNT: u32 = 2;
pub const DEFAULT_PROVIDER_QUORUM_WEIGHT: u32 = 1;

#[derive(Debug)]
pub struct ChainHandler<F: StatementFormatter, S: Storage> {
    db: Arc<Database<F, S>>,
    config: Arc<MystikoConfig>,
    contracts: ContractHandler<F, S>,
}
impl<F, S> ChainHandler<F, S>
where
    F: StatementFormatter,
    S: Storage,
{
    pub fn new(db: Arc<Database<F, S>>, config: Arc<MystikoConfig>) -> Self {
        Self {
            db: db.clone(),
            config: config.clone(),
            contracts: ContractHandler::new(db, config),
        }
    }

    pub async fn find<Q: Into<QueryFilter>>(&self, query_filter: Q) -> Result<Vec<ProtoChain>> {
        let documents = self
            .db
            .chains
            .find(query_filter)
            .await
            .map_err(MystikoError::StorageError)?;
        Ok(documents.into_iter().map(to_proto_chain).collect())
    }

    pub async fn find_all(&self) -> Result<Vec<ProtoChain>> {
        let documents = self.db.chains.find_all().await.map_err(MystikoError::StorageError)?;
        Ok(documents.into_iter().map(to_proto_chain).collect())
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<ProtoChain>> {
        Ok(self
            .db
            .chains
            .find_by_id(id)
            .await
            .map_err(MystikoError::StorageError)?
            .map(to_proto_chain))
    }

    pub async fn find_by_chain_id(&self, chain_id: u64) -> Result<Option<ProtoChain>> {
        Ok(self
            .db
            .chains
            .find_one(SubFilter::equal(ChainColumn::ChainId, chain_id))
            .await
            .map_err(MystikoError::StorageError)?
            .map(to_proto_chain))
    }

    pub async fn count<Q: Into<QueryFilter>>(&self, query_filter: Q) -> Result<u64> {
        self.db
            .chains
            .count(query_filter)
            .await
            .map_err(MystikoError::StorageError)
    }

    pub async fn count_all(&self) -> Result<u64> {
        self.db.chains.count_all().await.map_err(MystikoError::StorageError)
    }

    pub async fn initialize(&self) -> Result<Vec<ProtoChain>> {
        let mut insert_chains: Vec<Chain> = vec![];
        let mut update_chains: Vec<Document<Chain>> = vec![];
        let mut chains: Vec<Document<Chain>> = vec![];
        for chain_config in self.config.chains() {
            if let Some(mut existing_chain) = self.find_by_chain_id(chain_config.chain_id()).await? {
                if !existing_chain.name_override {
                    existing_chain.name = chain_config.name().to_string();
                }
                if !existing_chain.provider_override {
                    existing_chain.providers = convert_providers(&chain_config.providers())
                        .iter()
                        .map(|provider| provider.clone().into())
                        .collect();
                }
                update_chains.push(to_document_chain(existing_chain));
            } else {
                insert_chains.push(Chain {
                    chain_id: chain_config.chain_id(),
                    name: chain_config.name().to_string(),
                    name_override: false,
                    providers: convert_providers(&chain_config.providers()),
                    provider_override: false,
                    synced_block_number: 0,
                });
            }
        }
        chains.extend(
            self.db
                .chains
                .insert_batch(&insert_chains)
                .await
                .map_err(MystikoError::StorageError)?,
        );
        chains.extend(
            self.db
                .chains
                .update_batch(&update_chains)
                .await
                .map_err(MystikoError::StorageError)?,
        );
        Ok(chains.iter().map(|chain| to_proto_chain(chain.clone())).collect())
    }

    pub async fn reset_name_and_providers(&self, chain_id: u64) -> Result<Option<ProtoChain>> {
        if let (Some(chain_config), Some(existing_chain)) =
            (self.config.find_chain(chain_id), self.find_by_chain_id(chain_id).await?)
        {
            let mut existing_chain = to_document_chain(existing_chain);
            existing_chain.data.name = chain_config.name().to_string();
            existing_chain.data.name_override = false;
            existing_chain.data.providers = convert_providers(&chain_config.providers());
            existing_chain.data.provider_override = false;
            Ok(Some(to_proto_chain(
                self.db
                    .chains
                    .update(&existing_chain)
                    .await
                    .map_err(MystikoError::StorageError)?,
            )))
        } else {
            Ok(None)
        }
    }

    pub async fn update_by_id(&self, id: &str, options: &UpdateChainOptions) -> Result<Option<ProtoChain>> {
        let existing_chain = self.find_by_id(id).await?.map(to_document_chain);
        match self.update(existing_chain, options).await? {
            None => Ok(None),
            Some(chain) => Ok(Some(to_proto_chain(chain))),
        }
    }

    pub async fn update_by_chain_id(&self, chain_id: u64, options: &UpdateChainOptions) -> Result<Option<ProtoChain>> {
        let existing_chain = self.find_by_chain_id(chain_id).await?.map(to_document_chain);
        match self.update(existing_chain, options).await? {
            None => Ok(None),
            Some(chain) => Ok(Some(to_proto_chain(chain))),
        }
    }

    pub async fn reset_synced_block(&self, chain_id: u64) -> Result<Option<ProtoChain>> {
        self.rs_synced_block(chain_id, None).await
    }

    pub async fn reset_synced_block_to(&self, chain_id: u64, to_block: u64) -> Result<Option<ProtoChain>> {
        self.rs_synced_block(chain_id, Some(to_block)).await
    }

    async fn update(
        &self,
        existing_chain: Option<Document<Chain>>,
        options: &UpdateChainOptions,
    ) -> Result<Option<Document<Chain>>> {
        if let Some(mut existing_chain) = existing_chain {
            if let Some(chain_config) = self.config.find_chain(existing_chain.data.chain_id) {
                let new_providers = &options.providers;
                if !new_providers.is_empty() {
                    for new_provider in new_providers {
                        if !HTTP_REGEX.is_match(&new_provider.url) && !WS_REGEX.is_match(&new_provider.url) {
                            return Err(MystikoError::InvalidProviderUrlError(new_provider.url.clone()));
                        }
                    }
                }
                let mut has_update = false;
                if let Some(new_name) = &options.name {
                    if !new_name.is_empty() && new_name != &existing_chain.data.name {
                        existing_chain.data.name = new_name.to_string();
                        existing_chain.data.name_override = true;
                        has_update = true;
                    }
                }
                let update_provider_options = &options.providers;
                if !update_provider_options.is_empty()
                    && !compare_providers(update_provider_options, &existing_chain.data.providers)
                {
                    existing_chain.data.providers = wrap_providers(
                        update_provider_options,
                        &chain_config.providers(),
                        &existing_chain.data.providers,
                    );
                    existing_chain.data.provider_override = true;
                    has_update = true;
                }
                return if has_update {
                    Ok(Some(
                        self.db
                            .chains
                            .update(&existing_chain)
                            .await
                            .map_err(MystikoError::StorageError)?,
                    ))
                } else {
                    Ok(Some(existing_chain))
                };
            }
        }
        Ok(None)
    }

    async fn rs_synced_block(&self, chain_id: u64, to_block: Option<u64>) -> Result<Option<ProtoChain>> {
        if let (Some(chain_config), Some(chain)) =
            (self.config.find_chain(chain_id), self.find_by_chain_id(chain_id).await?)
        {
            let mut chain = to_document_chain(chain);
            chain.data.synced_block_number = to_block.unwrap_or(0);
            let updated_chain = self
                .db
                .chains
                .update(&chain)
                .await
                .map_err(MystikoError::StorageError)?;
            let mut contracts: Vec<Document<Contract>> = Vec::new();
            for contract_config in chain_config.contracts_with_disabled().iter() {
                if let Some(contract) = self
                    .contracts
                    .find_by_address(chain_id, contract_config.address())
                    .await?
                {
                    let mut contract = to_document_contract(contract);
                    contract.data.synced_block_number = to_block.unwrap_or(contract_config.start_block());
                    contracts.push(contract);
                }
            }
            self.db
                .contracts
                .update_batch(&contracts)
                .await
                .map_err(MystikoError::StorageError)?;
            Ok(Some(to_proto_chain(updated_chain)))
        } else {
            Ok(None)
        }
    }
}
#[async_trait]
impl<F, S> ChainProvidersOptions for ChainHandler<F, S>
where
    F: StatementFormatter,
    S: Storage,
{
    async fn providers_options(&self, chain_id: u64) -> anyhow::Result<Option<ProvidersOptions>> {
        if let (Some(chain_config), Some(chain)) =
            (self.config.find_chain(chain_id), self.find_by_chain_id(chain_id).await?)
        {
            let chain = to_document_chain(chain);
            let mut providers_options: Vec<ProviderOptions> = vec![];
            for provider_config in chain.data.providers {
                let provider_options = ProviderOptions::builder()
                    .url(provider_config.url.clone())
                    .quorum_weight(provider_config.quorum_weight as u64)
                    .timeout_retries(provider_config.max_try_count - 1)
                    .rate_limit_retries(provider_config.max_try_count - 1)
                    .request_timeout(Duration::from_millis(provider_config.timeout_ms as u64))
                    .build();
                providers_options.push(provider_options);
            }
            match chain_config.provider_type() {
                ProviderType::Failover => Ok(Some(ProvidersOptions::Failover(providers_options))),
                ProviderType::Quorum => {
                    let quorum_options = QuorumProviderOptions::builder()
                        .quorum(Quorum::Percentage(chain_config.provider_quorum_percentage()))
                        .build();
                    Ok(Some(ProvidersOptions::Quorum(providers_options, quorum_options)))
                }
            }
        } else {
            Ok(None)
        }
    }
}

fn compare_providers(update_provider_options: &[UpdateProviderOptions], previous_providers: &[Provider]) -> bool {
    if update_provider_options.len() == previous_providers.len() {
        for index in 0..previous_providers.len() {
            if update_provider_options[index].url != previous_providers[index].url
                || update_provider_options[index]
                    .timeout_ms
                    .unwrap_or(previous_providers[index].timeout_ms)
                    != previous_providers[index].timeout_ms
                || update_provider_options[index]
                    .max_try_count
                    .unwrap_or(previous_providers[index].max_try_count)
                    != previous_providers[index].max_try_count
                || update_provider_options[index]
                    .quorum_weight
                    .unwrap_or(previous_providers[index].quorum_weight)
                    != previous_providers[index].quorum_weight
            {
                return false;
            }
        }
        return true;
    }
    false
}

fn wrap_providers(
    update_provider_options: &[UpdateProviderOptions],
    providers_config: &[&ProviderConfig],
    previous_providers: &[Provider],
) -> Vec<Provider> {
    update_provider_options
        .iter()
        .map(|options| {
            let previous_provider = previous_providers.iter().find(|p| p.url == options.url);
            let provider_config = providers_config.iter().find(|p| p.url() == options.url);
            Provider {
                url: options.url.clone(),
                timeout_ms: options
                    .timeout_ms
                    .or(previous_provider.map(|p| p.timeout_ms))
                    .or(provider_config.map(|p| p.timeout_ms()))
                    .unwrap_or(DEFAULT_PROVIDER_TIMEOUT_MS),
                max_try_count: options
                    .max_try_count
                    .or(previous_provider.map(|p| p.max_try_count))
                    .or(provider_config.map(|p| p.max_try_count()))
                    .unwrap_or(DEFAULT_PROVIDER_MAX_TRY_COUNT),
                quorum_weight: options
                    .quorum_weight
                    .or(previous_provider.map(|p| p.quorum_weight))
                    .or(provider_config.map(|p| p.quorum_weight()))
                    .unwrap_or(DEFAULT_PROVIDER_QUORUM_WEIGHT),
            }
        })
        .collect()
}

fn convert_providers(providers_config: &[&ProviderConfig]) -> Vec<Provider> {
    providers_config
        .iter()
        .map(|provider_config| Provider {
            url: provider_config.url().to_string(),
            timeout_ms: provider_config.timeout_ms(),
            max_try_count: provider_config.max_try_count(),
            quorum_weight: provider_config.quorum_weight(),
        })
        .collect()
}

fn to_proto_chain(document: Document<Chain>) -> ProtoChain {
    let providers = document
        .data
        .providers
        .iter()
        .map(|provider| provider.clone().into())
        .collect::<Vec<ProtoProvider>>();
    ProtoChain::builder()
        .id(document.id)
        .created_at(document.created_at)
        .updated_at(document.updated_at)
        .chain_id(document.data.chain_id)
        .name(document.data.name)
        .name_override(document.data.name_override)
        .providers(providers)
        .provider_override(document.data.provider_override)
        .synced_block_number(document.data.synced_block_number)
        .build()
}

pub fn to_document_chain(chain: ProtoChain) -> Document<Chain> {
    let providers = chain
        .providers
        .iter()
        .map(|provider| Provider {
            url: provider.url.clone(),
            timeout_ms: provider.timeout_ms,
            max_try_count: provider.max_try_count,
            quorum_weight: provider.quorum_weight,
        })
        .collect::<Vec<Provider>>();
    Document::new(
        chain.id,
        chain.created_at,
        chain.updated_at,
        Chain {
            chain_id: chain.chain_id,
            name: chain.name,
            name_override: chain.name_override,
            providers,
            provider_override: chain.provider_override,
            synced_block_number: chain.synced_block_number,
        },
    )
}
