use crate::{extract_data, setup};
use mystiko_crypto::crypto::decrypt_symmetric;
use mystiko_lib::account::{
    count, count_all, create, export_secret_key, find, find_by_identifier, update, update_encryption,
};
use mystiko_lib::wallet;
use mystiko_lib::wallet::update_password;
use mystiko_protocol::address::ShieldedAddress;
use mystiko_protocol::key::full_public_key;
use mystiko_protocol::types::{FullPk, FullSk};
use mystiko_protos::api::handler::v1::find_account_request::Condition;
use mystiko_protos::api::handler::v1::{
    export_secret_key_request, find_account_by_identifier_request, update_account_request, CountAccountRequest,
    CountAccountResponse, CreateAccountRequest, CreateAccountResponse, CreateWalletRequest, ExportSecretKeyRequest,
    ExportSecretKeyResponse, FindAccountByIdentifierRequest, FindAccountRequest, FindAccountResponse,
    UpdateAccountRequest, UpdateAccountResponse, UpdateEncryptionRequest, UpdatePasswordRequest,
};
use mystiko_protos::core::handler::v1::{CreateAccountOptions, CreateWalletOptions, UpdateAccountOptions};
use mystiko_protos::storage::v1::{ConditionOperator, QueryFilter};
use mystiko_utils::hex::{decode_hex_with_length, encode_hex};
use serial_test::serial;

const DEFAULT_WALLET_PASSWORD: &str = "P@ssw0rd";

fn account_setup() {
    setup(None);
    // create default wallet
    let options = CreateWalletOptions::builder()
        .password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let response = wallet::create(CreateWalletRequest::builder().options(options).build());
    assert!(response.code.unwrap().success);
}

#[test]
#[serial]
fn test_create_default() {
    account_setup();
    let response = create(
        CreateAccountRequest::builder()
            .options(
                CreateAccountOptions::builder()
                    .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
                    .build(),
            )
            .build(),
    );
    assert!(response.code.unwrap().success);
    let account = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    assert_eq!(account.name, "Account 1");
    let full_pk: FullPk = decode_hex_with_length(&account.public_key).unwrap();
    assert_eq!(
        account.shielded_address,
        ShieldedAddress::from_full_public_key(&full_pk).address()
    );
    let full_sk_str = decrypt_symmetric(DEFAULT_WALLET_PASSWORD, &account.encrypted_secret_key).unwrap();
    let full_sk: FullSk = decode_hex_with_length(full_sk_str).unwrap();
    assert_eq!(full_pk, full_public_key(&full_sk).unwrap());
}

#[test]
#[serial]
fn test_count() {
    account_setup();
    let response = create(
        CreateAccountRequest::builder()
            .options(
                CreateAccountOptions::builder()
                    .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
                    .build(),
            )
            .build(),
    );
    assert!(response.code.unwrap().success);
    let response = count(
        CountAccountRequest::builder()
            .filter(
                QueryFilter::builder()
                    .conditions_operator(ConditionOperator::And)
                    .build(),
            )
            .build(),
    );
    assert!(response.code.unwrap().success);
    let count = CountAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .count;
    assert_eq!(count, 1);
}

#[test]
#[serial]
fn test_count_all() {
    account_setup();
    let response = create(
        CreateAccountRequest::builder()
            .options(
                CreateAccountOptions::builder()
                    .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
                    .build(),
            )
            .build(),
    );
    assert!(response.code.unwrap().success);
    let response = count_all();
    assert!(response.code.unwrap().success);
    let count = CountAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .count;
    assert_eq!(count, 1);
}

#[test]
#[serial]
fn test_find() {
    account_setup();
    let response = find(
        FindAccountRequest::builder()
            .condition(Condition::Filter(
                QueryFilter::builder()
                    .conditions_operator(ConditionOperator::And)
                    .build(),
            ))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let accounts = FindAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account;
    assert_eq!(accounts, vec![]);
    let response = create(
        CreateAccountRequest::builder()
            .options(
                CreateAccountOptions::builder()
                    .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
                    .build(),
            )
            .build(),
    );
    assert!(response.code.unwrap().success);
    let account = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    let response = find(
        FindAccountRequest::builder()
            .condition(Condition::Filter(
                QueryFilter::builder()
                    .conditions_operator(ConditionOperator::And)
                    .build(),
            ))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let accounts = FindAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account;
    assert_eq!(accounts, vec![account]);
    let response = find(
        FindAccountRequest::builder()
            .condition(Condition::FindAll(true))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let accounts = FindAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account;
    assert_eq!(accounts.len(), 1);
}

#[test]
#[serial]
fn test_find_by_id() {
    account_setup();
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let response = create(CreateAccountRequest::builder().options(options.clone()).build());
    assert!(response.code.unwrap().success);
    let account1 = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    let response = create(CreateAccountRequest::builder().options(options.clone()).build());
    assert!(response.code.unwrap().success);
    let account2 = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    let response = find_by_identifier(
        FindAccountByIdentifierRequest::builder()
            .identifier(find_account_by_identifier_request::Identifier::Id(account1.id.clone()))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let account3 = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    assert_eq!(account1, account3);
    let response = find_by_identifier(
        FindAccountByIdentifierRequest::builder()
            .identifier(find_account_by_identifier_request::Identifier::Id(account2.id.clone()))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let account4 = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    assert_eq!(account2, account4);
}

#[test]
#[serial]
fn test_find_by_public_key() {
    account_setup();
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let response = create(CreateAccountRequest::builder().options(options.clone()).build());
    assert!(response.code.unwrap().success);
    let account1 = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    let response = create(CreateAccountRequest::builder().options(options.clone()).build());
    assert!(response.code.unwrap().success);
    let account2 = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    let response = find_by_identifier(
        FindAccountByIdentifierRequest::builder()
            .identifier(find_account_by_identifier_request::Identifier::PublicKey(
                account1.public_key.clone(),
            ))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let account3 = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    assert_eq!(account1, account3);
    let response = find_by_identifier(
        FindAccountByIdentifierRequest::builder()
            .identifier(find_account_by_identifier_request::Identifier::PublicKey(
                account2.public_key.clone(),
            ))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let account4 = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    assert_eq!(account2, account4);
}

#[test]
#[serial]
fn test_find_by_shielded_address() {
    account_setup();
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let response = create(CreateAccountRequest::builder().options(options.clone()).build());
    assert!(response.code.unwrap().success);
    let account1 = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    let response = create(CreateAccountRequest::builder().options(options.clone()).build());
    assert!(response.code.unwrap().success);
    let account2 = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    let response = find_by_identifier(
        FindAccountByIdentifierRequest::builder()
            .identifier(find_account_by_identifier_request::Identifier::ShieldedAddress(
                account1.shielded_address.clone(),
            ))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let account3 = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    assert_eq!(account1, account3);
    let response = find_by_identifier(
        FindAccountByIdentifierRequest::builder()
            .identifier(find_account_by_identifier_request::Identifier::ShieldedAddress(
                account2.shielded_address.clone(),
            ))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let account4 = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    assert_eq!(account2, account4);
}

#[test]
#[serial]
fn test_update() {
    account_setup();
    let response = create(
        CreateAccountRequest::builder()
            .options(
                CreateAccountOptions::builder()
                    .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
                    .build(),
            )
            .build(),
    );
    assert!(response.code.unwrap().success);
    let account = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    let mut update_options = UpdateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .name(String::new())
        .build();
    let response = update(
        UpdateAccountRequest::builder()
            .options(update_options.clone())
            .identifier(update_account_request::Identifier::PublicKey(account.public_key))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let mut updated_account = UpdateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    assert_eq!(updated_account.name, account.name);
    assert_eq!(updated_account.updated_at, account.updated_at);
    update_options.name = Some(String::from("Awesome Account Name"));
    let response = update(
        UpdateAccountRequest::builder()
            .options(update_options)
            .identifier(update_account_request::Identifier::ShieldedAddress(
                account.shielded_address,
            ))
            .build(),
    );
    assert!(response.code.unwrap().success);
    updated_account = UpdateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    assert_eq!(updated_account.name, "Awesome Account Name");
}

#[test]
#[serial]
fn test_update_encryption() {
    account_setup();
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let response1 = create(CreateAccountRequest::builder().options(options.clone()).build());
    assert!(response1.code.unwrap().success);
    let account1 = CreateAccountResponse::try_from(extract_data(response1.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    let response2 = create(CreateAccountRequest::builder().options(options.clone()).build());
    assert!(response2.code.unwrap().success);
    let account2 = CreateAccountResponse::try_from(extract_data(response2.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    let sk1_response = export_secret_key(
        ExportSecretKeyRequest::builder()
            .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
            .identifier(export_secret_key_request::Identifier::Id(account1.id.clone()))
            .build(),
    );
    assert!(sk1_response.code.unwrap().success);
    let sk1 = ExportSecretKeyResponse::try_from(extract_data(sk1_response.result.unwrap()))
        .unwrap()
        .secret_key;
    let sk2_response = export_secret_key(
        ExportSecretKeyRequest::builder()
            .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
            .identifier(export_secret_key_request::Identifier::Id(account2.id.clone()))
            .build(),
    );
    assert!(sk2_response.code.unwrap().success);
    let sk2 = ExportSecretKeyResponse::try_from(extract_data(sk2_response.result.unwrap()))
        .unwrap()
        .secret_key;
    let new_wallet_password: &str = "newP@ssw0rd";
    let response = update_encryption(
        UpdateEncryptionRequest::builder()
            .old_wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
            .new_wallet_password(new_wallet_password.to_string())
            .build(),
    );
    assert!(response.code.unwrap().success);
    let response = update_password(
        UpdatePasswordRequest::builder()
            .old_password(DEFAULT_WALLET_PASSWORD)
            .new_password(new_wallet_password)
            .build(),
    );
    assert!(response.code.unwrap().success);
    let sk3_response = export_secret_key(
        ExportSecretKeyRequest::builder()
            .wallet_password(new_wallet_password)
            .identifier(export_secret_key_request::Identifier::Id(account1.id))
            .build(),
    );
    assert!(sk3_response.code.unwrap().success);
    let sk3 = ExportSecretKeyResponse::try_from(extract_data(sk3_response.result.unwrap()))
        .unwrap()
        .secret_key;
    let sk4_response = export_secret_key(
        ExportSecretKeyRequest::builder()
            .wallet_password(new_wallet_password)
            .identifier(export_secret_key_request::Identifier::Id(account2.id))
            .build(),
    );
    assert!(sk4_response.code.unwrap().success);
    let sk4 = ExportSecretKeyResponse::try_from(extract_data(sk4_response.result.unwrap()))
        .unwrap()
        .secret_key;
    assert_eq!(sk1, sk3);
    assert_eq!(sk2, sk4);
}

#[test]
#[serial]
fn test_export_secret_key_by_id() {
    account_setup();
    let response = create(
        CreateAccountRequest::builder()
            .options(
                CreateAccountOptions::builder()
                    .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
                    .build(),
            )
            .build(),
    );
    assert!(response.code.unwrap().success);
    let account = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    // export secret key by id
    let response = export_secret_key(
        ExportSecretKeyRequest::builder()
            .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
            .identifier(export_secret_key_request::Identifier::Id(account.id))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let secret_key_str = ExportSecretKeyResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .secret_key;
    let full_sk: FullSk = decode_hex_with_length(secret_key_str).unwrap();
    let full_pk = full_public_key(&full_sk).unwrap();
    assert_eq!(account.public_key, encode_hex(full_pk));
    assert_eq!(
        account.shielded_address,
        ShieldedAddress::from_full_public_key(&full_pk).address()
    );
}

#[test]
#[serial]
fn test_export_secret_key_by_public_key() {
    account_setup();
    let response = create(
        CreateAccountRequest::builder()
            .options(
                CreateAccountOptions::builder()
                    .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
                    .build(),
            )
            .build(),
    );
    assert!(response.code.unwrap().success);
    let account = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    // export secret key by id
    let response = export_secret_key(
        ExportSecretKeyRequest::builder()
            .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
            .identifier(export_secret_key_request::Identifier::PublicKey(
                account.public_key.clone(),
            ))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let secret_key_str = ExportSecretKeyResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .secret_key;
    let full_sk: FullSk = decode_hex_with_length(secret_key_str).unwrap();
    let full_pk = full_public_key(&full_sk).unwrap();
    assert_eq!(account.public_key, encode_hex(full_pk));
    assert_eq!(
        account.shielded_address,
        ShieldedAddress::from_full_public_key(&full_pk).address()
    );
}

#[test]
#[serial]
fn test_export_secret_key_by_shielded_address() {
    account_setup();
    let response = create(
        CreateAccountRequest::builder()
            .options(
                CreateAccountOptions::builder()
                    .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
                    .build(),
            )
            .build(),
    );
    assert!(response.code.unwrap().success);
    let account = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    // export secret key by id
    let response = export_secret_key(
        ExportSecretKeyRequest::builder()
            .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
            .identifier(export_secret_key_request::Identifier::ShieldedAddress(
                account.shielded_address.clone(),
            ))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let secret_key_str = ExportSecretKeyResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .secret_key;
    let full_sk: FullSk = decode_hex_with_length(secret_key_str).unwrap();
    let full_pk = full_public_key(&full_sk).unwrap();
    assert_eq!(account.public_key, encode_hex(full_pk));
    assert_eq!(
        account.shielded_address,
        ShieldedAddress::from_full_public_key(&full_pk).address()
    );
}
