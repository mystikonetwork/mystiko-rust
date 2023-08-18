use crate::data::LoadedData;
use crate::handler::{CommitmentQueryOption, DataHandler};
use crate::validator::rule::checker::RuleChecker;
use crate::validator::rule::types::ValidateContractData;
use crate::validator::rule::SequenceCheckerError;
use crate::validator::rule::{CheckerResult, RuleCheckData, ValidateCommitment};
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
impl<R, H> RuleChecker<R> for SequenceChecker<R, H>
where
    R: LoadedData,
    H: DataHandler<R>,
{
    async fn check(&self, data: &RuleCheckData<R>) -> CheckerResult<()> {
        match data.merged_data.commitments.first() {
            Some(f) => {
                self.check_commitment_leaf_index_sequence(data.merged_data).await?;
                self.check_commitment_status_sequence(f, data.merged_data)?;
                self.check_commitment_sequence_with_handler(f, data.merged_data).await
            }
            None => Ok(()),
        }
    }
}

impl<R, H> SequenceChecker<R, H>
where
    R: LoadedData,
    H: DataHandler<R>,
{
    async fn check_commitment_leaf_index_sequence(&self, data: &ValidateContractData) -> CheckerResult<()> {
        if data
            .commitments
            .windows(2)
            .any(|window| window[0].leaf_index + 1 != window[1].leaf_index)
        {
            return Err(SequenceCheckerError::LeafIndexNotSequencedError.into());
        }
        Ok(())
    }

    fn check_commitment_status_sequence(
        &self,
        first: &ValidateCommitment,
        data: &ValidateContractData,
    ) -> CheckerResult<()> {
        if first.status == CommitmentStatus::Queued {
            for cm in &data.commitments {
                if cm.status != first.status {
                    return Err(SequenceCheckerError::CommitmentStatusNotSequencedError.into());
                }
            }
        } else if first.status == CommitmentStatus::Included {
            let mut queued_seen = false;
            let mut inner_merged_seen = false;

            for cm in &data.commitments {
                if cm.status == CommitmentStatus::Queued {
                    queued_seen = true;
                } else if cm.status == CommitmentStatus::Included && queued_seen {
                    return Err(SequenceCheckerError::CommitmentStatusNotSequencedError.into());
                }

                if cm.inner_merge {
                    inner_merged_seen = true;
                } else if cm.status == CommitmentStatus::Included && !cm.inner_merge && inner_merged_seen {
                    return Err(SequenceCheckerError::CommitmentMergedNotSequencedError.into());
                }
            }
        }

        Ok(())
    }

    async fn check_commitment_sequence_with_handler(
        &self,
        first: &ValidateCommitment,
        data: &ValidateContractData,
    ) -> CheckerResult<()> {
        if first.status == CommitmentStatus::Included && !first.inner_merge {
            return Ok(());
        }

        let count = self.query_handler_commitment_count(data, first.status).await?;
        if count != first.leaf_index {
            Err(SequenceCheckerError::CommitmentNotSequenceWithHandlerError(count, first.leaf_index).into())
        } else {
            Ok(())
        }
    }

    async fn query_handler_commitment_count(
        &self,
        data: &ValidateContractData,
        status: CommitmentStatus,
    ) -> CheckerResult<u64> {
        let target_block = data.start_block - 1;
        let option = CommitmentQueryOption::builder()
            .chain_id(data.chain_id)
            .contract_address(data.contract_address.clone())
            .status(status)
            .end_block(target_block)
            .build();
        let query_result = self.handler.count_commitments(&option).await?;
        if query_result.end_block != target_block {
            return Err(SequenceCheckerError::TargetBlockError(target_block, query_result.end_block).into());
        }

        Ok(query_result.result)
    }
}
