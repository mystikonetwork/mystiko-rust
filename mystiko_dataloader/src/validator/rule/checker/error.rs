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
pub enum MerkleTreeCheckerError {
    #[error("target block error expected {0} actual {1}")]
    TargetBlockError(u64, u64),
    #[error("merkle tree root not known")]
    MerkleTreeRootNotKnown,
}
