pub mod counter;
pub mod sequence;
pub mod tree;

use crate::validator::data::ValidateContractData;
use crate::validator::types::{Result, ValidateOption};
use async_trait::async_trait;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ValidatorRuleType {
    Sequence,
    Counter,
    Tree,
}

#[async_trait]
pub trait ValidatorRule: Send + Sync {
    async fn check(&self, data: &ValidateContractData, option: &ValidateOption) -> Result<()>;
}

#[async_trait]
impl ValidatorRule for Box<dyn ValidatorRule> {
    async fn check(&self, data: &ValidateContractData, option: &ValidateOption) -> Result<()> {
        self.as_ref().check(data, option).await
    }
}
