use crate::data::LoadedData;
use crate::validator::rule::types::ValidateMergedData;
use async_trait::async_trait;

mod counter;
mod error;
mod integrity;
mod sequence;
mod tree;

use crate::validator::rule::ValidateOriginalData;
pub use counter::*;
pub use error::*;
pub use integrity::*;
pub use sequence::*;
pub use tree::*;

pub type CheckerResult<T> = anyhow::Result<T, RuleCheckError>;

#[async_trait]
pub trait RuleChecker<R: LoadedData>: Send + Sync {
    async fn check<'a>(
        &self,
        data: &ValidateOriginalData<'a, R>,
        merged_data: &ValidateMergedData,
    ) -> CheckerResult<()>;
}

#[async_trait]
impl<R> RuleChecker<R> for Box<dyn RuleChecker<R>>
where
    R: LoadedData,
{
    async fn check<'a>(
        &self,
        data: &ValidateOriginalData<'a, R>,
        merged_data: &ValidateMergedData,
    ) -> CheckerResult<()> {
        self.as_ref().check(data, merged_data).await
    }
}
