use crate::data::{DataRef, FullData, LiteData, LoadedData};
use crate::validator::rule::{CheckerResult, IntegrityCheckerError, RuleCheckData, RuleChecker};
use async_trait::async_trait;
use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_utils::convert::bytes_to_biguint;
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
        self.check_commitments(&data.commitments).await?;
        self.check_nullifiers(&data.nullifiers).await
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
            None => return Err(IntegrityCheckerError::CommitmentStatusError.into()),
        };

        match status {
            CommitmentStatus::Unspecified => return Err(IntegrityCheckerError::CommitmentStatusError.into()),
            CommitmentStatus::SrcSucceeded => {
                if commitment.src_chain_block_number.is_none() {
                    return Err(IntegrityCheckerError::CommitmentSrcChainBlockNumberError.into());
                }

                if commitment.src_chain_transaction_hash.is_none() {
                    return Err(IntegrityCheckerError::CommitmentSrcChainTransactionHashError.into());
                }
            }
            CommitmentStatus::Queued => {
                if commitment.leaf_index.is_none() {
                    return Err(IntegrityCheckerError::CommitmentLeafIndexError.into());
                }

                if commitment.rollup_fee.is_none() {
                    return Err(IntegrityCheckerError::CommitmentRollupFeeError.into());
                }

                if commitment.encrypted_note.is_none() {
                    return Err(IntegrityCheckerError::CommitmentEncryptedNoteError.into());
                }

                if commitment.queued_transaction_hash.is_none() {
                    return Err(IntegrityCheckerError::CommitmentQueuedTransactionHashError.into());
                }
            }
            CommitmentStatus::Included => {
                if commitment.included_block_number.is_none() {
                    return Err(IntegrityCheckerError::CommitmentIncludedBlockNumberError.into());
                }

                if commitment.included_transaction_hash.is_none() {
                    return Err(IntegrityCheckerError::CommitmentIncludedTransactionHashError.into());
                }
            }
        }

        if bytes_to_biguint(&commitment.commitment_hash).ge(&FIELD_SIZE) {
            return Err(IntegrityCheckerError::CommitmentBiggerThanFieldSizeError.into());
        }

        Ok(())
    }

    async fn check_nullifiers(&self, nullifiers: &[Nullifier]) -> CheckerResult<()> {
        for n in nullifiers {
            if bytes_to_biguint(&n.nullifier).ge(&FIELD_SIZE) {
                return Err(IntegrityCheckerError::NullifierBiggerThanFieldSizeError.into());
            }
        }

        Ok(())
    }
}
