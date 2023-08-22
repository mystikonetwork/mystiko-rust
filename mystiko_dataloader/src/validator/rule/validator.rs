use crate::data::ChainData;
use crate::data::LoadedData;
use crate::data::{ChainResult, ContractResult};
use crate::handler::DataHandler;
use crate::validator::rule::checker::RuleChecker;
use crate::validator::rule::error::Result;
use crate::validator::rule::merger::DataMerger;
use crate::validator::rule::{RuleValidatorError, ValidateOriginalData};
use crate::validator::types::{DataValidator, ValidateOption, ValidateResult};
use async_trait::async_trait;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct RuleValidatorOptions<R, H = Box<dyn DataHandler<R>>, C = Box<dyn RuleChecker<R>>> {
    handler: Arc<H>,
    pub checkers: Vec<Arc<C>>,
    #[builder(default = Default::default())]
    _phantom: std::marker::PhantomData<R>,
}

#[derive(Debug)]
pub struct RuleValidator<R, H = Box<dyn DataHandler<R>>, C = Box<dyn RuleChecker<R>>> {
    merger: DataMerger<R, H>,
    checkers: Vec<Arc<C>>,
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R, H, C> DataValidator<R> for RuleValidator<R, H, C>
where
    R: LoadedData,
    H: DataHandler<R>,
    C: RuleChecker<R>,
{
    async fn validate(&self, data: &ChainData<R>, option: &ValidateOption) -> ValidateResult {
        if data.contracts_data.is_empty() {
            return Err(
                <RuleValidatorError as Into<anyhow::Error>>::into(RuleValidatorError::EmptyValidateDataError).into(),
            );
        }

        let chain = option.config.find_chain(data.chain_id).ok_or_else(|| {
            <RuleValidatorError as Into<anyhow::Error>>::into(RuleValidatorError::ChainNotFoundError(data.chain_id))
        })?;

        let mut futures = Vec::new();
        let mut contract_results = vec![];
        for contract_data in &data.contracts_data {
            match chain.find_pool_contract_by_address(&contract_data.address) {
                None => {
                    contract_results.push(
                        ContractResult::builder()
                            .address(contract_data.address.clone())
                            .result(Ok(()))
                            .build(),
                    );
                }
                Some(contract_cfg) => {
                    futures.push(
                        self.contract_data_validate(
                            ValidateOriginalData::builder()
                                .chain_id(data.chain_id)
                                .contract_config(contract_cfg)
                                .contract_data(contract_data)
                                .option(option)
                                .build(),
                        ),
                    );
                }
            }
        }

        let results = futures::future::join_all(futures).await;
        contract_results.extend(results);
        let chain_result = ChainResult::builder()
            .chain_id(data.chain_id)
            .contract_results(contract_results)
            .build();

        Ok(chain_result)
    }
}

impl<R, H, C> RuleValidator<R, H, C>
where
    R: LoadedData,
    H: DataHandler<R>,
    C: RuleChecker<R>,
{
    pub fn new(options: &RuleValidatorOptions<R, H, C>) -> RuleValidator<R, H, C> {
        let merger = DataMerger::builder().handler(options.handler.clone()).build();
        RuleValidator {
            merger,
            checkers: options.checkers.clone(),
            _phantom: std::marker::PhantomData,
        }
    }

    async fn contract_data_validate(&self, data: ValidateOriginalData<'_, R>) -> ContractResult<()> {
        match self.merge_and_check(&data).await {
            Ok(_) => ContractResult::builder()
                .address(data.contract_data.address.clone())
                .result(Ok(()))
                .build(),
            Err(e) => ContractResult::builder()
                .address(data.contract_data.address.clone())
                .result(Err(e.into()))
                .build(),
        }
    }

    async fn merge_and_check(&self, data: &ValidateOriginalData<'_, R>) -> Result<()> {
        let merged_data = self.merger.merge_contract_data(data).await?;
        for checker in &self.checkers {
            checker.check(data, &merged_data).await?;
        }
        Ok(())
    }
}
