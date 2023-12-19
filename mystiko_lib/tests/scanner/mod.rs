use crate::{extract_data, setup};
use mystiko_lib::{account, scanner, wallet};
use mystiko_protos::api::handler::v1::{
    CreateAccountRequest, CreateAccountResponse, CreateWalletRequest, CreateWalletResponse,
};
use mystiko_protos::api::scanner::v1::{
    AssetsRequest, AssetsResponse, BalanceRequest, BalanceResponse, ChainAssetsRequest, ChainAssetsResponse,
    ResetRequest, ResetResponse, ScanRequest, ScanResponse,
};
use mystiko_protos::api::v1::status_code::Error;
use mystiko_protos::api::v1::ScannerError;
use mystiko_protos::core::document::v1::{Account, Wallet};
use mystiko_protos::core::handler::v1::{CreateAccountOptions, CreateWalletOptions};
use mystiko_protos::core::scanner::v1::{AssetsOptions, BalanceOptions, ResetOptions, ScanOptions};
use serial_test::serial;

const DEFAULT_WALLET_PASSWORD: &str = "P@ssw0rd";

fn scanner_setup() -> (Wallet, Account) {
    setup(None);
    // create wallet
    let response = wallet::create(
        CreateWalletRequest::builder()
            .options(CreateWalletOptions::builder().password(DEFAULT_WALLET_PASSWORD).build())
            .build(),
    );
    assert!(response.code.unwrap().success);
    let wallet = CreateWalletResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .wallet
        .unwrap();

    // create account
    let response = account::create(
        CreateAccountRequest::builder()
            .options(
                CreateAccountOptions::builder()
                    .wallet_password(DEFAULT_WALLET_PASSWORD)
                    .build(),
            )
            .build(),
    );
    assert!(response.code.unwrap().success);
    let account = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    (wallet, account)
}

#[test]
#[serial]
fn test_scan() {
    scanner_setup();

    let response = scanner::scan(ScanRequest::builder().options(ScanOptions::builder().build()).build());
    assert_eq!(
        response.code.unwrap().error.unwrap(),
        Error::Scanner(ScannerError::WalletHandlerError as i32)
    );

    let response = scanner::scan(
        ScanRequest::builder()
            .options(ScanOptions::builder().wallet_password("wrong_password").build())
            .build(),
    );
    assert_eq!(
        response.code.unwrap().error.unwrap(),
        Error::Scanner(ScannerError::WalletHandlerError as i32)
    );

    let response = scanner::scan(
        ScanRequest::builder()
            .options(ScanOptions::builder().wallet_password(DEFAULT_WALLET_PASSWORD).build())
            .build(),
    );
    assert!(response.code.unwrap().success);
    let scan_result = ScanResponse::try_from(extract_data(response.result.unwrap()));
    assert!(scan_result.is_ok());
}

#[test]
#[serial]
fn test_reset() {
    scanner_setup();
    let response = scanner::reset(ResetRequest::builder().options(ResetOptions::builder().build()).build());
    assert!(response.code.unwrap().success);
    let result = ResetResponse::try_from(extract_data(response.result.unwrap()));
    assert!(result.is_ok());
}

#[test]
#[serial]
fn test_balance() {
    scanner_setup();

    let response = scanner::balance(
        BalanceRequest::builder()
            .options(BalanceOptions::builder().build())
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = BalanceResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .result
        .unwrap();
    assert!(result.balances.is_empty());
}

#[test]
#[serial]
fn test_assets() {
    let (_, account) = scanner_setup();
    let response = scanner::assets(
        AssetsRequest::builder()
            .options(
                AssetsOptions::builder()
                    .shielded_addresses(vec![account.shielded_address])
                    .build(),
            )
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = AssetsResponse::try_from(extract_data(response.result.unwrap()));
    assert!(result.is_ok());
    assert!(result.unwrap().results.is_empty());
}

#[test]
#[serial]
fn test_chain_assets() {
    let (_, account) = scanner_setup();

    let response = scanner::chain_assets(
        ChainAssetsRequest::builder()
            .options(
                AssetsOptions::builder()
                    .shielded_addresses(vec![account.shielded_address])
                    .build(),
            )
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = ChainAssetsResponse::try_from(extract_data(response.result.unwrap()));
    assert!(result.is_ok());
    assert!(result.unwrap().result.is_none());
}
