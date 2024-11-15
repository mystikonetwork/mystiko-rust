use crate::{AccountsError, CommitmentPoolContractsError, PublicAssetsError, TransactionsError, WalletsError};
use mystiko_protos::core::handler::v1::SpendInvalidCode;
use mystiko_relayer_client::error::RelayerClientError;
use mystiko_storage::StorageError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpendsError {
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error(transparent)]
    PublicAssetsError(#[from] PublicAssetsError),
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
    RelayerClientError(#[from] RelayerClientError),
    #[error(transparent)]
    HexStringError(#[from] rustc_hex::FromHexError),
    #[error(transparent)]
    ParseBigIntError(#[from] num_bigint::ParseBigIntError),
    #[error(transparent)]
    ProtocolError(#[from] mystiko_protocol::error::ProtocolError),
    #[error(transparent)]
    ProtocolKeyError(#[from] mystiko_protocol::error::ProtocolKeyError),
    #[error(transparent)]
    CryptoError(#[from] mystiko_crypto::error::CryptoError),
    #[error(transparent)]
    MerkleTreeError(#[from] mystiko_crypto::error::MerkleTreeError),
    #[error(transparent)]
    G16ProverError(#[from] mystiko_crypto::zkp::G16ProverError),
    #[error(transparent)]
    LocalWalletError(#[from] ethers_signers::WalletError),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
    #[error(transparent)]
    ProtosError(#[from] prost::DecodeError),
    #[error(transparent)]
    FetcherError(#[from] mystiko_dataloader::fetcher::FetcherError),
    #[error("missing pool contract for chain_id={0}, asset_symbol={1}, bridge_type={2:?}, version={3:?}")]
    NoPoolContractFoundError(u64, String, mystiko_types::BridgeType, Option<u32>),
    #[error("missing pool contract for chain_id={0}, contract_address={1}")]
    NoPoolContractAddressFoundError(u64, String),
    #[error("missing private key")]
    MissingPrivateKeyError,
    #[error("unsupported chain_id={0}")]
    UnsupportedChainIdError(u64),
    #[error("unsupported spend join_split_type: num_inputs={0}, num_outputs={1}")]
    UnsupportedSpendJoinSplitTypeError(u64, u64),
    #[error("invalid create options with code: {0:?}")]
    InvalidCreateOptionsError(SpendInvalidCode),
    #[error("invalid public address {0}")]
    InvalidPublicAddressError(String),
    #[error("invalid mystiko address {0}")]
    InvalidMystikoAddressError(String),
    #[error("invalid amount: {0}")]
    InvalidAmountError(f64),
    #[error("invalid rollup fee amount: {0}")]
    InvalidRollupFeeAmountError(f64),
    #[error("insufficient pool balance: {0}")]
    InsufficientPoolBalanceError(f64),
    #[error("unknown merkle tree root: {0}")]
    UnknownMerkleRootError(String),
    #[error("already spent commitment: {0}")]
    AlreadySpentCommitmentError(String),
    #[error("missing shielded_address field in commitment id={0}")]
    MissingShieldedAddressInCommitmentError(String),
    #[error("missing encrypted_note field in commitment id={0}")]
    MissingEncryptedNoteInCommitmentError(String),
    #[error("cannot find commitment_hash={0} in merkle tree")]
    MissingCommitmentInMerkleTree(String),
    #[error("shielded address {0} is not owned by us")]
    NonOwnedShieldedAddressError(String),
    #[error("missing circuit_type={0:?} in contract config")]
    MissingCircuitTypeInConfigError(mystiko_types::CircuitType),
    #[error("failed to verify the generated zk proof")]
    InvalidZKProofError,
    #[error("Spend(id={0}) is not found")]
    SpendNotFoundError(String),
    #[error("cannot send spend with status={0}")]
    SpendStatusError(String),
    #[error("missing relayer with name={0}")]
    MissingGivenRelayerError(String),
    #[error("missing transaction hash from relayer job(uuid={0})")]
    MissingTransactionHashFromRelayerJobError(String),
    #[error("missing resource: {0}")]
    MissingResourceError(String),
    #[error("raw resource: {0}")]
    RawResourceError(String),
    #[error("fetch from provider error: {0} ")]
    FetchFromProviderError(String),
}
