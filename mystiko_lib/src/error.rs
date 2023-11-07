use mystiko_core::MystikoError;
use mystiko_protos::api::v1::StatusCode;

pub fn parse_mystiko_error(err: &MystikoError) -> StatusCode {
    match err {
        MystikoError::ConfigError(_) => StatusCode::ConfigError,
        MystikoError::DatabaseMigrationError(_) => StatusCode::DatabaseMigrationError,
        MystikoError::CryptoError(_) => StatusCode::CryptoError,
        MystikoError::MnemonicError(_) => StatusCode::MnemonicError,
        MystikoError::HexStringError(_) => StatusCode::HexStringError,
        MystikoError::StorageError(_) => StatusCode::StorageError,
        MystikoError::InvalidPasswordError(_) => StatusCode::InvalidPasswordError,
        MystikoError::MismatchedPasswordError => StatusCode::MismatchedPasswordError,
        MystikoError::NoExistingWalletError => StatusCode::NoExistingWalletError,
        MystikoError::NoSuchAccountError(_, _) => StatusCode::NoSuchAccountError,
        MystikoError::InvalidProviderUrlError(_) => StatusCode::InvalidProviderUrlError,
        MystikoError::DataLoaderError(_) => StatusCode::DataLoaderError,
    }
}
