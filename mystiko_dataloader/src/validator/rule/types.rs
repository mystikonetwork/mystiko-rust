use crate::validator::rule::data::ValidateContractData;
use crate::validator::rule::error::Result;
use crate::validator::types::ValidateOption;
use async_trait::async_trait;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RuleCheckerType {
    Sequence,
    Counter,
    Tree,
}

#[async_trait]
pub trait RuleChecker: Send + Sync {
    async fn check(&self, data: &ValidateContractData, option: &ValidateOption) -> Result<()>;
}

#[async_trait]
impl RuleChecker for Box<dyn RuleChecker> {
    async fn check(&self, data: &ValidateContractData, option: &ValidateOption) -> Result<()> {
        self.as_ref().check(data, option).await
    }
}
