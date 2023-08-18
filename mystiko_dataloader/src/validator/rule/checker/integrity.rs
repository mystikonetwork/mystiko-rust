use crate::data::{DataRef, FullData, LiteData, LoadedData};
use crate::validator::rule::{CheckerResult, IntegrityCheckerError, RuleCheckData, RuleChecker};
use async_trait::async_trait;
use mystiko_protos::data::v1::{Commitment, CommitmentStatus};
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct IntegrityChecker<R> {
    #[builder(default = Default::default())]
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R> RuleChecker<R> for IntegrityChecker<R>
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

impl<R> IntegrityChecker<R>
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
            None => return Err(IntegrityCheckerError::InvalidCommitmentStatus.into()),
        };

        match status {
            CommitmentStatus::Unspecified => return Err(IntegrityCheckerError::InvalidCommitmentStatus.into()),
            CommitmentStatus::SrcSucceeded => {
                if commitment.src_chain_block_number.is_none() {
                    return Err(IntegrityCheckerError::InvalidCommitmentSrcChainBlockNumber.into());
                }

                if commitment.src_chain_transaction_hash.is_none() {
                    return Err(IntegrityCheckerError::InvalidCommitmentSrcChainTransactionHash.into());
                }
            }
            CommitmentStatus::Queued => {
                if commitment.leaf_index.is_none() {
                    return Err(IntegrityCheckerError::InvalidCommitmentLeafIndex.into());
                }

                if commitment.rollup_fee.is_none() {
                    return Err(IntegrityCheckerError::InvalidCommitmentRollupFee.into());
                }

                if commitment.encrypted_note.is_none() {
                    return Err(IntegrityCheckerError::InvalidCommitmentEncryptedNote.into());
                }

                if commitment.queued_transaction_hash.is_none() {
                    return Err(IntegrityCheckerError::InvalidCommitmentQueuedTransactionHash.into());
                }
            }
            CommitmentStatus::Included => {
                if commitment.included_block_number.is_none() {
                    return Err(IntegrityCheckerError::InvalidCommitmentIncludedBlockNumber.into());
                }

                if commitment.included_transaction_hash.is_none() {
                    return Err(IntegrityCheckerError::InvalidCommitmentIncludedTransactionHash.into());
                }
            }
        }

        Ok(())
    }
}
