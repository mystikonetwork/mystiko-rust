use anyhow::anyhow;
use mystiko_core::{AccountHandlerV1Error, MystikoError, SynchronizerError, WalletHandlerV1Error};
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
    let e1: AccountHandlerV1Error =
        AccountHandlerV1Error::StorageError(StorageError::MissingDataError("_".to_string()));
    let e2: AccountHandlerV1Error = AccountHandlerV1Error::CryptoError(CryptoError::InternalError);
    let e3: AccountHandlerV1Error = AccountHandlerV1Error::MnemonicError(bip32::Error::Decode);
    let e4: AccountHandlerV1Error = AccountHandlerV1Error::HexStringError(rustc_hex::FromHexError::InvalidHexLength);
    let e5: AccountHandlerV1Error = AccountHandlerV1Error::WalletHandlerError(WalletHandlerV1Error::StorageError(
        StorageError::MissingDataError("_".to_string()),
    ));
    let e6: AccountHandlerV1Error = AccountHandlerV1Error::NoSuchAccountError("_".to_string(), "_".to_string());
    assert_eq!(parse_account_error(&e1), StatusCode::StorageError);
    assert_eq!(parse_account_error(&e2), StatusCode::CryptoError);
    assert_eq!(parse_account_error(&e3), StatusCode::MnemonicError);
    assert_eq!(parse_account_error(&e4), StatusCode::HexStringError);
    assert_eq!(parse_account_error(&e5), StatusCode::StorageError);
    assert_eq!(parse_account_error(&e6), StatusCode::NoSuchAccountError);
}

#[test]
fn test_parse_wallet_error() {
    let e1: WalletHandlerV1Error = WalletHandlerV1Error::StorageError(StorageError::MissingDataError("_".to_string()));
    let e2: WalletHandlerV1Error = WalletHandlerV1Error::CryptoError(CryptoError::InternalError);
    let e3: WalletHandlerV1Error = WalletHandlerV1Error::MnemonicError(bip32::Error::Decode);
    let e4: WalletHandlerV1Error = WalletHandlerV1Error::HexStringError(rustc_hex::FromHexError::InvalidHexLength);
    let e5: WalletHandlerV1Error = WalletHandlerV1Error::InvalidPasswordError("error".to_string());
    let e6: WalletHandlerV1Error = WalletHandlerV1Error::MismatchedPasswordError;
    let e7: WalletHandlerV1Error = WalletHandlerV1Error::NoExistingWalletError;
    assert_eq!(parse_wallet_error(&e1), StatusCode::StorageError);
    assert_eq!(parse_wallet_error(&e2), StatusCode::CryptoError);
    assert_eq!(parse_wallet_error(&e3), StatusCode::MnemonicError);
    assert_eq!(parse_wallet_error(&e4), StatusCode::HexStringError);
    assert_eq!(parse_wallet_error(&e5), StatusCode::InvalidPasswordError);
    assert_eq!(parse_wallet_error(&e6), StatusCode::MismatchedPasswordError);
    assert_eq!(parse_wallet_error(&e7), StatusCode::NoExistingWalletError);
}
