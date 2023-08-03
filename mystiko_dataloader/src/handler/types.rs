use crate::data::chain::ChainData;
use crate::data::result::ChainResult;
use crate::data::types::LoadedData;
use anyhow::Result;
use async_trait::async_trait;
use mystiko_config::wrapper::contract::ContractConfig;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use std::fmt::Debug;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct HandleOption {
    pub config: Arc<MystikoConfig>,
}

pub type HandleResult = Result<ChainResult<()>>;

#[async_trait]
pub trait DataHandler<R: LoadedData>: Send + Sync {
    async fn loading_contracts(&self, _chain_id: u64) -> Result<Option<Vec<ContractConfig>>> {
        Ok(None)
    }
    async fn chain_loaded_block(&self, chain_id: u64) -> Result<Option<u64>>;
    async fn contract_loaded_block(&self, chain_id: u64, contract_address: &str) -> Result<Option<u64>>;
    async fn handle(&self, data: &ChainData<R>, option: &HandleOption) -> HandleResult;
}

#[async_trait]
impl<R> DataHandler<R> for Box<dyn DataHandler<R>>
where
    R: LoadedData,
{
    async fn loading_contracts(&self, chain_id: u64) -> Result<Option<Vec<ContractConfig>>> {
        self.as_ref().loading_contracts(chain_id).await
    }

    async fn chain_loaded_block(&self, chain_id: u64) -> Result<Option<u64>> {
        self.as_ref().chain_loaded_block(chain_id).await
    }

    async fn contract_loaded_block(&self, chain_id: u64, contract_address: &str) -> Result<Option<u64>> {
        self.as_ref().contract_loaded_block(chain_id, contract_address).await
    }

    async fn handle(&self, data: &ChainData<R>, option: &HandleOption) -> HandleResult {
        self.as_ref().handle(data, option).await
    }
}
