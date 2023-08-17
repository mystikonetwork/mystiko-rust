use crate::handler::HandlerError;
use ethers_contract::ContractError;
use mystiko_crypto::error::MerkleTreeError;
use mystiko_ethers::provider::factory::Provider;
use thiserror::Error;

pub type CheckerResult<T> = anyhow::Result<T, RuleCheckError>;

#[derive(Error, Debug)]
pub enum RuleCheckError {
    #[error("contract address invalid {0}")]
    ContractAddressInvalid(String),
    #[error("provider not found for chain {0}")]
    ProviderNotFound(u64),
    #[error(transparent)]
    ContractError(#[from] ContractError<Provider>),
    #[error(transparent)]
    HandlerError(#[from] HandlerError),
    #[error(transparent)]
    MerkleTreeError(#[from] MerkleTreeError),
    #[error(transparent)]
    PartialCheckerError(#[from] PartialCheckerError),
    #[error(transparent)]
    SequenceCheckerError(#[from] SequenceCheckerError),
    #[error(transparent)]
    CounterCheckerError(#[from] CounterCheckerError),
    #[error(transparent)]
    TreeCheckerError(#[from] TreeCheckerError),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum PartialCheckerError {
    #[error("invalid commitment status ")]
    InvalidCommitmentStatus,
    #[error("invalid commitment src chain block number")]
    InvalidCommitmentSrcChainBlockNumber,
    #[error("invalid commitment src chain transaction hash")]
    InvalidCommitmentSrcChainTransactionHash,
    #[error("invalid commitment leaf index")]
    InvalidCommitmentLeafIndex,
    #[error("invalid commitment nullifier")]
    InvalidCommitmentNullifier,
    #[error("invalid commitment rollup fee")]
    InvalidCommitmentRollupFee,
    #[error("invalid commitment encrypted note")]
    InvalidCommitmentEncryptedNote,
    #[error("invalid commitment queued transaction hash")]
    InvalidCommitmentQueuedTransactionHash,
    #[error("invalid commitment included block number")]
    InvalidCommitmentIncludedBlockNumber,
    #[error("invalid commitment included transaction hash")]
    InvalidCommitmentIncludedTransactionHash,
}

#[derive(Error, Debug)]
pub enum SequenceCheckerError {
    #[error("target block error expected {0} actual {1}")]
    TargetBlockError(u64, u64),
    #[error("commitment data empty")]
    CommitmentDataEmpty,
    #[error("leaf index not sequenced")]
    LeafIndexNotSequenced,
    #[error("commitment status not sequenced")]
    CommitmentStatusNotSequenced,
    #[error("commitment merged not sequenced")]
    CommitmentMergedNotSequenced,
    #[error("commitments not sequenced handler {0} fetcher {1} ")]
    CommitmentNotSequenceWithHandler(u64, u64),
}

#[derive(Error, Debug)]
pub enum CounterCheckerError {
    #[error("target block error expected {0} actual {1}")]
    TargetBlockError(u64, u64),
    #[error("included count mismatch fetcher {0} provider {1}")]
    IncludedCountMismatch(u64, u64),
}

#[derive(Error, Debug)]
pub enum TreeCheckerError {
    #[error("target block error expected {0} actual {1}")]
    TargetBlockError(u64, u64),
    #[error("tree root not known")]
    TreeRootNotKnown,
}
