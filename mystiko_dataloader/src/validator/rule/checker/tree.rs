use crate::data::LoadedData;
use crate::handler::DataHandler;
use crate::validator::rule::checker::RuleChecker;
use crate::validator::rule::data::ValidateContractData;
use crate::validator::rule::error::{Result, RuleValidatorError};
use crate::validator::types::ValidateOption;
use async_trait::async_trait;
use mystiko_ethers::provider::factory::Provider;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct TreeChecker<R, H = Box<dyn DataHandler<R>>> {
    provider: Arc<Provider>,
    handler: Arc<H>,
    #[builder(default = Default::default())]
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R, H> RuleChecker for TreeChecker<R, H>
where
    R: LoadedData,
    H: DataHandler<R>,
{
    async fn check(&self, _data: &ValidateContractData, _option: &ValidateOption) -> Result<()> {
        // todo
        let _ = self.handler;
        let _ = self.provider;
        Err(RuleValidatorError::ValidateError(
            "tree checker not implemented".to_string(),
        ))
    }
}
