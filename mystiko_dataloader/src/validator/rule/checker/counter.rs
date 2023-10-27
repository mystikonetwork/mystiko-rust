use crate::data::LoadedData;
use crate::handler::{CommitmentQueryOption, DataHandler, NullifierQueryOption};
use crate::validator::rule::checker::error::{CounterCheckerError, RuleCheckError};
use crate::validator::rule::checker::CheckerResult;
use crate::validator::rule::checker::RuleChecker;
use crate::validator::rule::types::ValidateMergedData;
use crate::validator::rule::ValidateOriginalData;
use async_trait::async_trait;
use ethers_core::types::{BlockId, BlockNumber};
use mystiko_abi::commitment_pool::CommitmentPool;
use mystiko_ethers::{Provider, Providers};
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_utils::address::ethers_address_from_string;
use std::sync::Arc;
use typed_builder::TypedBuilder;

const CONTRACT_VERSION_SUPPORT_FULL_COUNTER_QUERY: u32 = 6;
const RULE_COUNTER_CHECKER_NAME: &str = "counter";

#[derive(Debug, TypedBuilder)]
pub struct CounterChecker<R, H = Box<dyn DataHandler<R>>, P = Box<dyn Providers>> {
    providers: Arc<P>,
    handler: Arc<H>,
    #[builder(default = Default::default())]
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R, H, P> RuleChecker<R> for CounterChecker<R, H, P>
where
    R: LoadedData,
    H: DataHandler<R>,
    P: Providers,
{
    fn name(&self) -> &'static str {
        RULE_COUNTER_CHECKER_NAME
    }

    async fn check<'a>(
        &self,
        data: &ValidateOriginalData<'a, R>,
        merged_data: &ValidateMergedData,
    ) -> CheckerResult<()> {
        let address = ethers_address_from_string(&merged_data.contract_address)
            .map_err(|_| RuleCheckError::ContractAddressError(merged_data.contract_address.to_string()))?;
        let provider = self.providers.get_provider(merged_data.chain_id).await?;
        let commitment_contract = CommitmentPool::new(address, provider);
        self.check_commitment(data, merged_data, &commitment_contract).await?;
        self.check_nullifier(data, merged_data, &commitment_contract).await
    }
}

impl<R, H, P> CounterChecker<R, H, P>
where
    R: LoadedData,
    H: DataHandler<R>,
    P: Providers,
{
    async fn check_commitment<'a>(
        &self,
        data: &ValidateOriginalData<'a, R>,
        merged_data: &ValidateMergedData,
        contract: &CommitmentPool<Provider>,
    ) -> CheckerResult<()> {
        if data.should_skip_commitment_check() {
            if merged_data.commitments.is_empty() {
                Ok(())
            } else {
                Err(CounterCheckerError::CommitmentAfterContractDisabledError.into())
            }
        } else {
            self.check_commitment_included_count(merged_data, contract).await?;
            self.check_commitment_count(data, merged_data, contract).await
        }
    }

    async fn check_commitment_included_count(
        &self,
        merged_data: &ValidateMergedData,
        contract: &CommitmentPool<Provider>,
    ) -> CheckerResult<()> {
        let included_cms = merged_data
            .commitments
            .iter()
            .filter(|c| c.status == CommitmentStatus::Included)
            .collect::<Vec<_>>();
        let included_count = contract
            .get_commitment_included_count()
            .block(BlockId::Number(BlockNumber::Number(merged_data.end_block.into())))
            .await?
            .as_u64();
        match included_cms.last() {
            None => {
                let target_block = merged_data.start_block - 1;
                let option = CommitmentQueryOption::builder()
                    .chain_id(merged_data.chain_id)
                    .contract_address(merged_data.contract_address.clone())
                    .end_block(target_block)
                    .status(CommitmentStatus::Included)
                    .build();
                let query_result = self.handler.count_commitments(&option).await?;
                if query_result.end_block != target_block {
                    return Err(CounterCheckerError::TargetBlockError(target_block, query_result.end_block).into());
                }

                if query_result.result != included_count {
                    return Err(CounterCheckerError::CommitmentIncludedCountMismatchError(
                        query_result.result,
                        included_count,
                    )
                    .into());
                }

                Ok(())
            }
            Some(cm) => {
                let fetched_include_count = cm.leaf_index + 1;
                if included_count != fetched_include_count {
                    Err(CounterCheckerError::CommitmentIncludedCountMismatchError(
                        fetched_include_count,
                        included_count,
                    )
                    .into())
                } else {
                    Ok(())
                }
            }
        }
    }

    async fn check_commitment_count<'a>(
        &self,
        data: &ValidateOriginalData<'a, R>,
        merged_data: &ValidateMergedData,
        contract: &CommitmentPool<Provider>,
    ) -> CheckerResult<()> {
        if data.contract_config.version() >= CONTRACT_VERSION_SUPPORT_FULL_COUNTER_QUERY {
            let total = contract
                .get_commitment_count()
                .block(BlockId::Number(BlockNumber::Number(merged_data.end_block.into())))
                .await?
                .as_u64();

            match merged_data.commitments.last() {
                None => self.check_commitment_count_with_handler(total, merged_data).await?,
                Some(cm) => {
                    if cm.status == CommitmentStatus::Included && !cm.inner_merge {
                        self.check_commitment_count_with_handler(total, merged_data).await?;
                    } else {
                        let fetched_total_count = cm.leaf_index + 1;
                        if total != fetched_total_count {
                            return Err(
                                CounterCheckerError::CommitmentCountMismatchError(fetched_total_count, total).into(),
                            );
                        }
                    }
                }
            };
        }

        Ok(())
    }

    async fn check_commitment_count_with_handler<'a>(
        &self,
        total: u64,
        merged_data: &ValidateMergedData,
    ) -> CheckerResult<()> {
        let target_block = merged_data.start_block - 1;
        let option = CommitmentQueryOption::builder()
            .chain_id(merged_data.chain_id)
            .contract_address(merged_data.contract_address.clone())
            .end_block(target_block)
            .build();
        let query_result = self.handler.count_commitments(&option).await?;
        if query_result.end_block != target_block {
            return Err(CounterCheckerError::TargetBlockError(target_block, query_result.end_block).into());
        }
        if total != query_result.result {
            return Err(CounterCheckerError::CommitmentCountMismatchError(query_result.result, total).into());
        }
        Ok(())
    }

    async fn check_nullifier<'a>(
        &self,
        data: &ValidateOriginalData<'a, R>,
        merged_data: &ValidateMergedData,
        contract: &CommitmentPool<Provider>,
    ) -> CheckerResult<()> {
        if data.contract_config.version() >= CONTRACT_VERSION_SUPPORT_FULL_COUNTER_QUERY {
            if let Some(nullifiers) = &merged_data.nullifiers {
                let total = contract
                    .get_nullifier_count()
                    .block(BlockId::Number(BlockNumber::Number(merged_data.end_block.into())))
                    .await?
                    .as_u64();
                let target_block = merged_data.start_block - 1;
                let option = NullifierQueryOption::builder()
                    .chain_id(merged_data.chain_id)
                    .contract_address(merged_data.contract_address.clone())
                    .end_block(target_block)
                    .build();
                let query_result = self.handler.count_nullifiers(&option).await?;
                if query_result.end_block != target_block {
                    return Err(CounterCheckerError::TargetBlockError(target_block, query_result.end_block).into());
                }
                let fetcher_total_count = query_result.result + nullifiers.len() as u64;
                if total != fetcher_total_count {
                    return Err(CounterCheckerError::NullifierCountMismatchError(fetcher_total_count, total).into());
                }
            }
        }

        Ok(())
    }
}
