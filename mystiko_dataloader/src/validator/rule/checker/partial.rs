use crate::data::{DataRef, FullData, LiteData, LoadedData};
use crate::validator::rule::{CheckerResult, PartialCheckerError, RuleCheckData, RuleChecker};
use async_trait::async_trait;
use mystiko_protos::data::v1::{Commitment, CommitmentStatus};
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct PartialChecker<R> {
    #[builder(default = Default::default())]
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R> RuleChecker<R> for PartialChecker<R>
where
    R: LoadedData,
{
    async fn check(&self, data: &RuleCheckData<R>) -> CheckerResult<()> {
        match data.contract_data.data {
            None => Ok(()),
            Some(ref d) => match R::data_ref(d) {
                DataRef::Full(full) => self.check_contract_full_data(full).await,
                DataRef::Lite(lite) => self.check_contract_lite_data(lite).await,
            },
        }
    }
}

impl<R> PartialChecker<R>
where
    R: LoadedData,
{
    async fn check_contract_full_data(&self, data: &FullData) -> CheckerResult<()> {
        self.check_commitments(&data.commitments).await
    }

    async fn check_contract_lite_data(&self, data: &LiteData) -> CheckerResult<()> {
        self.check_commitments(&data.commitments).await
    }

    async fn check_commitments(&self, commitments: &[Commitment]) -> CheckerResult<()> {
        for cm in commitments {
            self.check_commitment(cm).await?;
        }

        Ok(())
    }

    async fn check_commitment(&self, commitment: &Commitment) -> CheckerResult<()> {
        let status = match CommitmentStatus::from_i32(commitment.status) {
            Some(status) => status,
            None => return Err(PartialCheckerError::InvalidCommitmentStatus.into()),
        };

        match status {
            CommitmentStatus::Unspecified => Err(PartialCheckerError::InvalidCommitmentStatus.into()),
            CommitmentStatus::SrcSucceeded => {
                if commitment.src_chain_block_number.is_none() {
                    return Err(PartialCheckerError::InvalidCommitmentSrcChainBlockNumber.into());
                }

                if commitment.src_chain_transaction_hash.is_none() {
                    return Err(PartialCheckerError::InvalidCommitmentSrcChainTransactionHash.into());
                }

                Ok(())
            }
            CommitmentStatus::Queued => {
                if commitment.leaf_index.is_none() {
                    return Err(PartialCheckerError::InvalidCommitmentLeafIndex.into());
                }

                if commitment.rollup_fee.is_none() {
                    return Err(PartialCheckerError::InvalidCommitmentRollupFee.into());
                }

                if commitment.encrypted_note.is_none() {
                    return Err(PartialCheckerError::InvalidCommitmentEncryptedNote.into());
                }

                if commitment.queued_transaction_hash.is_none() {
                    return Err(PartialCheckerError::InvalidCommitmentQueuedTransactionHash.into());
                }

                Ok(())
            }
            CommitmentStatus::Included => {
                if commitment.included_block_number.is_none() {
                    return Err(PartialCheckerError::InvalidCommitmentIncludedBlockNumber.into());
                }

                if commitment.included_transaction_hash.is_none() {
                    return Err(PartialCheckerError::InvalidCommitmentIncludedTransactionHash.into());
                }

                Ok(())
            }
        }
    }
}
