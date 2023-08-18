use crate::data::{ContractData, LoadedData};
use crate::validator::rule::types::ValidateContractData;
use crate::validator::ValidateOption;
use async_trait::async_trait;
use typed_builder::TypedBuilder;

mod counter;
mod error;
mod integrity;
mod sequence;
mod tree;

pub use counter::*;
pub use error::*;
pub use integrity::*;
pub use sequence::*;
pub use tree::*;

pub type CheckerResult<T> = anyhow::Result<T, RuleCheckError>;

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
