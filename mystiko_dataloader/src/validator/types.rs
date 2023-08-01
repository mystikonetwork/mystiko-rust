use crate::data::chain::ChainData;
use crate::data::result::ChainResult;
use crate::data::types::LoadedData;
use anyhow::Result;
use async_trait::async_trait;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_protos::data::v1::Commitment;
use std::fmt::Debug;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ValidateOption {
    pub config: Arc<MystikoConfig>,
}

pub type ValidateResult = Result<ChainResult<()>>;

#[async_trait]
pub trait DataValidator<R: LoadedData>: Send + Sync {
    async fn validate(&self, data: &ChainData<R>, option: &ValidateOption) -> ValidateResult;
}

#[async_trait]
impl<R> DataValidator<R> for Box<dyn DataValidator<R>>
where
    R: LoadedData,
{
    async fn validate(&self, data: &ChainData<R>, option: &ValidateOption) -> ValidateResult {
        self.as_ref().validate(data, option).await
    }
}

#[async_trait]
pub trait DataRetrieval: Send + Sync {
    async fn latest_leaf_index(&self, chain_id: u64, contract_address: &str) -> Result<u64>;
    async fn included_count(&self, chain_id: u64, contract_address: &str) -> Result<u64>;
    async fn commitment(
        &self,
        chain_id: u64,
        contract_address: &str,
        commitment_hash: &[u8],
    ) -> Result<Option<Commitment>>;
}
