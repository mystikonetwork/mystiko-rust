use crate::data::chain::ChainData;
use crate::data::result::ChainResult;
use crate::data::types::LoadedData;
use crate::handler::error::HandlerError;
use async_trait::async_trait;
use mystiko_config::wrapper::contract::ContractConfig;
use mystiko_config::wrapper::mystiko::MystikoConfig;
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

#[derive(Debug, Clone, TypedBuilder)]
pub struct ContractCommitment {
    pub contract_address: String,
    pub commitment: Commitment,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct ContractNullifier {
    pub contract_address: String,
    pub nullifier: Nullifier,
}

#[derive(Debug, Clone)]
pub enum OrderType {
    ASC,
    DESC,
}

#[derive(Debug, Clone)]
pub enum CommitmentOrderByType {
    BlockNumber,
    LeafIndex,
}

#[derive(Debug, Clone)]
pub enum NullifierOrderByType {
    BlockNumber,
}

#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommitmentQueryOption {
    #[builder(default, setter(strip_option))]
    pub start_block: Option<u64>,
    #[builder(default, setter(strip_option))]
    pub end_block: Option<u64>,
    #[builder(default, setter(strip_option))]
    pub contract_address: Option<Vec<String>>,
    #[builder(default, setter(strip_option))]
    pub commitment_hash: Option<Vec<BigUint>>,
    #[builder(default, setter(strip_option))]
    pub status: Option<CommitmentStatus>,
    #[builder(default, setter(strip_option))]
    pub order_by: Option<CommitmentOrderByType>,
    #[builder(default, setter(strip_option))]
    pub order: Option<OrderType>,
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct NullifierQueryOption {
    #[builder(default, setter(strip_option))]
    pub start_block: Option<u64>,
    #[builder(default, setter(strip_option))]
    pub end_block: Option<u64>,
    #[builder(default, setter(strip_option))]
    pub contract_address: Option<Vec<String>>,
    #[builder(default, setter(strip_option))]
    pub nullifier: Option<BigUint>,
    #[builder(default, setter(strip_option))]
    pub order_by: Option<NullifierOrderByType>,
    #[builder(default, setter(strip_option))]
    pub order: Option<OrderType>,
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
    async fn query_commitment(
        &self,
        chain_id: u64,
        option: &CommitmentQueryOption,
    ) -> Result<Option<ContractCommitment>> {
        Ok(self.query_commitments(chain_id, option).await?.into_iter().next())
    }
    async fn query_commitments(&self, chain_id: u64, option: &CommitmentQueryOption)
        -> Result<Vec<ContractCommitment>>;
    async fn query_nullifier(&self, chain_id: u64, option: &NullifierQueryOption) -> Result<Option<ContractNullifier>> {
        Ok(self.query_nullifiers(chain_id, option).await?.into_iter().next())
    }
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
