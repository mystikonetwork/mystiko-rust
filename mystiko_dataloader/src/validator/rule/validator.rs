use crate::data::ChainData;
use crate::data::ContractData;
use crate::data::LoadedData;
use crate::data::{ChainResult, ContractResult};
use crate::handler::DataHandler;
use crate::validator::rule::checker::RuleChecker;
use crate::validator::rule::error::Result;
use crate::validator::rule::merger::data::DataMerger;
use crate::validator::rule::RuleCheckData;
use crate::validator::types::{DataValidator, ValidateOption, ValidateResult};
use async_trait::async_trait;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct RuleValidatorInitParam<R, H = Box<dyn DataHandler<R>>, L = Box<dyn RuleChecker<R>>> {
    handler: Arc<H>,
    pub rules: Vec<Arc<L>>,
    #[builder(default = Default::default())]
    _phantom: std::marker::PhantomData<R>,
}

pub struct RuleValidator<R, H = Box<dyn DataHandler<R>>, L = Box<dyn RuleChecker<R>>> {
    merger: DataMerger<R, H>,
    rules: Vec<Arc<L>>,
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R, H, L> DataValidator<R> for RuleValidator<R, H, L>
where
    R: LoadedData,
    H: DataHandler<R>,
    L: RuleChecker<R>,
{
    async fn validate(&self, data: &ChainData<R>, option: &ValidateOption) -> ValidateResult {
        if data.contracts_data.is_empty() {
            return Err(anyhow::Error::msg("data to be validated is empty").into());
        }

        let mut futures = Vec::new();
        for contract_data in &data.contracts_data {
            futures.push(self.contract_data_validate(data.chain_id, contract_data, option));
        }

        let results = futures::future::join_all(futures).await;
        let chain_result = ChainResult::builder()
            .chain_id(data.chain_id)
            .contract_results(results)
            .build();

        Ok(chain_result)
    }
}

impl<R, H, L> RuleValidator<R, H, L>
where
    R: LoadedData,
    H: DataHandler<R>,
    L: RuleChecker<R>,
{
    pub fn new(param: &RuleValidatorInitParam<R, H, L>) -> RuleValidator<R, H, L> {
        let merger = DataMerger::builder().handler(param.handler.clone()).build();
        RuleValidator {
            merger,
            rules: param.rules.clone(),
            _phantom: std::marker::PhantomData,
        }
    }

    async fn contract_data_validate(
        &self,
        chain_id: u64,
        data: &ContractData<R>,
        option: &ValidateOption,
    ) -> ContractResult<()> {
        match self.merge_and_check(chain_id, data, option).await {
            Ok(_) => ContractResult::builder()
                .address(data.address.clone())
                .result(Ok(()))
                .build(),
            Err(e) => ContractResult::builder()
                .address(data.address.clone())
                .result(Err(e.into()))
                .build(),
        }
    }

    async fn merge_and_check(&self, chain_id: u64, data: &ContractData<R>, option: &ValidateOption) -> Result<()> {
        let merged_data = self.merger.merge_contract_data(chain_id, data).await?;
        let rule_check_data = RuleCheckData::builder()
            .chain_id(chain_id)
            .contract_data(data)
            .merged_data(&merged_data)
            .option(option)
            .build();
        for rule in &self.rules {
            rule.check(&rule_check_data).await?;
        }
        Ok(())
    }
}
