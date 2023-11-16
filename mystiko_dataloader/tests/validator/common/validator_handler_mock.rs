use async_trait::async_trait;
use mystiko_config::ContractConfig;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::LoadedData;
use mystiko_dataloader::handler::{
    CommitmentQueryOption, DataHandler, HandleOption, HandleResult, NullifierQueryOption, QueryResult,
    Result as HandlerErrorResult,
};
use mystiko_dataloader::loader::ResetOptions;
use mystiko_protos::data::v1::{Commitment, Nullifier};
use mystiko_utils::convert::bytes_to_biguint;
use std::collections::HashSet;
use tokio::sync::RwLock;

pub struct MockChainDataHandler<R> {
    pub _phantom: std::marker::PhantomData<R>,
    pub cms: RwLock<Vec<Vec<Commitment>>>,
    pub nullifiers: RwLock<Vec<Vec<Nullifier>>>,
}

impl<R> Default for MockChainDataHandler<R>
where
    R: LoadedData + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<R> MockChainDataHandler<R>
where
    R: LoadedData + Clone,
{
    pub fn new() -> Self {
        MockChainDataHandler {
            _phantom: std::marker::PhantomData,
            cms: RwLock::new(vec![]),
            nullifiers: RwLock::new(vec![]),
        }
    }

    pub async fn add_commitments(&self, commitments: Vec<Commitment>) {
        let mut cms = self.cms.write().await;
        cms.push(commitments);
    }

    #[allow(dead_code)]
    pub async fn add_nullifiers(&self, nullifiers: Vec<Nullifier>) {
        let mut ns = self.nullifiers.write().await;
        ns.push(nullifiers);
    }
}

// pub type HandlerErrorResult<T> = anyhow::Result<T, HandlerError>;

#[async_trait]
impl<R> DataHandler<R> for MockChainDataHandler<R>
where
    R: LoadedData,
{
    async fn query_loading_contracts(&self, _chain_id: u64) -> HandlerErrorResult<Option<Vec<ContractConfig>>> {
        Ok(None)
    }

    async fn query_chain_loaded_block(&self, _chain_id: u64) -> HandlerErrorResult<Option<u64>> {
        Err(anyhow::Error::msg("query_chain_loaded_block error".to_string()).into())
    }

    async fn query_contract_loaded_block(
        &self,
        _chain_id: u64,
        _contract_address: &str,
    ) -> HandlerErrorResult<Option<u64>> {
        Err(anyhow::Error::msg("query_contract_loaded_block error".to_string()).into())
    }

    async fn query_commitments(
        &self,
        option: &CommitmentQueryOption,
    ) -> HandlerErrorResult<QueryResult<Vec<Commitment>>> {
        self.cms.write().await.pop().map_or_else(
            || Err(anyhow::Error::msg("mock query commitments error".to_string()).into()),
            |cms| match &option.commitment_hash {
                None => Ok(QueryResult::builder().end_block(option.end_block).result(cms).build()),
                Some(query_hash) => {
                    let hash_set: HashSet<_> = query_hash.iter().cloned().collect();
                    let mut result = Vec::new();

                    for h in query_hash {
                        for cm in &cms {
                            if bytes_to_biguint(&cm.commitment_hash) == *h {
                                result.push(cm.clone());
                            }
                        }
                    }

                    for cm in &cms {
                        if !hash_set.contains(&bytes_to_biguint(&cm.commitment_hash)) {
                            result.push(cm.clone());
                        }
                    }

                    Ok(QueryResult::builder()
                        .end_block(option.end_block)
                        .result(result)
                        .build())
                }
            },
        )
    }

    async fn count_commitments(&self, option: &CommitmentQueryOption) -> HandlerErrorResult<QueryResult<u64>> {
        let query_result = self.query_commitments(option).await?;
        Ok(QueryResult::builder()
            .end_block(option.end_block)
            .result(query_result.result.len() as u64)
            .build())
    }

    async fn query_nullifiers(&self, option: &NullifierQueryOption) -> HandlerErrorResult<QueryResult<Vec<Nullifier>>> {
        self.nullifiers.write().await.pop().map_or_else(
            || Err(anyhow::Error::msg("mock query nullifiers error".to_string()).into()),
            |ns| Ok(QueryResult::builder().end_block(option.end_block).result(ns).build()),
        )
    }

    async fn count_nullifiers(&self, option: &NullifierQueryOption) -> HandlerErrorResult<QueryResult<u64>> {
        let query_result = self.query_nullifiers(option).await?;
        Ok(QueryResult::builder()
            .end_block(query_result.end_block)
            .result(query_result.result.len() as u64)
            .build())
    }

    async fn handle(&self, _data: &ChainData<R>, _option: &HandleOption) -> HandleResult {
        Err(anyhow::Error::msg("handle error".to_string()).into())
    }

    async fn reset(&self, _options: &ResetOptions) -> HandlerErrorResult<()> {
        Ok(())
    }
}
