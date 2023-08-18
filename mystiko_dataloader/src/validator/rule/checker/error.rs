use crate::handler::HandlerError;
use ethers_contract::ContractError;
use mystiko_crypto::error::MerkleTreeError;
use mystiko_ethers::provider::factory::Provider;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuleCheckError {
    #[error("contract address {0} error")]
    ContractAddressError(String),
    #[error("provider not found for chain {0} error")]
    ProviderNotFoundError(u64),
    #[error(transparent)]
    ContractError(#[from] ContractError<Provider>),
    #[error(transparent)]
    HandlerError(#[from] HandlerError),
    #[error(transparent)]
    MerkleTreeError(#[from] MerkleTreeError),
    #[error(transparent)]
    IntegrityCheckerError(#[from] IntegrityCheckerError),
    #[error(transparent)]
    SequenceCheckerError(#[from] SequenceCheckerError),
    #[error(transparent)]
    CounterCheckerError(#[from] CounterCheckerError),
    #[error(transparent)]
    MerkleTreeCheckerError(#[from] MerkleTreeCheckerError),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum IntegrityCheckerError {
    #[error("commitment status error")]
    CommitmentStatusError,
    #[error("commitment src chain block number error")]
    CommitmentSrcChainBlockNumberError,
    #[error("commitment src chain transaction hash error")]
    CommitmentSrcChainTransactionHashError,
    #[error("commitment leaf index error")]
    CommitmentLeafIndexError,
    #[error("commitment nullifier error")]
    CommitmentNullifierError,
    #[error("commitment rollup fee error")]
    CommitmentRollupFeeError,
    #[error("commitment encrypted note error")]
    CommitmentEncryptedNoteError,
    #[error("commitment queued transaction hash error")]
    CommitmentQueuedTransactionHashError,
    #[error("commitment included block number error")]
    CommitmentIncludedBlockNumberError,
    #[error("commitment included transaction hash error")]
    CommitmentIncludedTransactionHashError,
    #[error("commitment bigger than field size error")]
    CommitmentBiggerThanFieldSizeError,
    #[error("nullifier bigger than field size error")]
    NullifierBiggerThanFieldSizeError,
}

#[derive(Error, Debug)]
pub enum SequenceCheckerError {
    #[error("target block error expected {0} actual {1}")]
    TargetBlockError(u64, u64),
    #[error("commitment data empty error")]
    CommitmentDataEmptyError,
    #[error("leaf index not sequenced error")]
    LeafIndexNotSequencedError,
    #[error("commitment status not sequenced error")]
    CommitmentStatusNotSequencedError,
    #[error("commitment merged not sequenced error")]
    CommitmentMergedNotSequencedError,
    #[error("commitments not sequenced error handler {0} fetcher {1} ")]
    CommitmentNotSequenceWithHandlerError(u64, u64),
}

#[derive(Error, Debug)]
pub enum CounterCheckerError {
    #[error("target block error expected {0} actual {1}")]
    TargetBlockError(u64, u64),
    #[error("included count mismatch error fetcher {0} provider {1}")]
    IncludedCountMismatchError(u64, u64),
}

#[derive(Error, Debug)]
pub enum MerkleTreeCheckerError {
    #[error("target block error expected {0} actual {1}")]
    TargetBlockError(u64, u64),
    #[error("merkle tree root not known error")]
    MerkleTreeRootNotKnownError,
}
