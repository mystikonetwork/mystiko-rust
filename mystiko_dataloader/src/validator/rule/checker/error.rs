use crate::handler::HandlerError;
use ethers_contract::ContractError;
use mystiko_crypto::error::MerkleTreeError;
use mystiko_ethers::Provider;
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
    #[error("integrity checker chain {0} config not found error")]
    ChainConfigNotFoundError(u64),
    #[error("integrity checker chain {0} contract {1} config not found error")]
    ContractConfigNotFoundError(u64, String),
    #[error("integrity checker commitment status error")]
    CommitmentStatusError,
    #[error("integrity checker commitment src chain block number error")]
    CommitmentSrcChainBlockNumberError,
    #[error("integrity checker commitment src chain transaction hash error")]
    CommitmentSrcChainTransactionHashError,
    #[error("integrity checker commitment leaf index error")]
    CommitmentLeafIndexError,
    #[error("integrity checker commitment nullifier error")]
    CommitmentNullifierError,
    #[error("integrity checker commitment rollup fee error")]
    CommitmentRollupFeeError,
    #[error("integrity checker commitment encrypted note error")]
    CommitmentEncryptedNoteError,
    #[error("integrity checker commitment queued transaction hash error")]
    CommitmentQueuedTransactionHashError,
    #[error("integrity checker commitment included block number error")]
    CommitmentIncludedBlockNumberError,
    #[error("integrity checker commitment included transaction hash error")]
    CommitmentIncludedTransactionHashError,
    #[error("integrity checker commitment bigger than field size error")]
    CommitmentBiggerThanFieldSizeError,
    #[error("integrity checker nullifier bigger than field size error")]
    NullifierBiggerThanFieldSizeError,
    #[error("integrity checker commitment block number {0} less than contract start block {1} error")]
    CommitmentBlockNumberTooSmallError(u64, u64),
    #[error("integrity checker commitment src chain block number {0} not equal with block number {1} error")]
    CommitmentSrcChainNumberMismatchError(u64, u64),
    #[error("integrity checker commitment src chain block number {0} less than contract start block {1} error")]
    CommitmentSrcChainNumberTooSmallError(u64, u64),
    #[error("integrity checker commitment included block number {0} less than {1} {2} error")]
    CommitmentIncludedNumberTooSmallError(u64, String, u64),
    #[error("integrity checker nullifier block number {0} less than contract start block {1} error")]
    NullifierBlockNumberTooSmallError(u64, u64),
}

#[derive(Error, Debug)]
pub enum SequenceCheckerError {
    #[error("sequence checker target block error expected {0} actual {1}")]
    TargetBlockError(u64, u64),
    #[error("sequence checker commitment data empty error")]
    CommitmentDataEmptyError,
    #[error("sequence checker leaf index not sequenced error previous {0} next {1}")]
    LeafIndexNotSequencedError(u64, u64),
    #[error("sequence checker commitment status error")]
    CommitmentStatusError,
    #[error("sequence checker commitment status not sequenced error")]
    CommitmentStatusNotSequencedError,
    #[error("sequence checker commitment merged not sequenced error")]
    CommitmentMergedNotSequencedError,
    #[error("sequence checker commitments not sequenced with handler status {0} handler {1} fetcher {2} ")]
    CommitmentNotSequenceWithHandlerError(i32, u64, u64),
}

#[derive(Error, Debug)]
pub enum CounterCheckerError {
    #[error("counter checker target block error expected {0} actual {1}")]
    TargetBlockError(u64, u64),
    #[error("counter checker fetch commitment after contract disabled error")]
    CommitmentAfterContractDisabledError,
    #[error("counter checker commitment included count mismatch error fetcher {0} provider {1}")]
    CommitmentIncludedCountMismatchError(u64, u64),
    #[error("counter checker commitment total count mismatch error fetcher {0} provider {1}")]
    CommitmentCountMismatchError(u64, u64),
    #[error("counter checker nullifier total count mismatch error fetcher {0} provider {1}")]
    NullifierCountMismatchError(u64, u64),
}

#[derive(Error, Debug)]
pub enum MerkleTreeCheckerError {
    #[error("merkle tree checker target block error expected {0} actual {1}")]
    TargetBlockError(u64, u64),
    #[error("merkle tree checker merkle tree root not known error")]
    MerkleTreeRootNotKnownError,
}
