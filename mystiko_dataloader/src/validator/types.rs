use crate::data::ChainData;
use crate::data::ChainResult;
use crate::data::LoadedData;
use crate::validator::ValidatorError;
use async_trait::async_trait;
use mystiko_config::MystikoConfig;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use typed_builder::TypedBuilder;

pub type Result<T> = anyhow::Result<T, ValidatorError>;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ValidateOption {
    pub config: Arc<MystikoConfig>,
    pub validate_concurrency: usize,
    #[builder(default = HashMap::new())]
    pub skip_checkers: HashMap<String, bool>,
}

pub type ValidateResult = Result<ChainResult<()>>;

#[async_trait]
pub trait DataValidator<R: LoadedData>: Send + Sync {
    fn name(&self) -> &'static str;
    async fn validate(&self, data: &ChainData<R>, option: &ValidateOption) -> ValidateResult;
}

#[async_trait]
impl<R> DataValidator<R> for Box<dyn DataValidator<R>>
where
    R: LoadedData,
{
    fn name(&self) -> &'static str {
        self.as_ref().name()
    }

    async fn validate(&self, data: &ChainData<R>, option: &ValidateOption) -> ValidateResult {
        self.as_ref().validate(data, option).await
    }
}
