use crate::{
    AccountsError, CommitmentPoolContractsError, DepositContractsError, PublicAssetsError, TransactionsError,
    WalletsError,
};
use mystiko_storage::StorageError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DepositsError {
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error(transparent)]
    HexStringError(#[from] rustc_hex::FromHexError),
    #[error(transparent)]
    FromDecStrError(#[from] ethers_core::abi::ethereum_types::FromDecStrErr),
    #[error(transparent)]
    ParseBytesError(#[from] ethers_core::types::ParseBytesError),
    #[error(transparent)]
    ProviderError(#[from] ethers_providers::ProviderError),
    #[error(transparent)]
    ProtocolError(#[from] mystiko_protocol::error::ProtocolError),
    #[error(transparent)]
    ProtocolKeyError(#[from] mystiko_protocol::error::ProtocolKeyError),
    #[error(transparent)]
    PublicAssetsError(#[from] PublicAssetsError),
    #[error(transparent)]
    DepositContractsError(#[from] DepositContractsError),
    #[error(transparent)]
    CommitmentPoolContractsError(#[from] CommitmentPoolContractsError),
    #[error(transparent)]
    TransactionsError(#[from] TransactionsError),
    #[error(transparent)]
    StorageError(#[from] StorageError),
    #[error(transparent)]
    WalletsError(#[from] WalletsError),
    #[error(transparent)]
    AccountsError(#[from] AccountsError),
    #[error(transparent)]
    ParseBigIntError(#[from] num_bigint::ParseBigIntError),
    #[error("unsupported chain_id={0}")]
    UnsupportedChainIdError(u64),
    #[error("no deposit contract found for chain_id={0} asset_symbol={1}, dst_chain_id={2:?}, bridge_type={3:?}")]
    NoDepositContractFoundError(u64, String, u64, mystiko_types::BridgeType),
    #[error("deposit amount {0} is less than min_amount {1} or greater than max_amount {2}")]
    InvalidDepositAmountError(f64, f64, f64),
    #[error("rollup fee amount {0} is less than min_rollup_fee_amount {1}")]
    InvalidRollupFeeAmountError(f64, f64),
    #[error("bridge fee amount {0} is less than min_bridge_fee_amount {1}")]
    InvalidBridgeFeeAmountError(f64, f64),
    #[error("executor fee amount {0} is less than min_executor_fee_amount {1}")]
    InvalidExecutorFeeAmountError(f64, f64),
    #[error("insufficient balance for asset {0} amount {1}")]
    InsufficientBalanceError(String, f64),
    #[error("deposit with id {0} not found")]
    IdNotFoundError(String),
    #[error("missing private key")]
    MissingPrivateKeyError,
    #[error("cannot send deposit with status={0}")]
    DepositStatusError(String),
    #[error("duplicate commitment={0} in chain_id={1} contract_address={2}")]
    DuplicateCommitmentError(String, u64, String),
}
