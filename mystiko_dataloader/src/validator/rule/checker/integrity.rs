use crate::data::{DataRef, FullData, LiteData, LoadedData};
use crate::validator::rule::{
    CheckerResult, IntegrityCheckerError, RuleChecker, ValidateMergedData, ValidateOriginalData,
};
use async_trait::async_trait;
use mystiko_config::PoolContractConfig;
use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_utils::convert::bytes_to_biguint;
use typed_builder::TypedBuilder;

pub const RULE_INTEGRITY_CHECKER_NAME: &str = "integrity";

#[derive(Debug, Default, TypedBuilder)]
pub struct IntegrityChecker<R> {
    #[builder(default = Default::default())]
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R> RuleChecker<R> for IntegrityChecker<R>
where
    R: LoadedData,
{
    fn name(&self) -> &'static str {
        RULE_INTEGRITY_CHECKER_NAME
    }

    async fn check<'a>(
        &self,
        data: &ValidateOriginalData<'a, R>,
        _merged_data: &ValidateMergedData,
    ) -> CheckerResult<()> {
        match data.contract_data.data {
            None => Ok(()),
            Some(ref d) => match R::data_ref(d) {
                DataRef::Full(full) => self.check_contract_full_data(full, data.contract_config),
                DataRef::Lite(lite) => self.check_contract_lite_data(lite, data.contract_config),
            },
        }
    }
}

impl<R> IntegrityChecker<R>
where
    R: LoadedData,
{
    fn check_contract_full_data(&self, data: &FullData, contract_cfg: &PoolContractConfig) -> CheckerResult<()> {
        self.check_commitments(&data.commitments, contract_cfg)?;
        self.check_nullifiers(&data.nullifiers, contract_cfg)
    }

    fn check_contract_lite_data(&self, data: &LiteData, contract_cfg: &PoolContractConfig) -> CheckerResult<()> {
        self.check_commitments(&data.commitments, contract_cfg)
    }

    fn check_commitments(&self, commitments: &[Commitment], contract_cfg: &PoolContractConfig) -> CheckerResult<()> {
        for cm in commitments {
            self.check_commitment(cm, contract_cfg)?;
        }

        Ok(())
    }

    fn check_commitment(&self, commitment: &Commitment, contract_cfg: &PoolContractConfig) -> CheckerResult<()> {
        let status = match CommitmentStatus::from_i32(commitment.status) {
            Some(status) => status,
            None => return Err(IntegrityCheckerError::CommitmentStatusError.into()),
        };

        match status {
            CommitmentStatus::Unspecified => return Err(IntegrityCheckerError::CommitmentStatusError.into()),
            CommitmentStatus::SrcSucceeded => {
                match commitment.src_chain_block_number {
                    None => {
                        return Err(IntegrityCheckerError::CommitmentSrcChainBlockNumberError.into());
                    }
                    Some(src_chain_block_number) => {
                        if src_chain_block_number != commitment.block_number {
                            return Err(IntegrityCheckerError::CommitmentSrcChainNumberMismatchError(
                                src_chain_block_number,
                                commitment.block_number,
                            )
                            .into());
                        }

                        if src_chain_block_number <= contract_cfg.start_block() {
                            return Err(IntegrityCheckerError::CommitmentSrcChainNumberTooSmallError(
                                src_chain_block_number,
                                contract_cfg.start_block(),
                            )
                            .into());
                        }
                    }
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
                match commitment.included_block_number {
                    None => {
                        return Err(IntegrityCheckerError::CommitmentIncludedBlockNumberError.into());
                    }
                    Some(included_block_number) => {
                        if included_block_number < commitment.block_number {
                            return Err(IntegrityCheckerError::CommitmentIncludedNumberTooSmallError(
                                included_block_number,
                                "block number".to_string(),
                                contract_cfg.start_block(),
                            )
                            .into());
                        }
                        if included_block_number < contract_cfg.start_block() {
                            return Err(IntegrityCheckerError::CommitmentIncludedNumberTooSmallError(
                                included_block_number,
                                "contract start block".to_string(),
                                contract_cfg.start_block(),
                            )
                            .into());
                        }
                    }
                }

                if commitment.included_transaction_hash.is_none() {
                    return Err(IntegrityCheckerError::CommitmentIncludedTransactionHashError.into());
                }
            }
        }

        if bytes_to_biguint(&commitment.commitment_hash).ge(&FIELD_SIZE) {
            return Err(IntegrityCheckerError::CommitmentBiggerThanFieldSizeError.into());
        }

        if commitment.block_number <= contract_cfg.start_block() {
            return Err(IntegrityCheckerError::CommitmentBlockNumberTooSmallError(
                commitment.block_number,
                contract_cfg.start_block(),
            )
            .into());
        }

        Ok(())
    }

    fn check_nullifiers(&self, nullifiers: &[Nullifier], contract_cfg: &PoolContractConfig) -> CheckerResult<()> {
        for n in nullifiers {
            self.check_nullifier(n, contract_cfg)?;
        }

        Ok(())
    }

    fn check_nullifier(&self, n: &Nullifier, contract_cfg: &PoolContractConfig) -> CheckerResult<()> {
        if bytes_to_biguint(&n.nullifier).ge(&FIELD_SIZE) {
            return Err(IntegrityCheckerError::NullifierBiggerThanFieldSizeError.into());
        }

        if n.block_number <= contract_cfg.start_block() {
            return Err(IntegrityCheckerError::NullifierBlockNumberTooSmallError(
                n.block_number,
                contract_cfg.start_block(),
            )
            .into());
        }

        Ok(())
    }
}
