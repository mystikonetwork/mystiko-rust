use crate::data::ChainData;
use crate::data::ChainResult;
use crate::data::LoadedData;
use crate::handler::HandlerError;
use async_trait::async_trait;
use mystiko_config::{ContractConfig, MystikoConfig};
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use num_bigint::BigUint;
use std::fmt::Debug;
use std::sync::Arc;
use typed_builder::TypedBuilder;

pub type Result<T> = anyhow::Result<T, HandlerError>;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct HandleOption {
    pub config: Arc<MystikoConfig>,
}

#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommitmentQueryOption {
    pub chain_id: u64,
    pub contract_address: String,
    pub end_block: u64,
    #[builder(default)]
    pub start_block: Option<u64>,
    #[builder(default)]
    pub commitment_hash: Option<Vec<BigUint>>,
    #[builder(default)]
    pub status: Option<CommitmentStatus>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct NullifierQueryOption {
    pub chain_id: u64,
    pub contract_address: String,
    pub end_block: u64,
    #[builder(default)]
    pub start_block: Option<u64>,
    #[builder(default)]
    pub nullifier: Option<Vec<BigUint>>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct QueryResult<T> {
    pub end_block: u64,
    pub result: T,
}

pub type HandleResult = Result<ChainResult<()>>;

#[async_trait]
pub trait DataHandler<R: LoadedData>: Send + Sync {
    async fn query_loading_contracts(&self, _chain_id: u64) -> Result<Option<Vec<ContractConfig>>> {
        Ok(None)
    }
    async fn query_chain_loaded_block(&self, chain_id: u64) -> Result<Option<u64>>;
    async fn query_contract_loaded_block(&self, chain_id: u64, contract_address: &str) -> Result<Option<u64>>;
    async fn query_commitment(&self, option: &CommitmentQueryOption) -> Result<Option<Commitment>> {
        Ok(self.query_commitments(option).await?.result.into_iter().next())
    }
    async fn query_commitments(&self, option: &CommitmentQueryOption) -> Result<QueryResult<Vec<Commitment>>>;
    async fn count_commitments(&self, option: &CommitmentQueryOption) -> Result<QueryResult<u64>>;
    async fn query_nullifier(&self, option: &NullifierQueryOption) -> Result<Option<Nullifier>> {
        Ok(self.query_nullifiers(option).await?.result.into_iter().next())
    }
    async fn query_nullifiers(&self, option: &NullifierQueryOption) -> Result<QueryResult<Vec<Nullifier>>>;
    async fn count_nullifiers(&self, option: &NullifierQueryOption) -> Result<QueryResult<u64>>;
    async fn handle(&self, data: &ChainData<R>, option: &HandleOption) -> HandleResult;
}

#[async_trait]
impl<R> DataHandler<R> for Box<dyn DataHandler<R>>
where
    R: LoadedData,
{
    async fn query_loading_contracts(&self, chain_id: u64) -> Result<Option<Vec<ContractConfig>>> {
        self.as_ref().query_loading_contracts(chain_id).await
    }

    async fn query_chain_loaded_block(&self, chain_id: u64) -> Result<Option<u64>> {
        self.as_ref().query_chain_loaded_block(chain_id).await
    }

    async fn query_contract_loaded_block(&self, chain_id: u64, contract_address: &str) -> Result<Option<u64>> {
        self.as_ref()
            .query_contract_loaded_block(chain_id, contract_address)
            .await
    }

    async fn query_commitment(&self, option: &CommitmentQueryOption) -> Result<Option<Commitment>> {
        self.as_ref().query_commitment(option).await
    }

    async fn query_commitments(&self, option: &CommitmentQueryOption) -> Result<QueryResult<Vec<Commitment>>> {
        self.as_ref().query_commitments(option).await
    }

    async fn count_commitments(&self, option: &CommitmentQueryOption) -> Result<QueryResult<u64>> {
        self.as_ref().count_commitments(option).await
    }

    async fn query_nullifier(&self, option: &NullifierQueryOption) -> Result<Option<Nullifier>> {
        self.as_ref().query_nullifier(option).await
    }

    async fn query_nullifiers(&self, option: &NullifierQueryOption) -> Result<QueryResult<Vec<Nullifier>>> {
        self.as_ref().query_nullifiers(option).await
    }

    async fn count_nullifiers(&self, option: &NullifierQueryOption) -> Result<QueryResult<u64>> {
        self.as_ref().count_nullifiers(option).await
    }

    async fn handle(&self, data: &ChainData<R>, option: &HandleOption) -> HandleResult {
        self.as_ref().handle(data, option).await
    }
}
