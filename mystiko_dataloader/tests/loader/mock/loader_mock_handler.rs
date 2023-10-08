use async_trait::async_trait;
use mystiko_config::{ContractConfig, MystikoConfig};
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::data::LoadedData;
use mystiko_dataloader::data::{ChainResult, ContractResult};
use mystiko_dataloader::handler::{
    CommitmentQueryOption, DataHandler, HandleOption, HandleResult, NullifierQueryOption, QueryResult,
    Result as HandlerErrorResult,
};
use mystiko_protos::data::v1::{Commitment, Nullifier};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

//
// mock! {
//     #[derive(Debug)]
//     pub ChainDataHandler<R>
//     where
//         R: LoadedData,
//     {}
//
//     #[async_trait]
//     impl<R> DataHandler<R> for ChainDataHandler<R>
//     where
//         R: LoadedData,
//     {
//         async fn query_loading_contracts(&self, chain_id: u64) -> HandlerErrorResult<Option<Vec<ContractConfig>>>;
//         async fn query_chain_loaded_block(&self, chain_id: u64) -> HandlerErrorResult<Option<u64>>;
//         async fn query_contract_loaded_block(&self, chain_id: u64, contract_address: &str) -> HandlerErrorResult<Option<u64>>;
//         async fn query_commitments(&self, option: &CommitmentQueryOption) -> HandlerErrorResult<QueryResult<Vec<Commitment>>>;
//         async fn count_commitments(&self, option: &CommitmentQueryOption) -> HandlerErrorResult<QueryResult<u64>>;
//         async fn query_nullifiers(&self, option: &NullifierQueryOption) -> HandlerErrorResult<QueryResult<Vec<Nullifier>>>;
//         async fn count_nullifiers(&self, option: &NullifierQueryOption) -> HandlerErrorResult<QueryResult<u64>>;
//         async fn handle(&self, data: &ChainData<R>, option: &HandleOption) -> HandleResult;
//     }
// }

#[derive(Debug)]
pub(crate) struct MockHandler<R>
where
    R: LoadedData,
{
    contracts: RwLock<Vec<ContractConfig>>,
    chain_loaded_blocks_error: RwLock<bool>,
    contract_loaded_blocks_error: RwLock<HashMap<String, bool>>,
    all_success: RwLock<bool>,
    result: RwLock<HandleResult>,
    data: RwLock<HashMap<String, ContractData<R>>>,
}

impl<R> Default for MockHandler<R>
where
    R: LoadedData + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<R> MockHandler<R>
where
    R: LoadedData + Clone,
{
    pub fn new() -> Self {
        MockHandler {
            contracts: RwLock::new(vec![]),
            chain_loaded_blocks_error: RwLock::new(false),
            contract_loaded_blocks_error: RwLock::new(HashMap::new()),
            all_success: RwLock::new(true),
            result: RwLock::new(HandleResult::Err(anyhow::Error::msg("handle error".to_string()).into())),
            data: RwLock::new(HashMap::new()),
        }
    }

    pub async fn set_contracts(&self, chain_id: u64, contracts: HashSet<&str>, core_cfg: Arc<MystikoConfig>) {
        let c = core_cfg
            .find_chain(chain_id)
            .unwrap()
            .contracts()
            .into_iter()
            .filter(|c| contracts.contains(c.address()))
            .collect::<Vec<_>>();

        *self.contracts.write().await = c;
    }

    pub async fn set_all_success(&self) {
        *self.all_success.write().await = true;
    }

    pub async fn set_result(&self, result: HandleResult) {
        *self.all_success.write().await = false;
        *self.result.write().await = result;
    }

    pub async fn drain_data(&self) -> HashMap<String, ContractData<R>> {
        let mut data_write = self.data.write().await;
        let mut drained_data = HashMap::new();
        data_write.drain().for_each(|(k, v)| {
            drained_data.insert(k, v);
        });
        drained_data
    }
}

#[async_trait]
impl<R> DataHandler<R> for MockHandler<R>
where
    R: LoadedData,
{
    async fn query_loading_contracts(&self, _chain_id: u64) -> HandlerErrorResult<Option<Vec<ContractConfig>>> {
        if self.contracts.read().await.is_empty() {
            Ok(None)
        } else {
            Ok(Some(self.contracts.read().await.clone()))
        }
    }

    async fn query_chain_loaded_block(&self, _chain_id: u64) -> HandlerErrorResult<Option<u64>> {
        if *self.chain_loaded_blocks_error.read().await {
            return Err(anyhow::Error::msg("error".to_string()).into());
        }

        let data = self.data.read().await;
        let min_end_block = data.iter().map(|(_, v)| v.end_block).min();

        Ok(min_end_block)
    }

    async fn query_contract_loaded_block(
        &self,
        _chain_id: u64,
        contract_address: &str,
    ) -> HandlerErrorResult<Option<u64>> {
        if let Some(blocks) = self.contract_loaded_blocks_error.read().await.get(contract_address) {
            if *blocks {
                return Err(anyhow::Error::msg("error".to_string()).into());
            }
        }

        let end_block = self.data.read().await.get(contract_address).map(|d| d.end_block);

        Ok(end_block)
    }

    async fn query_commitments(
        &self,
        _option: &CommitmentQueryOption,
    ) -> HandlerErrorResult<QueryResult<Vec<Commitment>>> {
        Err(anyhow::Error::msg("query_commitments error".to_string()).into())
    }

    async fn count_commitments(&self, option: &CommitmentQueryOption) -> HandlerErrorResult<QueryResult<u64>> {
        let query_result = self.query_commitments(option).await?;
        Ok(QueryResult::builder()
            .end_block(query_result.end_block)
            .result(query_result.result.len() as u64)
            .build())
    }

    async fn query_nullifiers(
        &self,
        _option: &NullifierQueryOption,
    ) -> HandlerErrorResult<QueryResult<Vec<Nullifier>>> {
        Err(anyhow::Error::msg("query_commitments error".to_string()).into())
    }

    async fn count_nullifiers(&self, option: &NullifierQueryOption) -> HandlerErrorResult<QueryResult<u64>> {
        let query_result = self.query_nullifiers(option).await?;
        Ok(QueryResult::builder()
            .end_block(query_result.end_block)
            .result(query_result.result.len() as u64)
            .build())
    }

    async fn handle(&self, data: &ChainData<R>, _option: &HandleOption) -> HandleResult {
        if *self.all_success.read().await {
            let contract_results = data
                .contracts_data
                .iter()
                .map(|d| {
                    ContractResult::builder()
                        .address(d.address.clone())
                        .result(Ok(()))
                        .build()
                })
                .collect::<Vec<_>>();

            let mut data_write = self.data.write().await;
            data.contracts_data.iter().for_each(|d| {
                data_write.insert(
                    d.address.clone(),
                    ContractData::builder()
                        .address(d.address.clone())
                        .start_block(d.start_block)
                        .end_block(d.end_block)
                        .build(),
                );
            });

            Ok(ChainResult::builder()
                .chain_id(data.chain_id)
                .contract_results(contract_results)
                .build())
        } else {
            let result = self.result.read().await;
            match &*result {
                Ok(result) => {
                    let contract_results = result
                        .contract_results
                        .iter()
                        .map(|d| match d.result {
                            Ok(_) => ContractResult::builder()
                                .address(d.address.clone())
                                .result(Ok(()))
                                .build(),
                            Err(_) => ContractResult::builder()
                                .address(d.address.clone())
                                .result(Err(anyhow::Error::msg("error".to_string())))
                                .build(),
                        })
                        .collect::<Vec<_>>();

                    let mut data_write = self.data.write().await;
                    data.contracts_data.iter().for_each(|d| {
                        data_write.insert(
                            d.address.clone(),
                            ContractData::builder()
                                .address(d.address.clone())
                                .start_block(d.start_block)
                                .end_block(d.end_block)
                                .build(),
                        );
                    });

                    Ok(ChainResult::builder()
                        .chain_id(result.chain_id)
                        .contract_results(contract_results)
                        .build())
                }
                Err(_) => Err(anyhow::Error::msg("handle error".to_string()).into()),
            }
        }
    }
}
