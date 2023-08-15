use crate::data::types::LoadedData;
use crate::handler::types::DataHandler;
use crate::validator::data::ValidateContractData;
use crate::validator::error::ValidatorError;
use crate::validator::rule::ValidatorRule;
use crate::validator::types::{Result, ValidateOption};
use mystiko_ethers::provider::factory::Provider;
use std::sync::Arc;

#[derive(Debug)]
pub struct TreeChecker<R, H = Box<dyn DataHandler<R>>> {
    _phantom: std::marker::PhantomData<R>,
    pub provider: Arc<Provider>,
    _handler: Arc<H>,
}

#[derive(Debug, Default)]
pub struct TreeCheckerBuilder<R, H = Box<dyn DataHandler<R>>> {
    _phantom: std::marker::PhantomData<R>,
    provider: Option<Arc<Provider>>,
    handler: Option<Arc<H>>,
}

impl<R, H> TreeCheckerBuilder<R, H>
where
    R: 'static + LoadedData,
    H: 'static + DataHandler<R>,
{
    pub fn new() -> Self {
        TreeCheckerBuilder {
            _phantom: Default::default(),
            provider: None,
            handler: None,
        }
    }

    pub fn shared_provider(mut self, provider: Arc<Provider>) -> Self {
        self.provider = Some(provider);
        self
    }

    pub fn shared_handle(mut self, handle: Arc<H>) -> Self {
        self.handler = Some(handle);
        self
    }

    pub fn build(self) -> Result<TreeChecker<R, H>> {
        let provider = self
            .provider
            .ok_or_else(|| ValidatorError::ValidatorBuildError("provider cannot be None".to_string()))?;

        let handler = self
            .handler
            .ok_or_else(|| ValidatorError::ValidatorBuildError("handler cannot be None".to_string()))?;

        Ok(TreeChecker {
            _phantom: Default::default(),
            provider,
            _handler: handler,
        })
    }
}

#[async_trait::async_trait]
impl<R, H> ValidatorRule for TreeChecker<R, H>
where
    R: 'static + LoadedData,
    H: 'static + DataHandler<R>,
{
    async fn check(&self, _data: &ValidateContractData, _option: &ValidateOption) -> Result<()> {
        // todo
        Err(ValidatorError::ValidatorValidateError(
            "tree checker not implemented".to_string(),
        ))
    }
}
