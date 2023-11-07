use mystiko_core::{AccountHandlerError, MystikoError, WalletHandlerError};
use mystiko_protos::api::v1::StatusCode;

pub fn parse_mystiko_error(err: &MystikoError) -> StatusCode {
    match err {
        MystikoError::SynchronizerError(_) => StatusCode::SynchronizerError,
        MystikoError::DataLoaderError(_) => StatusCode::DataLoaderError,
        MystikoError::ConfigError(_) => StatusCode::ConfigError,
        MystikoError::DatabaseMigrationError(_) => StatusCode::DatabaseMigrationError,
        MystikoError::InvalidProviderUrlError(_) => StatusCode::InvalidProviderUrlError,
    }
}

pub fn parse_account_error(err: &AccountHandlerError) -> StatusCode {
    match err {
        AccountHandlerError::StorageError(_) => StatusCode::StorageError,
        AccountHandlerError::CryptoError(_) => StatusCode::CryptoError,
        AccountHandlerError::MnemonicError(_) => StatusCode::MnemonicError,
        AccountHandlerError::HexStringError(_) => StatusCode::HexStringError,
        AccountHandlerError::WalletHandlerError(err) => parse_wallet_error(err),
        AccountHandlerError::NoSuchAccountError(_, _) => StatusCode::NoSuchAccountError,
    }
}

pub fn parse_wallet_error(err: &WalletHandlerError) -> StatusCode {
    match err {
        WalletHandlerError::StorageError(_) => StatusCode::StorageError,
        WalletHandlerError::CryptoError(_) => StatusCode::CryptoError,
        WalletHandlerError::HexStringError(_) => StatusCode::HexStringError,
        WalletHandlerError::MnemonicError(_) => StatusCode::MnemonicError,
        WalletHandlerError::InvalidPasswordError(_) => StatusCode::InvalidPasswordError,
        WalletHandlerError::MismatchedPasswordError => StatusCode::MismatchedPasswordError,
        WalletHandlerError::NoExistingWalletError => StatusCode::NoExistingWalletError,
    }
}
