use crate::data::LoadedData;
use crate::get_provider;
use crate::handler::{CommitmentQueryOption, DataHandler};
use crate::validator::rule::checker::error::{CounterCheckerError, RuleCheckError};
use crate::validator::rule::checker::CheckerResult;
use crate::validator::rule::checker::RuleChecker;
use crate::validator::rule::types::ValidateContractData;
use crate::validator::rule::RuleCheckData;
use async_trait::async_trait;
use ethers_core::types::{Address, BlockId, BlockNumber};
use mystiko_abi::commitment_pool::CommitmentPool;
use mystiko_ethers::provider::factory::Provider;
use mystiko_ethers::provider::pool::Providers;
use mystiko_protos::data::v1::CommitmentStatus;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct CounterChecker<R, H = Box<dyn DataHandler<R>>, P = Box<dyn Providers>> {
    providers: Arc<RwLock<P>>,
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
    async fn check(&self, data: &RuleCheckData<R>) -> CheckerResult<()> {
        let address = Address::from_str(data.merged_data.contract_address.as_str())
            .map_err(|_| RuleCheckError::ContractAddressError(data.merged_data.contract_address.to_string()))?;
        let provider = get_provider(&self.providers, data.chain_id).await?;
        let commitment_contract = CommitmentPool::new(address, provider);
        self.check_commitment(data.merged_data, &commitment_contract).await?;
        self.check_nullifier(data.merged_data, &commitment_contract).await
    }
}

impl<R, H, P> CounterChecker<R, H, P>
where
    R: LoadedData,
    H: DataHandler<R>,
    P: Providers,
{
    async fn check_commitment(
        &self,
        data: &ValidateContractData,
        contract: &CommitmentPool<Provider>,
    ) -> CheckerResult<()> {
        self.check_commitment_included_count(data, contract).await?;
        self.check_commitment_queued_count(data, contract).await
    }

    async fn check_commitment_included_count(
        &self,
        data: &ValidateContractData,
        contract: &CommitmentPool<Provider>,
    ) -> CheckerResult<()> {
        let included_cms = data
            .commitments
            .iter()
            .filter(|c| c.status == CommitmentStatus::Included)
            .collect::<Vec<_>>();
        let included_count = contract
            .get_commitment_included_count()
            .block(BlockId::Number(BlockNumber::Number(data.end_block.into())))
            .await?
            .as_u64();
        match included_cms.last() {
            None => {
                let target_block = data.start_block - 1;
                let option = CommitmentQueryOption::builder()
                    .chain_id(data.chain_id)
                    .contract_address(contract.address().to_string())
                    .end_block(target_block)
                    .status(CommitmentStatus::Included)
                    .build();
                let query_result = self.handler.count_commitments(&option).await?;
                if query_result.end_block != target_block {
                    return Err(CounterCheckerError::TargetBlockError(target_block, query_result.end_block).into());
                }

                if query_result.result != included_count {
                    return Err(
                        CounterCheckerError::IncludedCountMismatchError(query_result.result, included_count).into(),
                    );
                }

                Ok(())
            }
            Some(cm) => {
                let fetched_include_count = cm.leaf_index + 1;
                if included_count != fetched_include_count {
                    Err(CounterCheckerError::IncludedCountMismatchError(fetched_include_count, included_count).into())
                } else {
                    Ok(())
                }
            }
        }
    }

    async fn check_commitment_queued_count(
        &self,
        _data: &ValidateContractData,
        _contract: &CommitmentPool<Provider>,
    ) -> CheckerResult<()> {
        // todo check queued count
        Ok(())
    }

    async fn check_nullifier(
        &self,
        _data: &ValidateContractData,
        _contract: &CommitmentPool<Provider>,
    ) -> CheckerResult<()> {
        // todo check nullifier
        Ok(())
    }
}
