use async_trait::async_trait;
use mystiko_dataloader::data::LoadedData;
use mystiko_dataloader::validator::rule::{CheckerResult, RuleChecker, ValidateMergedData, ValidateOriginalData};
use std::fmt::Debug;
use tokio::sync::RwLock;
use typed_builder::TypedBuilder;

pub enum RuleCheckerType {
    Integrity,
    Sequence,
    Counter,
    Tree,
}

#[derive(Debug, TypedBuilder)]
pub struct MockRuleChecker<R> {
    pub merged_data: RwLock<Option<ValidateMergedData>>,
    #[builder(default = Default::default())]
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R> RuleChecker<R> for MockRuleChecker<R>
where
    R: LoadedData,
{
    fn name(&self) -> &'static str {
        "mock_checker"
    }

    async fn check<'a>(
        &self,
        _data: &ValidateOriginalData<'a, R>,
        merged_data: &ValidateMergedData,
    ) -> CheckerResult<()> {
        *self.merged_data.write().await = Some(merged_data.clone());
        Ok(())
    }
}

impl<R> MockRuleChecker<R>
where
    R: LoadedData,
{
    pub async fn cmp_data(&self, data: Option<&ValidateMergedData>) -> bool {
        let mut merged_data = self.merged_data.write().await;
        let result = match (&*merged_data, data) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(a), Some(b)) => a == b,
        };

        *merged_data = None;
        result
    }
}
