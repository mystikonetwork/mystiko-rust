use crate::data::types::LoadedData;
use crate::handler::types::{CommitmentQueryOption, DataHandler};
use crate::validator::data::{ValidateCommitment, ValidateContractData, ValidateNullifier};
use crate::validator::error::ValidatorError;
use crate::validator::rule::ValidatorRule;
use crate::validator::types::{Result, ValidateOption};
use ethers_core::types::{Address, BlockId, BlockNumber};
use mystiko_abi::commitment_pool::CommitmentPool;
use mystiko_ethers::provider::factory::Provider;
use mystiko_protos::data::v1::CommitmentStatus;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug)]
pub struct CounterChecker<R, H = Box<dyn DataHandler<R>>> {
    _phantom: std::marker::PhantomData<R>,
    provider: Arc<Provider>,
    handler: Arc<H>,
}

#[derive(Debug, Default)]
pub struct CounterCheckerBuilder<R, H = Box<dyn DataHandler<R>>> {
    _phantom: std::marker::PhantomData<R>,
    provider: Option<Arc<Provider>>,
    handler: Option<Arc<H>>,
}

impl<R, H> CounterCheckerBuilder<R, H>
where
    R: 'static + LoadedData,
    H: 'static + DataHandler<R>,
{
    pub fn new() -> Self {
        CounterCheckerBuilder {
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

    pub fn build(self) -> Result<CounterChecker<R, H>> {
        let provider = self
            .provider
            .ok_or_else(|| ValidatorError::ValidatorBuildError("provider cannot be None".to_string()))?;
        let handler = self
            .handler
            .ok_or_else(|| ValidatorError::ValidatorBuildError("handler cannot be None".to_string()))?;

        Ok(CounterChecker {
            _phantom: Default::default(),
            provider,
            handler,
        })
    }
}

#[async_trait::async_trait]
impl<R, H> ValidatorRule for CounterChecker<R, H>
where
    R: 'static + LoadedData,
    H: 'static + DataHandler<R>,
{
    async fn check(&self, data: &ValidateContractData, _option: &ValidateOption) -> Result<()> {
        let address = Address::from_str(data.contract_address.as_str())
            .map_err(|_| ValidatorError::ValidatorValidateError("invalid contract address".to_string()))?;
        let commitment_contract = CommitmentPool::new(address, self.provider.clone());
        self.check_commitment(
            data.chain_id,
            &commitment_contract,
            data.start_block,
            data.end_block,
            &data.commitments,
        )
        .await?;
        self.check_nullifier(&commitment_contract, data.end_block, &data.nullifiers)
            .await
    }
}

impl<R, H> CounterChecker<R, H>
where
    R: 'static + LoadedData,
    H: 'static + DataHandler<R>,
{
    async fn check_commitment(
        &self,
        chain_id: u64,
        contract: &CommitmentPool<Provider>,
        start_block: u64,
        end_block: u64,
        cms: &[ValidateCommitment],
    ) -> Result<()> {
        self.check_commitment_included_count(chain_id, contract, start_block, end_block, cms)
            .await?;
        self.check_commitment_queued_count(contract, end_block, cms).await
    }

    async fn check_commitment_included_count(
        &self,
        chain_id: u64,
        contract: &CommitmentPool<Provider>,
        start_block: u64,
        end_block: u64,
        cms: &[ValidateCommitment],
    ) -> Result<()> {
        let included = cms
            .iter()
            .filter(|c| c.status == CommitmentStatus::Included)
            .collect::<Vec<_>>();
        let included_count = contract
            .get_commitment_included_count()
            .block(BlockId::Number(BlockNumber::Number(end_block.into())))
            .await?
            .as_u64();
        match included.last() {
            None => {
                let target_block = start_block - 1;
                let option = CommitmentQueryOption::builder()
                    .chain_id(chain_id)
                    .contract_address(contract.address().to_string())
                    .end_block(target_block)
                    .status(CommitmentStatus::Included)
                    .build();
                let query_result = self.handler.count_commitments(&option).await?;
                if query_result.end_block != target_block || query_result.result != included_count {
                    return Err(ValidatorError::ValidatorValidateError(format!(
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
                    Err(ValidatorError::ValidatorValidateError(format!(
                        "commitment included count mismatch, fetched: {}, expected: {}",
                        included_count, fetched_include_count
                    )))
                }
            }
        }
    }

    async fn check_commitment_queued_count(
        &self,
        _contract: &CommitmentPool<Provider>,
        _end_block: u64,
        _cms: &[ValidateCommitment],
    ) -> Result<()> {
        // todo check queued count
        Ok(())
    }

    async fn check_nullifier(
        &self,
        _contract: &CommitmentPool<Provider>,
        _end_block: u64,
        _nullifiers: &[ValidateNullifier],
    ) -> Result<()> {
        // todo check nullifier
        Ok(())
    }
}
