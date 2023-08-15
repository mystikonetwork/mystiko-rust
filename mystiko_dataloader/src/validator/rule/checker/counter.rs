use crate::data::LoadedData;
use crate::handler::{CommitmentQueryOption, DataHandler};
use crate::validator::rule::checker::RuleChecker;
use crate::validator::rule::data::ValidateContractData;
use crate::validator::rule::error::{Result, RuleValidatorError};
use crate::validator::types::ValidateOption;
use async_trait::async_trait;
use ethers_core::types::{Address, BlockId, BlockNumber};
use mystiko_abi::commitment_pool::CommitmentPool;
use mystiko_ethers::provider::factory::Provider;
use mystiko_ethers::provider::pool::Providers;
use mystiko_protos::data::v1::CommitmentStatus;
use std::str::FromStr;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct CounterChecker<R, H = Box<dyn DataHandler<R>>, P = Box<dyn Providers>> {
    providers: Arc<P>,
    handler: Arc<H>,
    #[builder(default = Default::default())]
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R, H, P> RuleChecker for CounterChecker<R, H, P>
where
    R: LoadedData,
    H: DataHandler<R>,
    P: Providers,
{
    async fn check(&self, data: &ValidateContractData, _option: &ValidateOption) -> Result<()> {
        let address = Address::from_str(data.contract_address.as_str())
            .map_err(|_| RuleValidatorError::ValidateError("invalid contract address".to_string()))?;
        let provider = self.providers.get_provider(data.chain_id).ok_or_else(|| {
            RuleValidatorError::ValidateError(format!("no provider found for chain id {}", data.chain_id))
        })?;
        let commitment_contract = CommitmentPool::new(address, provider);
        self.check_commitment(data, &commitment_contract).await?;
        self.check_nullifier(data, &commitment_contract).await
    }
}

impl<R, H, P> CounterChecker<R, H, P>
where
    R: LoadedData,
    H: DataHandler<R>,
    P: Providers,
{
    async fn check_commitment(&self, data: &ValidateContractData, contract: &CommitmentPool<Provider>) -> Result<()> {
        self.check_commitment_included_count(data, contract).await?;
        self.check_commitment_queued_count(data, contract).await
    }

    async fn check_commitment_included_count(
        &self,
        data: &ValidateContractData,
        contract: &CommitmentPool<Provider>,
    ) -> Result<()> {
        let included = data
            .commitments
            .iter()
            .filter(|c| c.status == CommitmentStatus::Included)
            .collect::<Vec<_>>();
        let included_count = contract
            .get_commitment_included_count()
            .block(BlockId::Number(BlockNumber::Number(data.end_block.into())))
            .await?
            .as_u64();
        match included.last() {
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
                    return Err(RuleValidatorError::ValidateError(format!(
                        "end block mismatch, expect: {}, query: {}",
                        target_block, query_result.end_block
                    )));
                }

                if query_result.result != included_count {
                    return Err(RuleValidatorError::ValidateError(format!(
                        "commitment included count mismatch, handler: {}, provider: {}",
                        query_result.result, included_count
                    )));
                }
                Ok(())
            }
            Some(cm) => {
                let fetched_include_count = cm.leaf_index + 1;
                if included_count == fetched_include_count {
                    Ok(())
                } else {
                    Err(RuleValidatorError::ValidateError(format!(
                        "commitment included count mismatch, fetched: {}, expected: {}",
                        included_count, fetched_include_count
                    )))
                }
            }
        }
    }

    async fn check_commitment_queued_count(
        &self,
        _data: &ValidateContractData,
        _contract: &CommitmentPool<Provider>,
    ) -> Result<()> {
        // todo check queued count
        Ok(())
    }

    async fn check_nullifier(&self, _data: &ValidateContractData, _contract: &CommitmentPool<Provider>) -> Result<()> {
        // todo check nullifier
        Ok(())
    }
}
