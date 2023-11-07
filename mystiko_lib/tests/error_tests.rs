use anyhow::anyhow;
use mystiko_core::{AccountHandlerError, MystikoError, SynchronizerError, WalletHandlerError};
use mystiko_crypto::error::CryptoError;
use mystiko_dataloader::DataLoaderError;
use mystiko_lib::error::{parse_account_error, parse_mystiko_error, parse_wallet_error};
use mystiko_protos::api::v1::StatusCode;
use mystiko_storage::StorageError;

#[test]
fn test_parse_mystiko_error() {
    let e1: MystikoError = MystikoError::SynchronizerError(SynchronizerError::UnsupportedChainError(1));
    let e2: MystikoError = MystikoError::DataLoaderError(DataLoaderError::LoaderNoContractsError);
    let e3: MystikoError = MystikoError::ConfigError(anyhow!("error"));
    let e4: MystikoError = MystikoError::DatabaseMigrationError(anyhow!("error"));
    let e5: MystikoError = MystikoError::InvalidProviderUrlError("_".to_string());
    assert_eq!(parse_mystiko_error(&e1), StatusCode::SynchronizerError);
    assert_eq!(parse_mystiko_error(&e2), StatusCode::DataLoaderError);
    assert_eq!(parse_mystiko_error(&e3), StatusCode::ConfigError);
    assert_eq!(parse_mystiko_error(&e4), StatusCode::DatabaseMigrationError);
    assert_eq!(parse_mystiko_error(&e5), StatusCode::InvalidProviderUrlError);
}

#[test]
fn test_parse_account_error() {
    let e1: AccountHandlerError = AccountHandlerError::StorageError(StorageError::MissingDataError("_".to_string()));
    let e2: AccountHandlerError = AccountHandlerError::CryptoError(CryptoError::InternalError);
    let e3: AccountHandlerError = AccountHandlerError::MnemonicError(bip32::Error::Decode);
    let e4: AccountHandlerError = AccountHandlerError::HexStringError(rustc_hex::FromHexError::InvalidHexLength);
    let e5: AccountHandlerError = AccountHandlerError::WalletHandlerError(WalletHandlerError::StorageError(
        StorageError::MissingDataError("_".to_string()),
    ));
    let e6: AccountHandlerError = AccountHandlerError::NoSuchAccountError("_".to_string(), "_".to_string());
    assert_eq!(parse_account_error(&e1), StatusCode::StorageError);
    assert_eq!(parse_account_error(&e2), StatusCode::CryptoError);
    assert_eq!(parse_account_error(&e3), StatusCode::MnemonicError);
    assert_eq!(parse_account_error(&e4), StatusCode::HexStringError);
    assert_eq!(parse_account_error(&e5), StatusCode::StorageError);
    assert_eq!(parse_account_error(&e6), StatusCode::NoSuchAccountError);
}

#[test]
fn test_parse_wallet_error() {
    let e1: WalletHandlerError = WalletHandlerError::StorageError(StorageError::MissingDataError("_".to_string()));
    let e2: WalletHandlerError = WalletHandlerError::CryptoError(CryptoError::InternalError);
    let e3: WalletHandlerError = WalletHandlerError::MnemonicError(bip32::Error::Decode);
    let e4: WalletHandlerError = WalletHandlerError::HexStringError(rustc_hex::FromHexError::InvalidHexLength);
    let e5: WalletHandlerError = WalletHandlerError::InvalidPasswordError("error".to_string());
    let e6: WalletHandlerError = WalletHandlerError::MismatchedPasswordError;
    let e7: WalletHandlerError = WalletHandlerError::NoExistingWalletError;
    assert_eq!(parse_wallet_error(&e1), StatusCode::StorageError);
    assert_eq!(parse_wallet_error(&e2), StatusCode::CryptoError);
    assert_eq!(parse_wallet_error(&e3), StatusCode::MnemonicError);
    assert_eq!(parse_wallet_error(&e4), StatusCode::HexStringError);
    assert_eq!(parse_wallet_error(&e5), StatusCode::InvalidPasswordError);
    assert_eq!(parse_wallet_error(&e6), StatusCode::MismatchedPasswordError);
    assert_eq!(parse_wallet_error(&e7), StatusCode::NoExistingWalletError);
}
