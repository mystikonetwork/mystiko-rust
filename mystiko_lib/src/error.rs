use mystiko_core::{AccountsError, DepositsError, MystikoError, ScannerError, SynchronizerError, WalletsError};
use mystiko_protos::api::v1::{
    AccountError as ProtoAccountError, DepositError as ProtoDepositError, MystikoError as ProtoMystikoError,
    ScannerError as ProtoScannerError, StatusCode, SynchronizerError as ProtoSynchronizerError,
    WalletError as ProtoWalletError,
};

pub fn parse_mystiko_error(err: &MystikoError) -> StatusCode {
    match err {
        MystikoError::SynchronizerError(_) => ProtoMystikoError::SynchronizerError.into(),
        MystikoError::ScannerError(_) => ProtoMystikoError::ScannerError.into(),
        MystikoError::DataLoaderError(_) => ProtoMystikoError::DataloaderError.into(),
        MystikoError::ConfigError(_) => ProtoMystikoError::ConfigError.into(),
        MystikoError::DatabaseMigrationError(_) => ProtoMystikoError::DatabaseMigrationError.into(),
        MystikoError::InvalidProviderUrlError(_) => ProtoMystikoError::InvalidProviderUrlError.into(),
        MystikoError::RelayerClientError(_) => ProtoMystikoError::RelayerClientError.into(),
    }
}

pub fn parse_account_error(err: &AccountsError) -> StatusCode {
    match err {
        AccountsError::StorageError(_) => ProtoAccountError::StorageError.into(),
        AccountsError::CryptoError(_) => ProtoAccountError::CryptoError.into(),
        AccountsError::MnemonicError(_) => ProtoAccountError::MnemonicError.into(),
        AccountsError::HexStringError(_) => ProtoAccountError::HexStringError.into(),
        AccountsError::WalletsError(_) => ProtoAccountError::WalletsError.into(),
        AccountsError::NoSuchAccountError(_, _) => ProtoAccountError::NoSuchAccountError.into(),
        AccountsError::ProtocolKeyError(_) => ProtoAccountError::ProtocolKeyError.into(),
    }
}

pub fn parse_wallet_error(err: &WalletsError) -> StatusCode {
    match err {
        WalletsError::StorageError(_) => ProtoWalletError::StorageError.into(),
        WalletsError::CryptoError(_) => ProtoWalletError::CryptoError.into(),
        WalletsError::HexStringError(_) => ProtoWalletError::HexStringError.into(),
        WalletsError::MnemonicError(_) => ProtoWalletError::MnemonicError.into(),
        WalletsError::InvalidPasswordError(_) => ProtoWalletError::InvalidPasswordError.into(),
        WalletsError::MismatchedPasswordError => ProtoWalletError::MismatchedPasswordError.into(),
        WalletsError::NoExistingWalletError => ProtoWalletError::NoExistingWalletError.into(),
    }
}

pub fn parse_deposit_error(err: &DepositsError) -> StatusCode {
    match err {
        DepositsError::HexStringError(_) => ProtoDepositError::HexStringError.into(),
        DepositsError::FromDecStrError(_) => ProtoDepositError::FromDecStrError.into(),
        DepositsError::ParseBytesError(_) => ProtoDepositError::ParseBytesError.into(),
        DepositsError::ProviderError(_) => ProtoDepositError::ProviderError.into(),
        DepositsError::AnyhowError(_) => ProtoDepositError::AnyhowError.into(),
        DepositsError::PublicAssetsError(_) => ProtoDepositError::PublicAssetsError.into(),
        DepositsError::DepositContractsError(_) => ProtoDepositError::DepositContractsError.into(),
        DepositsError::CommitmentPoolContractsError(_) => ProtoDepositError::CommitmentPoolContractsError.into(),
        DepositsError::TransactionsError(_) => ProtoDepositError::TransactionsError.into(),
        DepositsError::ProtocolError(_) => ProtoDepositError::ProtocolError.into(),
        DepositsError::StorageError(_) => ProtoDepositError::StorageError.into(),
        DepositsError::WalletsError(_) => ProtoDepositError::WalletsError.into(),
        DepositsError::AccountsError(_) => ProtoDepositError::AccountsError.into(),
        DepositsError::ParseBigIntError(_) => ProtoDepositError::ParseBigIntError.into(),
        DepositsError::UnsupportedChainIdError(_) => ProtoDepositError::UnsupportedChainIdError.into(),
        DepositsError::NoDepositContractFoundError(_, _, _, _) => ProtoDepositError::NoDepositContractFoundError.into(),
        DepositsError::InvalidDepositAmountError(_, _, _) => ProtoDepositError::InvalidDepositAmountError.into(),
        DepositsError::InvalidRollupFeeAmountError(_, _) => ProtoDepositError::InvalidRollupFeeAmountError.into(),
        DepositsError::InvalidBridgeFeeAmountError(_, _) => ProtoDepositError::InvalidBridgeFeeAmountError.into(),
        DepositsError::InvalidExecutorFeeAmountError(_, _) => ProtoDepositError::InvalidExecutorFeeAmountError.into(),
        DepositsError::InsufficientBalanceError(_, _) => ProtoDepositError::InsufficientBalanceError.into(),
        DepositsError::IdNotFoundError(_) => ProtoDepositError::IdNotFoundError.into(),
        DepositsError::MissingPrivateKeyError => ProtoDepositError::MissingPrivateKeyError.into(),
        DepositsError::DepositStatusError(_) => ProtoDepositError::DepositStatusError.into(),
        DepositsError::DuplicateCommitmentError(_, _, _) => ProtoDepositError::DuplicateCommitmentError.into(),
        DepositsError::ProtocolKeyError(_) => ProtoDepositError::ProtocolKeyError.into(),
    }
}

pub fn parse_synchronizer_error(err: &SynchronizerError) -> StatusCode {
    match err {
        SynchronizerError::UnsupportedChainError(_) => ProtoSynchronizerError::UnsupportedChainError.into(),
        SynchronizerError::DataLoaderError(_) => ProtoSynchronizerError::DataloaderError.into(),
        SynchronizerError::DataLoaderConfigError(_) => ProtoSynchronizerError::DataloaderConfigError.into(),
        SynchronizerError::AnyhowError(_) => ProtoSynchronizerError::AnyhowError.into(),
    }
}

pub fn parse_scanner_error(err: &ScannerError) -> StatusCode {
    match err {
        ScannerError::NoSuchAccountError => ProtoScannerError::NoSuchAccountError.into(),
        ScannerError::NoSuchContractConfigError(_, _) => ProtoScannerError::NoSuchContractConfigError.into(),
        ScannerError::CommitmentEmptyError => ProtoScannerError::CommitmentEmptyError.into(),
        ScannerError::AccountHandlerError(_) => ProtoScannerError::AccountHandlerError.into(),
        ScannerError::WalletHandlerError(_) => ProtoScannerError::WalletHandlerError.into(),
        ScannerError::CryptoError(_) => ProtoScannerError::CryptoError.into(),
        ScannerError::StorageError(_) => ProtoScannerError::StorageError.into(),
        ScannerError::JoinError(_) => ProtoScannerError::JoinError.into(),
        ScannerError::ProtocolError(_) => ProtoScannerError::ProtocolError.into(),
        ScannerError::FromHexError(_) => ProtoScannerError::FromHexError.into(),
        ScannerError::AnyhowError(_) => ProtoScannerError::AnyhowError.into(),
        ScannerError::ProtocolKeyError(_) => ProtoScannerError::ProtocolKeyError.into(),
    }
}
