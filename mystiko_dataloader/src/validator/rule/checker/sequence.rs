use crate::data::LoadedData;
use crate::handler::{CommitmentQueryOption, DataHandler};
use crate::validator::rule::checker::RuleChecker;
use crate::validator::rule::types::ValidateMergedData;
use crate::validator::rule::{CheckerResult, ValidateCommitment};
use crate::validator::rule::{SequenceCheckerError, ValidateOriginalData};
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
    async fn check<'a>(
        &self,
        _data: &ValidateOriginalData<'a, R>,
        merged_data: &ValidateMergedData,
    ) -> CheckerResult<()> {
        match merged_data.commitments.first() {
            Some(first) => {
                self.check_commitment_status_sequence(first, merged_data)?;
                self.check_commitment_leaf_index_sequence(merged_data).await
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
    fn check_commitment_status_sequence(
        &self,
        first: &ValidateCommitment,
        data: &ValidateMergedData,
    ) -> CheckerResult<()> {
        if first.status == CommitmentStatus::Queued || first.status == CommitmentStatus::SrcSucceeded {
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

    async fn check_commitment_leaf_index_sequence(&self, data: &ValidateMergedData) -> CheckerResult<()> {
        self.check_commitment_leaf_index_sequence_by_status(data, CommitmentStatus::SrcSucceeded)
            .await?;
        self.check_commitment_leaf_index_sequence_by_status(data, CommitmentStatus::Included)
            .await?;
        self.check_commitment_leaf_index_sequence_by_status(data, CommitmentStatus::Queued)
            .await
    }

    async fn check_commitment_leaf_index_sequence_by_status(
        &self,
        data: &ValidateMergedData,
        status: CommitmentStatus,
    ) -> CheckerResult<()> {
        let cms = data
            .commitments
            .iter()
            .filter(|cm| {
                if status == CommitmentStatus::Queued {
                    cm.status == status || cm.inner_merge
                } else {
                    cm.status == status
                }
            })
            .collect::<Vec<_>>();

        self.check_commitment_inner_leaf_index_sequence(cms.as_slice()).await?;
        self.check_commitment_leaf_index_sequence_with_handler(data, cms.as_slice(), status)
            .await
    }

    async fn check_commitment_inner_leaf_index_sequence(&self, cms: &[&ValidateCommitment]) -> CheckerResult<()> {
        if let Some(window) = cms
            .windows(2)
            .find(|&window| window[0].leaf_index + 1 != window[1].leaf_index)
        {
            return Err(
                SequenceCheckerError::LeafIndexNotSequencedError(window[0].leaf_index, window[1].leaf_index).into(),
            );
        }
        Ok(())
    }

    async fn check_commitment_leaf_index_sequence_with_handler(
        &self,
        data: &ValidateMergedData,
        cms: &[&ValidateCommitment],
        status: CommitmentStatus,
    ) -> CheckerResult<()> {
        if let Some(first) = cms.first() {
            let count = self.sum_handler_commitment_count(data, status).await?;
            if count != first.leaf_index {
                return Err(SequenceCheckerError::CommitmentNotSequenceWithHandlerError(
                    status as i32,
                    count,
                    first.leaf_index,
                )
                .into());
            }
        }

        Ok(())
    }

    async fn sum_handler_commitment_count(
        &self,
        data: &ValidateMergedData,
        status: CommitmentStatus,
    ) -> CheckerResult<u64> {
        match status {
            CommitmentStatus::Unspecified => Err(SequenceCheckerError::CommitmentStatusError.into()),
            CommitmentStatus::SrcSucceeded | CommitmentStatus::Included => {
                self.query_handler_commitment_count(data, status).await
            }
            CommitmentStatus::Queued => {
                let included_count = self
                    .query_handler_commitment_count(data, CommitmentStatus::Included)
                    .await?;
                let queued_count = self
                    .query_handler_commitment_count(data, CommitmentStatus::Queued)
                    .await?;
                Ok(included_count + queued_count)
            }
        }
    }

    async fn query_handler_commitment_count(
        &self,
        data: &ValidateMergedData,
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
