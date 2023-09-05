use crate::data::ChainData;
use crate::data::LoadedData;
use crate::handler::{CommitmentQueryOption, DataHandler, HandleOption, HandleResult, NullifierQueryOption};
use crate::handler::{QueryResult, Result};
use async_trait::async_trait;
use mystiko_config::ContractConfig;
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use typed_builder::TypedBuilder;

#[derive(Debug)]
pub struct CachedDataHandler<R: LoadedData, H = Box<dyn DataHandler<R>>> {
    pub raw: Arc<H>,
    commitments: RwLock<HashMap<String, ContractCache<Commitment>>>,
    nullifiers: RwLock<HashMap<String, ContractCache<Nullifier>>>,
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R, H> DataHandler<R> for CachedDataHandler<R, H>
where
    R: LoadedData,
    H: DataHandler<R>,
{
    async fn query_loading_contracts(&self, chain_id: u64) -> Result<Option<Vec<ContractConfig>>> {
        self.raw.query_loading_contracts(chain_id).await
    }

    async fn query_chain_loaded_block(&self, chain_id: u64) -> Result<Option<u64>> {
        self.raw.query_chain_loaded_block(chain_id).await
    }

    async fn query_contract_loaded_block(&self, chain_id: u64, contract_address: &str) -> Result<Option<u64>> {
        self.raw.query_contract_loaded_block(chain_id, contract_address).await
    }

    async fn query_commitment(&self, option: &CommitmentQueryOption) -> Result<Option<Commitment>> {
        self.raw.query_commitment(option).await
    }
    async fn query_commitments(&self, option: &CommitmentQueryOption) -> Result<QueryResult<Vec<Commitment>>> {
        let identifier = commitment_cache_identifier(option);
        self.cache_queued_commitment(&identifier, option).await?;
        let mut handler_option = CommitmentQueryOption::builder()
            .chain_id(option.chain_id)
            .contract_address(option.contract_address.clone())
            .end_block(option.end_block)
            .build();
        let mut commitments = vec![];
        if let Some(contract_cache) = self.commitments.read().await.get(&identifier) {
            if contract_cache.loaded_block >= option.end_block {
                return Ok(QueryResult::builder()
                    .end_block(option.end_block)
                    .result(filter_commitment(option, contract_cache))
                    .build());
            }
            commitments.extend(contract_cache.data.clone());
            handler_option.start_block = Some(contract_cache.loaded_block + 1);
        }
        let query_result = self.raw.query_commitments(&handler_option).await?;
        commitments.extend(query_result.result);
        commitments.sort_by_key(|c| c.commitment_hash_as_biguint());
        let contract_cache = ContractCache::builder()
            .loaded_block(query_result.end_block)
            .data(commitments)
            .build();
        let result = filter_commitment(option, &contract_cache);
        self.commitments.write().await.insert(identifier, contract_cache);
        Ok(QueryResult::builder()
            .end_block(query_result.end_block)
            .result(result)
            .build())
    }

    async fn count_commitments(&self, option: &CommitmentQueryOption) -> Result<QueryResult<u64>> {
        self.raw.count_commitments(option).await
    }

    async fn query_nullifier(&self, option: &NullifierQueryOption) -> Result<Option<Nullifier>> {
        self.raw.query_nullifier(option).await
    }

    async fn query_nullifiers(&self, option: &NullifierQueryOption) -> Result<QueryResult<Vec<Nullifier>>> {
        let identifier = nullifier_cache_identifier(option);
        let mut handler_option = NullifierQueryOption::builder()
            .chain_id(option.chain_id)
            .contract_address(option.contract_address.clone())
            .end_block(option.end_block)
            .build();
        let mut nullifiers = vec![];
        if let Some(contract_cache) = self.nullifiers.read().await.get(&identifier) {
            if contract_cache.loaded_block >= option.end_block {
                return Ok(QueryResult::builder()
                    .end_block(option.end_block)
                    .result(filter_nullifier(option, contract_cache))
                    .build());
            }
            nullifiers.extend(contract_cache.data.clone());
            handler_option.start_block = Some(contract_cache.loaded_block + 1);
        }
        let query_result = self.raw.query_nullifiers(&handler_option).await?;
        nullifiers.extend(query_result.result);
        nullifiers.sort_by_key(Nullifier::nullifier_as_biguint);
        let contract_cache = ContractCache::builder()
            .loaded_block(query_result.end_block)
            .data(nullifiers)
            .build();
        let result = filter_nullifier(option, &contract_cache);
        self.nullifiers.write().await.insert(identifier, contract_cache);
        Ok(QueryResult::builder()
            .end_block(query_result.end_block)
            .result(result)
            .build())
    }

    async fn count_nullifiers(&self, option: &NullifierQueryOption) -> Result<QueryResult<u64>> {
        self.raw.count_nullifiers(option).await
    }

    async fn handle(&self, data: &ChainData<R>, option: &HandleOption) -> HandleResult {
        self.raw.handle(data, option).await
    }
}

impl<R, H> CachedDataHandler<R, H>
where
    R: LoadedData,
    H: DataHandler<R>,
{
    pub fn new(handler: Arc<H>) -> Self {
        Self {
            raw: handler,
            commitments: RwLock::default(),
            nullifiers: RwLock::default(),
            _phantom: Default::default(),
        }
    }

    async fn cache_queued_commitment(&self, identifier: &str, option: &CommitmentQueryOption) -> Result<()> {
        let mut new_contract_cache: Option<ContractCache<Commitment>> = None;
        if let Some(contract_cache) = self.commitments.read().await.get(identifier) {
            let commitment_hashes = contract_cache
                .data
                .iter()
                .filter(|c| c.status == CommitmentStatus::Queued as i32)
                .map(|c| c.commitment_hash_as_biguint())
                .collect::<Vec<_>>();
            if !commitment_hashes.is_empty() {
                let options = CommitmentQueryOption::builder()
                    .chain_id(option.chain_id)
                    .contract_address(option.contract_address.clone())
                    .end_block(contract_cache.loaded_block)
                    .commitment_hash(commitment_hashes)
                    .build();
                let mut commitments = self
                    .raw
                    .query_commitments(&options)
                    .await?
                    .result
                    .into_iter()
                    .map(|c| (c.commitment_hash_as_biguint(), c))
                    .collect::<HashMap<_, _>>();
                let mut contract_cache = contract_cache.clone();
                contract_cache.data = contract_cache
                    .data
                    .into_iter()
                    .map(|c| {
                        if let Some(commitment) = commitments.remove(&c.commitment_hash_as_biguint()) {
                            commitment
                        } else {
                            c
                        }
                    })
                    .collect::<Vec<_>>();
                new_contract_cache = Some(contract_cache);
            }
        }
        if let Some(new_contract_cache) = new_contract_cache {
            self.commitments
                .write()
                .await
                .insert(identifier.to_string(), new_contract_cache);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
struct ContractCache<T> {
    pub(crate) loaded_block: u64,
    #[builder(default)]
    pub(crate) data: Vec<T>,
}

fn commitment_cache_identifier(options: &CommitmentQueryOption) -> String {
    format!("/chains/{}/contracts/{}", options.chain_id, options.contract_address)
}

fn nullifier_cache_identifier(options: &NullifierQueryOption) -> String {
    format!("/chains/{}/contracts/{}", options.chain_id, options.contract_address)
}

fn filter_commitment(options: &CommitmentQueryOption, cache: &ContractCache<Commitment>) -> Vec<Commitment> {
    let mut commitments = vec![];
    for commitment in cache.data.iter() {
        let mut block_number = commitment.block_number;
        if let Some(status) = options.status {
            if commitment.status != status as i32 {
                continue;
            }
            match status {
                CommitmentStatus::SrcSucceeded => {
                    block_number = commitment.src_chain_block_number.unwrap_or(block_number);
                }
                CommitmentStatus::Included => {
                    block_number = commitment.included_block_number.unwrap_or(block_number);
                }
                _ => {}
            }
        }
        if block_number > options.end_block {
            continue;
        }
        if let Some(start_block) = options.start_block {
            if block_number < start_block {
                continue;
            }
        }
        if let Some(commitment_hashes) = &options.commitment_hash {
            if !commitment_hashes.is_empty() && !commitment_hashes.contains(&commitment.commitment_hash_as_biguint()) {
                continue;
            }
        }
        commitments.push(commitment.clone());
    }
    commitments
}

fn filter_nullifier(options: &NullifierQueryOption, cache: &ContractCache<Nullifier>) -> Vec<Nullifier> {
    let mut nullifiers = vec![];
    for nullifier in cache.data.iter() {
        if nullifier.block_number > options.end_block {
            continue;
        }
        if let Some(start_block) = options.start_block {
            if nullifier.block_number < start_block {
                continue;
            }
        }
        if let Some(nullifier_hashes) = &options.nullifier {
            if !nullifier_hashes.is_empty() && !nullifier_hashes.contains(&nullifier.nullifier_as_biguint()) {
                continue;
            }
        }
        nullifiers.push(nullifier.clone());
    }
    nullifiers
}
