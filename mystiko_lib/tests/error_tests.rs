use anyhow::anyhow;
use ethers_core::abi::ethereum_types::FromDecStrErr;
use ethers_providers::ProviderError;
use mystiko_core::{
    AccountsError, CommitmentPoolContractsError, DepositContractsError, DepositsError, MystikoError, PublicAssetsError,
    ScannerError, SynchronizerError, TransactionsError, WalletsError,
};
use mystiko_crypto::error::CryptoError;
use mystiko_dataloader::loader::DataLoaderConfigError;
use mystiko_dataloader::DataLoaderError;
use mystiko_lib::error::{
    parse_account_error, parse_deposit_error, parse_mystiko_error, parse_scanner_error, parse_synchronizer_error,
    parse_wallet_error,
};
use mystiko_protocol::error::ProtocolError;
use mystiko_protos::api::v1::{
    AccountError as ProtoAccountError, DepositError as ProtoDepositError, MystikoError as ProtoMystikoError,
    ScannerError as ProtoScannerError, SynchronizerError as ProtoSynchronizerError, WalletError as ProtoWalletError,
};
use mystiko_storage::StorageError;
use mystiko_types::BridgeType;

#[test]
fn test_parse_mystiko_error() {
    let e1: MystikoError = MystikoError::SynchronizerError(SynchronizerError::UnsupportedChainError(1));
    let e2: MystikoError = MystikoError::ScannerError(ScannerError::NoSuchAccountError);
    let e3: MystikoError = MystikoError::DataLoaderError(DataLoaderError::LoaderNoContractsError);
    let e4: MystikoError = MystikoError::ConfigError(anyhow!("error"));
    let e5: MystikoError = MystikoError::DatabaseMigrationError(anyhow!("error"));
    let e6: MystikoError = MystikoError::InvalidProviderUrlError("_".to_string());

    assert_eq!(parse_mystiko_error(&e1), ProtoMystikoError::SynchronizerError.into(),);
    assert_eq!(parse_mystiko_error(&e2), ProtoMystikoError::ScannerError.into());
    assert_eq!(parse_mystiko_error(&e3), ProtoMystikoError::DataloaderError.into());
    assert_eq!(parse_mystiko_error(&e4), ProtoMystikoError::ConfigError.into());
    assert_eq!(
        parse_mystiko_error(&e5),
        ProtoMystikoError::DatabaseMigrationError.into()
    );
    assert_eq!(
        parse_mystiko_error(&e6),
        ProtoMystikoError::InvalidProviderUrlError.into()
    );
}

#[test]
fn test_parse_account_error() {
    let e1: AccountsError = AccountsError::StorageError(StorageError::MissingDataError("_".to_string()));
    let e2: AccountsError = AccountsError::CryptoError(CryptoError::InternalError);
    let e3: AccountsError = AccountsError::MnemonicError(bip32::Error::Decode);
    let e4: AccountsError = AccountsError::HexStringError(rustc_hex::FromHexError::InvalidHexLength);
    let e5: AccountsError = AccountsError::WalletsError(WalletsError::StorageError(StorageError::MissingDataError(
        "_".to_string(),
    )));
    let e6: AccountsError = AccountsError::NoSuchAccountError("_".to_string(), "_".to_string());
    assert_eq!(parse_account_error(&e1), ProtoAccountError::StorageError.into());
    assert_eq!(parse_account_error(&e2), ProtoAccountError::CryptoError.into());
    assert_eq!(parse_account_error(&e3), ProtoAccountError::MnemonicError.into());
    assert_eq!(parse_account_error(&e4), ProtoAccountError::HexStringError.into());
    assert_eq!(parse_account_error(&e5), ProtoAccountError::WalletsError.into());
    assert_eq!(parse_account_error(&e6), ProtoAccountError::NoSuchAccountError.into());
}

#[test]
fn test_parse_wallet_error() {
    let e1: WalletsError = WalletsError::StorageError(StorageError::MissingDataError("_".to_string()));
    let e2: WalletsError = WalletsError::CryptoError(CryptoError::InternalError);
    let e3: WalletsError = WalletsError::MnemonicError(bip32::Error::Decode);
    let e4: WalletsError = WalletsError::HexStringError(rustc_hex::FromHexError::InvalidHexLength);
    let e5: WalletsError = WalletsError::InvalidPasswordError("error".to_string());
    let e6: WalletsError = WalletsError::MismatchedPasswordError;
    let e7: WalletsError = WalletsError::NoExistingWalletError;
    assert_eq!(parse_wallet_error(&e1), ProtoWalletError::StorageError.into());
    assert_eq!(parse_wallet_error(&e2), ProtoWalletError::CryptoError.into());
    assert_eq!(parse_wallet_error(&e3), ProtoWalletError::MnemonicError.into());
    assert_eq!(parse_wallet_error(&e4), ProtoWalletError::HexStringError.into());
    assert_eq!(parse_wallet_error(&e5), ProtoWalletError::InvalidPasswordError.into());
    assert_eq!(
        parse_wallet_error(&e6),
        ProtoWalletError::MismatchedPasswordError.into()
    );
    assert_eq!(parse_wallet_error(&e7), ProtoWalletError::NoExistingWalletError.into());
}

#[test]
fn test_parse_deposit_error() {
    let e1: DepositsError = DepositsError::HexStringError(rustc_hex::FromHexError::InvalidHexLength);
    let e2: DepositsError = DepositsError::FromDecStrError(FromDecStrErr::InvalidLength);
    let e3: DepositsError = DepositsError::ProviderError(ProviderError::CustomError("err".to_string()));
    let e4: DepositsError = DepositsError::AnyhowError(anyhow!("error"));
    let e5: DepositsError = DepositsError::PublicAssetsError(PublicAssetsError::BalanceOfTimeoutError(1));
    let e6: DepositsError = DepositsError::DepositContractsError(DepositContractsError::ConfigError(anyhow!("error")));
    let e7: DepositsError =
        DepositsError::CommitmentPoolContractsError(CommitmentPoolContractsError::IsSpentNullifierTimeoutError(1));
    let e8: DepositsError = DepositsError::TransactionsError(TransactionsError::AnyhowError(anyhow!("error")));
    let e9: DepositsError = DepositsError::ProtocolError(ProtocolError::InvalidNoteSizeError);
    let e10: DepositsError = DepositsError::StorageError(StorageError::MissingDataError("_".to_string()));
    let e11: DepositsError = DepositsError::WalletsError(WalletsError::MismatchedPasswordError);
    let e12: DepositsError = DepositsError::AccountsError(AccountsError::CryptoError(CryptoError::InternalError));
    let e13: DepositsError = DepositsError::UnsupportedChainIdError(1);
    let e14: DepositsError = DepositsError::NoDepositContractFoundError(1, "".to_string(), 2, BridgeType::Axelar);
    let e15: DepositsError = DepositsError::InvalidDepositAmountError(1f64, 1f64, 1f64);
    let e16: DepositsError = DepositsError::InvalidRollupFeeAmountError(1f64, 1f64);
    let e17: DepositsError = DepositsError::InvalidBridgeFeeAmountError(1f64, 1f64);
    let e18: DepositsError = DepositsError::InvalidExecutorFeeAmountError(1f64, 1f64);
    let e19: DepositsError = DepositsError::InsufficientBalanceError("".to_string(), 1f64);
    let e20: DepositsError = DepositsError::IdNotFoundError("".to_string());
    let e21: DepositsError = DepositsError::MissingPrivateKeyError;
    let e22: DepositsError = DepositsError::DepositStatusError("".to_string());
    let e23: DepositsError = DepositsError::DuplicateCommitmentError("".to_string(), 1, "".to_string());

    assert_eq!(parse_deposit_error(&e1), ProtoDepositError::HexStringError.into());
    assert_eq!(parse_deposit_error(&e2), ProtoDepositError::FromDecStrError.into());
    assert_eq!(parse_deposit_error(&e3), ProtoDepositError::ProviderError.into());
    assert_eq!(parse_deposit_error(&e4), ProtoDepositError::AnyhowError.into());
    assert_eq!(parse_deposit_error(&e5), ProtoDepositError::PublicAssetsError.into());
    assert_eq!(
        parse_deposit_error(&e6),
        ProtoDepositError::DepositContractsError.into()
    );
    assert_eq!(
        parse_deposit_error(&e7),
        ProtoDepositError::CommitmentPoolContractsError.into()
    );
    assert_eq!(parse_deposit_error(&e8), ProtoDepositError::TransactionsError.into());
    assert_eq!(parse_deposit_error(&e9), ProtoDepositError::ProtocolError.into());
    assert_eq!(parse_deposit_error(&e10), ProtoDepositError::StorageError.into());
    assert_eq!(parse_deposit_error(&e11), ProtoDepositError::WalletsError.into());
    assert_eq!(parse_deposit_error(&e12), ProtoDepositError::AccountsError.into());
    assert_eq!(
        parse_deposit_error(&e13),
        ProtoDepositError::UnsupportedChainIdError.into()
    );
    assert_eq!(
        parse_deposit_error(&e14),
        ProtoDepositError::NoDepositContractFoundError.into()
    );
    assert_eq!(
        parse_deposit_error(&e15),
        ProtoDepositError::InvalidDepositAmountError.into()
    );
    assert_eq!(
        parse_deposit_error(&e16),
        ProtoDepositError::InvalidRollupFeeAmountError.into()
    );
    assert_eq!(
        parse_deposit_error(&e17),
        ProtoDepositError::InvalidBridgeFeeAmountError.into()
    );
    assert_eq!(
        parse_deposit_error(&e18),
        ProtoDepositError::InvalidExecutorFeeAmountError.into()
    );
    assert_eq!(
        parse_deposit_error(&e19),
        ProtoDepositError::InsufficientBalanceError.into()
    );
    assert_eq!(parse_deposit_error(&e20), ProtoDepositError::IdNotFoundError.into());
    assert_eq!(
        parse_deposit_error(&e21),
        ProtoDepositError::MissingPrivateKeyError.into()
    );
    assert_eq!(parse_deposit_error(&e22), ProtoDepositError::DepositStatusError.into());
    assert_eq!(
        parse_deposit_error(&e23),
        ProtoDepositError::DuplicateCommitmentError.into()
    );
}

#[test]
fn test_parse_synchronizer_error() {
    let e1: SynchronizerError = SynchronizerError::UnsupportedChainError(1);
    let e2: SynchronizerError = SynchronizerError::DataLoaderError(DataLoaderError::LoaderNoContractsError);
    let e3: SynchronizerError =
        SynchronizerError::DataLoaderConfigError(DataLoaderConfigError::AnyhowError(anyhow!("error")));
    let e4: SynchronizerError = SynchronizerError::AnyhowError(anyhow!("error"));
    assert_eq!(
        parse_synchronizer_error(&e1),
        ProtoSynchronizerError::UnsupportedChainError.into()
    );
    assert_eq!(
        parse_synchronizer_error(&e2),
        ProtoSynchronizerError::DataloaderError.into()
    );
    assert_eq!(
        parse_synchronizer_error(&e3),
        ProtoSynchronizerError::DataloaderConfigError.into()
    );
    assert_eq!(
        parse_synchronizer_error(&e4),
        ProtoSynchronizerError::AnyhowError.into()
    );
}

#[test]
fn test_parse_scanner_error() {
    let e1: ScannerError = ScannerError::NoSuchAccountError;
    let e2: ScannerError = ScannerError::NoSuchContractConfigError(1, "".to_string());
    let e3: ScannerError = ScannerError::CommitmentEmptyError;
    let e4: ScannerError = ScannerError::AccountHandlerError(AccountsError::CryptoError(CryptoError::InternalError));
    let e5: ScannerError = ScannerError::WalletHandlerError(WalletsError::MismatchedPasswordError);
    let e6: ScannerError = ScannerError::CryptoError(CryptoError::InternalError);
    let e7: ScannerError = ScannerError::StorageError(StorageError::MissingDataError("_".to_string()));
    let e8: ScannerError = ScannerError::ProtocolError(ProtocolError::InvalidNoteSizeError);
    let e9: ScannerError = ScannerError::FromHexError(rustc_hex::FromHexError::InvalidHexLength);
    let e10: ScannerError = ScannerError::AnyhowError(anyhow!("error"));
    assert_eq!(parse_scanner_error(&e1), ProtoScannerError::NoSuchAccountError.into());
    assert_eq!(
        parse_scanner_error(&e2),
        ProtoScannerError::NoSuchContractConfigError.into()
    );
    assert_eq!(parse_scanner_error(&e3), ProtoScannerError::CommitmentEmptyError.into());
    assert_eq!(parse_scanner_error(&e4), ProtoScannerError::AccountHandlerError.into());
    assert_eq!(parse_scanner_error(&e5), ProtoScannerError::WalletHandlerError.into());
    assert_eq!(parse_scanner_error(&e6), ProtoScannerError::CryptoError.into());
    assert_eq!(parse_scanner_error(&e7), ProtoScannerError::StorageError.into());
    assert_eq!(parse_scanner_error(&e8), ProtoScannerError::ProtocolError.into());
    assert_eq!(parse_scanner_error(&e9), ProtoScannerError::FromHexError.into());
    assert_eq!(parse_scanner_error(&e10), ProtoScannerError::AnyhowError.into());
}
