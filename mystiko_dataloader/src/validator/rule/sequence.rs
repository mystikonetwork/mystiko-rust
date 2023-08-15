use crate::data::types::LoadedData;
use crate::handler::types::{CommitmentQueryOption, DataHandler};
use crate::validator::data::ValidateContractData;
use crate::validator::error::ValidatorError;
use crate::validator::rule::ValidatorRule;
use crate::validator::types::{Result, ValidateOption};
use async_trait::async_trait;
use mystiko_protos::data::v1::CommitmentStatus;
use std::sync::Arc;

#[derive(Debug)]
pub struct SequenceChecker<R, H = Box<dyn DataHandler<R>>> {
    _phantom: std::marker::PhantomData<R>,
    handler: Arc<H>,
}

#[derive(Debug, Default)]
pub struct SequenceCheckerBuilder<R, H = Box<dyn DataHandler<R>>> {
    _phantom: std::marker::PhantomData<R>,
    handler: Option<Arc<H>>,
}

#[async_trait]
impl<R, H> ValidatorRule for SequenceChecker<R, H>
where
    R: 'static + LoadedData,
    H: 'static + DataHandler<R>,
{
    async fn check(&self, data: &ValidateContractData, _option: &ValidateOption) -> Result<()> {
        if data.commitments.is_empty() {
            return Ok(());
        }

        self.check_leaf_index_sequence_with_handler(data).await?;
        Ok(())
    }
}

impl<R, H> SequenceCheckerBuilder<R, H>
where
    R: 'static + LoadedData,
    H: 'static + DataHandler<R>,
{
    pub fn new() -> Self {
        SequenceCheckerBuilder {
            _phantom: Default::default(),
            handler: None,
        }
    }

    pub fn shared_handle(mut self, handle: Arc<H>) -> Self {
        self.handler = Some(handle);
        self
    }

    pub fn build(self) -> Result<SequenceChecker<R, H>> {
        let handler = self
            .handler
            .ok_or_else(|| ValidatorError::ValidatorBuildError("handler cannot be None".to_string()))?;

        Ok(SequenceChecker {
            _phantom: Default::default(),
            handler,
        })
    }
}

impl<R, H> SequenceChecker<R, H>
where
    R: 'static + LoadedData,
    H: 'static + DataHandler<R>,
{
    async fn check_leaf_index_sequence_with_handler(&self, data: &ValidateContractData) -> Result<()> {
        let first_cm = &data.commitments[0];
        if first_cm.status == CommitmentStatus::Included && !first_cm.merged {
            return Ok(());
        }

        let end_block = data.start_block - 1;
        let count = self
            .query_commitment_count(data.chain_id, &data.contract_address, end_block, first_cm.status)
            .await?;
        if count != first_cm.leaf_index {
            Err(ValidatorError::ValidatorValidateError(
                "commitment leaf index mismatch".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    async fn query_commitment_count(
        &self,
        chain_id: u64,
        contract_address: &str,
        end_block: u64,
        status: CommitmentStatus,
    ) -> Result<u64> {
        let option = CommitmentQueryOption::builder()
            .chain_id(chain_id)
            .contract_address(contract_address.to_string())
            .status(status)
            .end_block(end_block)
            .build();
        let query_result = self.handler.count_commitments(&option).await?;
        if query_result.end_block != end_block {
            return Err(ValidatorError::ValidatorValidateError(
                "commitment count query end block mismatch".to_string(),
            ));
        }

        Ok(query_result.result)
    }
}
