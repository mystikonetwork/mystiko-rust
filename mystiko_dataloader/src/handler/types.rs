use crate::data::chain::ChainData;
use crate::data::result::ChainResult;
use crate::data::types::LoadedData;
use anyhow::Result;
use async_trait::async_trait;
use mystiko_config::wrapper::contract::ContractConfig;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use num_bigint::BigUint;
use std::fmt::Debug;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct HandleOption {
    pub config: Arc<MystikoConfig>,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct ContractCommitment {
    pub contract_address: String,
    pub commitment: Commitment,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct ContractNullifier {
    pub contract_address: String,
    pub commitment: Nullifier,
}

#[derive(Debug, Clone)]
pub enum CommitmentOrderType {
    BlockNumber,
    LeafIndex,
}

#[derive(Debug, Clone)]
pub enum NullifierOrderType {
    BlockNumber,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommitmentQueryOption {
    pub start_block: Option<u64>,
    pub end_block: Option<u64>,
    pub contract_address: Option<Vec<String>>,
    pub commitment_hash: Option<BigUint>,
    pub status: Option<CommitmentStatus>,
    pub order_by: Option<CommitmentOrderType>,
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct NullifierQueryOption {
    pub start_block: Option<u64>,
    pub end_block: Option<u64>,
    pub contract_address: Option<Vec<String>>,
    pub order_by: Option<NullifierOrderType>,
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

pub type HandleResult = Result<ChainResult<()>>;

#[async_trait]
pub trait DataHandler<R: LoadedData>: Send + Sync {
    async fn query_loading_contracts(&self, _chain_id: u64) -> Result<Option<Vec<ContractConfig>>> {
        Ok(None)
    }
    async fn query_chain_loaded_block(&self, chain_id: u64) -> Result<Option<u64>>;
    async fn query_contract_loaded_block(&self, chain_id: u64, contract_address: &str) -> Result<Option<u64>>;
    async fn query_commitment(&self, chain_id: u64, option: &CommitmentQueryOption) -> Option<ContractCommitment> {
        self.query_commitments(chain_id, option)
            .await
            .ok()
            .and_then(|v| v.first().cloned())
    }
    async fn query_commitments(&self, chain_id: u64, option: &CommitmentQueryOption)
        -> Result<Vec<ContractCommitment>>;
    async fn query_nullifiers(&self, chain_id: u64, option: &NullifierQueryOption) -> Result<Vec<ContractNullifier>>;
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

    async fn query_commitments(
        &self,
        chain_id: u64,
        option: &CommitmentQueryOption,
    ) -> Result<Vec<ContractCommitment>> {
        self.as_ref().query_commitments(chain_id, option).await
    }

    async fn query_nullifiers(&self, chain_id: u64, option: &NullifierQueryOption) -> Result<Vec<ContractNullifier>> {
        self.as_ref().query_nullifiers(chain_id, option).await
    }

    async fn handle(&self, data: &ChainData<R>, option: &HandleOption) -> HandleResult {
        self.as_ref().handle(data, option).await
    }
}
