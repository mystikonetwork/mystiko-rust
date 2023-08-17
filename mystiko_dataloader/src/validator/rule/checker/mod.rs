use crate::data::{ContractData, LoadedData};
use crate::validator::rule::checker::error::CheckerResult;
use crate::validator::rule::types::ValidateContractData;
use crate::validator::ValidateOption;
use async_trait::async_trait;
use typed_builder::TypedBuilder;

pub mod counter;
pub mod error;
pub mod partial;
pub mod sequence;
pub mod tree;

#[derive(Debug, TypedBuilder)]
pub struct RuleCheckData<'a, R: LoadedData> {
    pub chain_id: u64,
    pub contract_data: &'a ContractData<R>,
    pub merged_data: &'a ValidateContractData,
    pub option: &'a ValidateOption,
}

#[async_trait]
pub trait RuleChecker<R: LoadedData>: Send + Sync {
    async fn check(&self, data: &RuleCheckData<R>) -> CheckerResult<()>;
}

#[async_trait]
impl<R> RuleChecker<R> for Box<dyn RuleChecker<R>>
where
    R: LoadedData,
{
    async fn check(&self, data: &RuleCheckData<R>) -> CheckerResult<()> {
        self.as_ref().check(data).await
    }
}
