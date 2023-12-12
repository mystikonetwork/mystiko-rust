use crate::{extract_data, setup};
use bip32::secp256k1::elliptic_curve::rand_core::OsRng;
use bip32::{Language, Mnemonic};
use mystiko_lib::wallet::{check_current, check_password, create, export_mnemonic_phrase, update_password};
use mystiko_protos::api::handler::v1::{
    CheckCurrentResponse, CheckPasswordRequest, CreateWalletRequest, CreateWalletResponse, ExportMnemonicPhraseRequest,
    ExportMnemonicPhraseResponse, UpdatePasswordRequest,
};
use mystiko_protos::api::v1::WalletError;
use mystiko_protos::core::handler::v1::CreateWalletOptions;
use serial_test::serial;

#[test]
#[serial]
fn test_create() {
    setup(None);
    let options = CreateWalletOptions::builder().password("P@ssw0rd".to_string()).build();
    let response = create(CreateWalletRequest::builder().options(options).build());
    assert!(response.code.unwrap().success);
    let wallet = CreateWalletResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .wallet
        .unwrap();
    let current = CheckCurrentResponse::try_from(extract_data(check_current().result.unwrap()))
        .unwrap()
        .wallet
        .unwrap();
    assert_eq!(wallet, current);
}

#[test]
#[serial]
fn test_create_with_mnemonic() {
    setup(None);
    let mnemonic = Mnemonic::random(OsRng, Language::English);
    let options = CreateWalletOptions::builder()
        .password("P@ssw0rd".to_string())
        .mnemonic_phrase(mnemonic.phrase().to_string())
        .build();
    let response = create(CreateWalletRequest::builder().options(options).build());
    assert!(response.code.unwrap().success);
}

#[test]
#[serial]
fn test_create_with_invalid_password() {
    setup(None);
    let response = create(
        CreateWalletRequest::builder()
            .options(CreateWalletOptions::builder().password("AAAAAAAA".to_string()).build())
            .build(),
    );
    // assert_eq!(response.code.unwrap().error.unwrap(), Error::Wallet(WalletError::InvalidPasswordError as i32));
    assert_eq!(response.code.unwrap(), WalletError::InvalidPasswordError.into());
}

#[test]
#[serial]
fn test_check_password() {
    setup(None);
    let response = check_password(CheckPasswordRequest::builder().password("P@ssw0rd".to_string()).build());
    assert_eq!(response.code.unwrap(), WalletError::NoExistingWalletError.into());
    let response = create(
        CreateWalletRequest::builder()
            .options(CreateWalletOptions::builder().password("P@ssw0rd".to_string()).build())
            .build(),
    );
    assert!(response.code.unwrap().success);
    let response = check_password(CheckPasswordRequest::builder().password("P@ssw0rd".to_string()).build());
    assert!(response.code.unwrap().success);
    let response = check_password(
        CheckPasswordRequest::builder()
            .password("wrong_password".to_string())
            .build(),
    );
    assert_eq!(response.code.unwrap(), WalletError::MismatchedPasswordError.into());
}

#[test]
#[serial]
fn test_update_password() {
    setup(None);
    let response = update_password(
        UpdatePasswordRequest::builder()
            .old_password("P@ssw0rd".to_string())
            .new_password("P@ssw0rd2".to_string())
            .build(),
    );
    assert_eq!(response.code.unwrap(), WalletError::NoExistingWalletError.into());
    let response = create(
        CreateWalletRequest::builder()
            .options(CreateWalletOptions::builder().password("P@ssw0rd".to_string()).build())
            .build(),
    );
    assert!(response.code.unwrap().success);
    let response = export_mnemonic_phrase(
        ExportMnemonicPhraseRequest::builder()
            .password("P@ssw0rd".to_string())
            .build(),
    );
    assert!(response.code.unwrap().success);
    let mnemonic_words = ExportMnemonicPhraseResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .mnemonic_phrase;
    let response = update_password(
        UpdatePasswordRequest::builder()
            .old_password("wrong_password".to_string())
            .new_password("P@ssw0rd2".to_string())
            .build(),
    );
    assert_eq!(response.code.unwrap(), WalletError::MismatchedPasswordError.into());
    let response = update_password(
        UpdatePasswordRequest::builder()
            .old_password("P@ssw0rd".to_string())
            .new_password("invalid_password".to_string())
            .build(),
    );
    assert_eq!(response.code.unwrap(), WalletError::InvalidPasswordError.into());
    let response = update_password(
        UpdatePasswordRequest::builder()
            .old_password("P@ssw0rd".to_string())
            .new_password("newP@ssw0rd".to_string())
            .build(),
    );
    assert!(response.code.unwrap().success);
    let response = check_password(
        CheckPasswordRequest::builder()
            .password("newP@ssw0rd".to_string())
            .build(),
    );
    assert!(response.code.unwrap().success);
    let response = export_mnemonic_phrase(
        ExportMnemonicPhraseRequest::builder()
            .password("newP@ssw0rd".to_string())
            .build(),
    );
    assert!(response.code.unwrap().success);
    let new_mnemonic_phrase = ExportMnemonicPhraseResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .mnemonic_phrase;
    assert_eq!(mnemonic_words, new_mnemonic_phrase);
}
