use crate::data::chain::ChainData;
use crate::data::types::LoadedData;
use crate::handler::types::Result;
use crate::handler::types::{CommitmentQueryOption, DataHandler, HandleOption, HandleResult, NullifierQueryOption};
use async_trait::async_trait;
use mystiko_config::wrapper::contract::ContractConfig;
use mystiko_protos::data::v1::{Commitment, Nullifier};
use mystiko_utils::convert::bytes_to_biguint;
use std::cmp::min;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use typed_builder::TypedBuilder;

#[derive(Debug)]
pub struct CachedDataHandler<R: LoadedData, H = Box<dyn DataHandler<R>>> {
    handler: Arc<H>,
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
        self.handler.query_loading_contracts(chain_id).await
    }

    async fn query_chain_loaded_block(&self, chain_id: u64) -> Result<Option<u64>> {
        self.handler.query_chain_loaded_block(chain_id).await
    }

    async fn query_contract_loaded_block(&self, chain_id: u64, contract_address: &str) -> Result<Option<u64>> {
        self.handler
            .query_contract_loaded_block(chain_id, contract_address)
            .await
    }

    async fn query_commitment(&self, option: &CommitmentQueryOption) -> Result<Option<Commitment>> {
        self.handler.query_commitment(option).await
    }
    async fn query_commitments(&self, option: &CommitmentQueryOption) -> Result<Vec<Commitment>> {
        if let Some(contract_loaded_block) = self
            .query_contract_loaded_block(option.chain_id, &option.contract_address)
            .await?
        {
            let identifier = commitment_cache_identifier(option);
            let end_block = min(contract_loaded_block, option.end_block);
            let mut handler_option = CommitmentQueryOption::builder()
                .chain_id(option.chain_id)
                .contract_address(option.contract_address.clone())
                .end_block(end_block)
                .build();
            let mut commitments = vec![];
            if let Some(contract_cache) = self.commitments.read().await.get(&identifier) {
                if contract_cache.loaded_block >= end_block {
                    return Ok(filter_commitment(option, contract_cache));
                }
                commitments.extend(contract_cache.data.clone());
                handler_option.start_block = Some(contract_cache.loaded_block + 1);
            }
            commitments.extend(self.handler.query_commitments(&handler_option).await?);
            commitments.sort_by_key(|c| bytes_to_biguint(&c.commitment_hash));
            let contract_cache = ContractCache::builder()
                .loaded_block(end_block)
                .data(commitments)
                .build();
            let result = filter_commitment(option, &contract_cache);
            self.commitments.write().await.insert(identifier, contract_cache);
            Ok(result)
        } else {
            Ok(vec![])
        }
    }

    async fn count_commitments(&self, option: &CommitmentQueryOption) -> Result<u64> {
        self.handler.count_commitments(option).await
    }

    async fn query_nullifier(&self, option: &NullifierQueryOption) -> Result<Option<Nullifier>> {
        self.handler.query_nullifier(option).await
    }

    async fn query_nullifiers(&self, option: &NullifierQueryOption) -> Result<Vec<Nullifier>> {
        todo!()
    }

    async fn count_nullifiers(&self, option: &NullifierQueryOption) -> Result<u64> {
        self.handler.count_nullifiers(option).await
    }

    async fn handle(&self, data: &ChainData<R>, option: &HandleOption) -> HandleResult {
        self.handler.handle(data, option).await
    }
}

impl<R, H> CachedDataHandler<R, H>
where
    R: LoadedData,
    H: DataHandler<R>,
{
    pub fn new(handler: Arc<H>) -> Self {
        Self {
            handler,
            commitments: RwLock::default(),
            nullifiers: RwLock::default(),
            _phantom: Default::default(),
        }
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
        if commitment.block_number > options.end_block {
            break;
        }
        if let Some(start_block) = options.start_block {
            if commitment.block_number < start_block {
                continue;
            }
        }
        if let Some(status) = options.status {
            if commitment.status != status as i32 {
                continue;
            }
        }
        if let Some(commitment_hashes) = &options.commitment_hash {
            if !commitment_hashes.is_empty()
                && !commitment_hashes.contains(&bytes_to_biguint(&commitment.commitment_hash))
            {
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
            break;
        }
        if let Some(start_block) = options.start_block {
            if nullifier.block_number < start_block {
                continue;
            }
        }
        if let Some(nullifier_hashes) = &options.nullifier {
            if !nullifier_hashes.is_empty() && !nullifier_hashes.contains(&bytes_to_biguint(&nullifier.nullifier)) {
                continue;
            }
        }
        nullifiers.push(nullifier.clone());
    }
    nullifiers
}
