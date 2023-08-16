use crate::data::LoadedData;
use crate::handler::{CommitmentQueryOption, DataHandler};
use crate::validator::rule::checker::RuleChecker;
use crate::validator::rule::data::ValidateContractData;
use crate::validator::rule::error::{Result, RuleValidatorError};
use crate::validator::types::ValidateOption;
use async_trait::async_trait;
use mystiko_protos::data::v1::CommitmentStatus;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct SequenceChecker<R, H = Box<dyn DataHandler<R>>> {
    handler: Arc<H>,
    #[builder(default = Default::default())]
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R, H> RuleChecker for SequenceChecker<R, H>
where
    R: LoadedData,
    H: DataHandler<R>,
{
    async fn check(&self, data: &ValidateContractData, _option: &ValidateOption) -> Result<()> {
        if !data.commitments.is_empty() {
            self.check_leaf_index_sequence_with_handler(data).await
        } else {
            return Ok(());
        }
    }
}

impl<R, H> SequenceChecker<R, H>
where
    R: LoadedData,
    H: DataHandler<R>,
{
    async fn check_leaf_index_sequence_with_handler(&self, data: &ValidateContractData) -> Result<()> {
        let first_cm = &data.commitments[0];
        if first_cm.status == CommitmentStatus::Included && !first_cm.merged {
            return Ok(());
        }

        let count = self.query_handler_commitment_count(data, first_cm.status).await?;
        if count != first_cm.leaf_index {
            Err(RuleValidatorError::ValidateError(
                "commitment leaf index mismatch".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    async fn query_handler_commitment_count(
        &self,
        data: &ValidateContractData,
        status: CommitmentStatus,
    ) -> Result<u64> {
        let target_block = data.start_block - 1;
        let option = CommitmentQueryOption::builder()
            .chain_id(data.chain_id)
            .contract_address(data.contract_address.clone())
            .status(status)
            .end_block(target_block)
            .build();
        let query_result = self.handler.count_commitments(&option).await?;
        if query_result.end_block != target_block {
            return Err(RuleValidatorError::ValidateError("end block mismatch".to_string()));
        }

        Ok(query_result.result)
    }
}
