mod scanner_assets_tests;
mod scanner_balance_tests;
mod scanner_reset_tests;
mod scanner_scan_bridge_tests;
mod scanner_scan_tests;

use crate::common::create_database;
use mystiko_config::MystikoConfig;
use mystiko_core::{
    AccountCollection, AccountHandler, Accounts, Commitment, CommitmentCollection, Database, NullifierCollection,
    Scanner, ScannerOptions, WalletCollection, WalletHandler, Wallets,
};
use mystiko_protocol::address::ShieldedAddress;
use mystiko_protocol::commitment::Commitment as ProtocolCommitment;
use mystiko_protocol::commitment::Note;
use mystiko_protocol::key::{separate_secret_keys, verification_secret_key};
use mystiko_protocol::types::{FullSk, VerifySk};
use mystiko_protocol::utils::compute_nullifier;
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::handler::v1::{CreateAccountOptions, CreateWalletOptions};
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_storage::{Document, SqlStatementFormatter};
use mystiko_storage_sqlite::SqliteStorage;
use mystiko_utils::hex::{decode_hex_with_length, encode_hex};
use num_bigint::BigUint;
use std::str::FromStr;
use std::string::ToString;
use std::sync::Arc;
use typed_builder::TypedBuilder;

pub(crate) const DEFAULT_WALLET_PASSWORD: &str = "P@ssw0rd";

#[derive(Debug, TypedBuilder)]
pub struct TestAccount {
    shielded_address: ShieldedAddress,
    v_sk: VerifySk,
}

pub async fn create_scanner(
    account_count: usize,
) -> (
    Scanner<SqlStatementFormatter, SqliteStorage>,
    Arc<Database<SqlStatementFormatter, SqliteStorage>>,
    Vec<TestAccount>,
) {
    let _ = env_logger::builder()
        .filter_module("mystiko_core", log::LevelFilter::Info)
        .is_test(true)
        .try_init();
    let config = Arc::new(
        MystikoConfig::from_json_file("tests/files/mystiko/config.json")
            .await
            .unwrap(),
    );

    let db = Arc::new(create_database().await);
    let wallet = WalletCollection::new(db.collection());
    wallet.migrate().await.unwrap();
    let wallet = Wallets::new(db.clone());
    let options = CreateWalletOptions::builder()
        .password(String::from(DEFAULT_WALLET_PASSWORD))
        .build();
    let _ = wallet.create(&options).await.unwrap();

    let account_db = AccountCollection::new(db.collection());
    account_db.migrate().await.unwrap();
    let account_handler = Accounts::new(db.clone());
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let mut test_accounts = vec![];
    for _ in 0..account_count {
        let account = account_handler.create(&options).await.unwrap();
        let shielded_address = ShieldedAddress::from_string(&account.shielded_address).unwrap();
        let secret_key = account_handler
            .export_secret_key_by_id(DEFAULT_WALLET_PASSWORD, &account.id)
            .await
            .unwrap();
        let secret_key_bytes: FullSk = decode_hex_with_length(secret_key).unwrap();
        let (v_sk, _) = separate_secret_keys(&secret_key_bytes).unwrap();
        test_accounts.push(
            TestAccount::builder()
                .shielded_address(shielded_address)
                .v_sk(verification_secret_key(&v_sk).unwrap())
                .build(),
        );
    }
    assert_eq!(account_handler.find_all().await.unwrap().len(), account_count);

    let commitment = CommitmentCollection::new(db.collection());
    commitment.migrate().await.unwrap();
    let nullifier = NullifierCollection::new(db.collection());
    nullifier.migrate().await.unwrap();
    let options = ScannerOptions::builder().db(db.clone()).build();
    (Scanner::new(config, options), db, test_accounts)
}

pub async fn insert_commitments(
    db: Arc<Database<SqlStatementFormatter, SqliteStorage>>,
    count: usize,
    test_account: Option<&TestAccount>,
) -> (Vec<Document<Commitment>>, Vec<BigUint>) {
    let mut insert_cms = vec![];
    let mut nullifiers = vec![];
    for index in 0..count {
        let amount = BigUint::from((index + 1) * 1000);
        let mut cm = default_commitment();
        match &test_account {
            Some(account) => {
                let note = Note::new(Some(amount.clone()), None).unwrap();
                let pcm = ProtocolCommitment::new(account.shielded_address.clone(), Some(note.clone()), None).unwrap();
                cm.commitment_hash = pcm.commitment_hash.clone();
                cm.encrypted_note = Some(encode_hex(pcm.encrypted_note));

                let nullifier = compute_nullifier(&account.v_sk, &pcm.note.random_p);
                nullifiers.push(nullifier);
            }
            None => {
                cm.commitment_hash += BigUint::from(index as u64);
            }
        }
        cm.amount = Some(amount);
        cm.leaf_index = Some(index as u64);
        insert_cms.push(cm)
    }
    let db_cms = db.commitments.insert_batch(&insert_cms).await.unwrap();
    (db_cms, nullifiers)
}

fn default_commitment() -> Commitment {
    Commitment {
        chain_id: 5,
        contract_address: "contract_address 1".to_string(),
        bridge_type: BridgeType::Loop as i32,
        commitment_hash: BigUint::from_str(
            "9709515941671889428395361755215352896616366060066411186055604144562505250548",
        )
        .unwrap(),
        asset_symbol: String::from("MTT"),
        asset_decimals: 18,
        asset_address: Some("asset_address 1".to_string()),
        status: CommitmentStatus::Queued as i32,
        spent: false,
        block_number: 10000000u64,
        src_chain_block_number: Some(10000000u64),
        included_block_number: Some(10000100u64),
        rollup_fee_amount: Some(BigUint::from(20000000000000000u64)),
        encrypted_note: Some(String::from("")),
        leaf_index: Some(0u64),
        amount: Some(BigUint::from(1000000000000000000u64)),
        nullifier: None,
        shielded_address: None,
        queued_transaction_hash: Some(String::from("")),
        included_transaction_hash: Some(String::from("")),
        src_chain_transaction_hash: Some(String::from("")),
    }
}
