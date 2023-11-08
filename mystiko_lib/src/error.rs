use mystiko_core::{AccountHandlerV1Error, MystikoError, WalletHandlerV1Error};
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

pub fn parse_account_error(err: &AccountHandlerV1Error) -> StatusCode {
    match err {
        AccountHandlerV1Error::StorageError(_) => StatusCode::StorageError,
        AccountHandlerV1Error::CryptoError(_) => StatusCode::CryptoError,
        AccountHandlerV1Error::MnemonicError(_) => StatusCode::MnemonicError,
        AccountHandlerV1Error::HexStringError(_) => StatusCode::HexStringError,
        AccountHandlerV1Error::WalletHandlerError(err) => parse_wallet_error(err),
        AccountHandlerV1Error::NoSuchAccountError(_, _) => StatusCode::NoSuchAccountError,
    }
}

pub fn parse_wallet_error(err: &WalletHandlerV1Error) -> StatusCode {
    match err {
        WalletHandlerV1Error::StorageError(_) => StatusCode::StorageError,
        WalletHandlerV1Error::CryptoError(_) => StatusCode::CryptoError,
        WalletHandlerV1Error::HexStringError(_) => StatusCode::HexStringError,
        WalletHandlerV1Error::MnemonicError(_) => StatusCode::MnemonicError,
        WalletHandlerV1Error::InvalidPasswordError(_) => StatusCode::InvalidPasswordError,
        WalletHandlerV1Error::MismatchedPasswordError => StatusCode::MismatchedPasswordError,
        WalletHandlerV1Error::NoExistingWalletError => StatusCode::NoExistingWalletError,
    }
}
