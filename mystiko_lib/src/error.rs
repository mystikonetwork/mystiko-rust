use mystiko_core::{AccountsError, MystikoError, WalletsError};
use mystiko_protos::api::v1::StatusCode;

pub fn parse_mystiko_error(err: &MystikoError) -> StatusCode {
    match err {
        MystikoError::SynchronizerError(_) => StatusCode::SynchronizerError,
        MystikoError::ScannerError(_) => StatusCode::ScannerError,
        MystikoError::DataLoaderError(_) => StatusCode::DataLoaderError,
        MystikoError::ConfigError(_) => StatusCode::ConfigError,
        MystikoError::DatabaseMigrationError(_) => StatusCode::DatabaseMigrationError,
        MystikoError::InvalidProviderUrlError(_) => StatusCode::InvalidProviderUrlError,
    }
}

pub fn parse_account_error(err: &AccountsError) -> StatusCode {
    match err {
        AccountsError::StorageError(_) => StatusCode::StorageError,
        AccountsError::CryptoError(_) => StatusCode::CryptoError,
        AccountsError::MnemonicError(_) => StatusCode::MnemonicError,
        AccountsError::HexStringError(_) => StatusCode::HexStringError,
        AccountsError::WalletsError(err) => parse_wallet_error(err),
        AccountsError::NoSuchAccountError(_, _) => StatusCode::NoSuchAccountError,
    }
}

pub fn parse_wallet_error(err: &WalletsError) -> StatusCode {
    match err {
        WalletsError::StorageError(_) => StatusCode::StorageError,
        WalletsError::CryptoError(_) => StatusCode::CryptoError,
        WalletsError::HexStringError(_) => StatusCode::HexStringError,
        WalletsError::MnemonicError(_) => StatusCode::MnemonicError,
        WalletsError::InvalidPasswordError(_) => StatusCode::InvalidPasswordError,
        WalletsError::MismatchedPasswordError => StatusCode::MismatchedPasswordError,
        WalletsError::NoExistingWalletError => StatusCode::NoExistingWalletError,
    }
}
