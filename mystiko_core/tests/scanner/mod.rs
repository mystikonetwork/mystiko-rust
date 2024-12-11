mod scanner_assets_tests;
mod scanner_balance_tests;
mod scanner_reset_tests;
mod scanner_scan_bridge_tests;
mod scanner_scan_import_cross_chain_tests;
mod scanner_scan_import_tests;
mod scanner_scan_tests;

use crate::common::{create_database, MockProvider, MockProviders};
use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, Bytes, Log, TransactionReceipt, TxHash, H256, U256, U64};
use mockall::mock;
use mystiko_config::MystikoConfig;
use mystiko_core::{
    AccountCollection, AccountHandler, Accounts, AuditorPublicKeysOptions, Commitment, CommitmentCollection,
    CommitmentPoolContractHandler, Database, IncludedCountOptions, IsHistoricCommitmentOptions, IsKnownRootOptions,
    IsSpentNullifierOptions, MinRollupFeeOptions, NullifierCollection, Scanner, ScannerOptions, TransactOptions,
    TransactionSigner, WalletCollection, WalletHandler, Wallets,
};
use mystiko_ethers::{Provider, ProviderWrapper};
use mystiko_protocol::address::ShieldedAddress;
use mystiko_protocol::commitment::Commitment as ProtocolCommitment;
use mystiko_protocol::commitment::Note;
use mystiko_protocol::key::{separate_secret_keys, verification_secret_key};
use mystiko_protocol::types::{FullSk, VerifySk};
use mystiko_protocol::utils::compute_nullifier;
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::handler::v1::{CreateAccountOptions, CreateWalletOptions};
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_protos::data::v1::{Commitment as ProtosCommitment, Nullifier as ProtosNullifier};
use mystiko_protos::sequencer::v1::{FetchChainRequest, FetchChainResponse};
use mystiko_sequencer_client::v1::SequencerClientError;
use mystiko_sequencer_client::{
    ChainLoadedBlock, CommitmentHashes, CommitmentsWithContract, GetCommitmentHashesOptions, NullifiersWithContract,
};
use mystiko_storage::{Document, SqlStatementFormatter};
use mystiko_storage_sqlite::SqliteStorage;
use mystiko_utils::hex::{decode_hex_with_length, encode_hex};
use num_bigint::BigUint;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;
use std::sync::Arc;
use typed_builder::TypedBuilder;

pub(crate) const DEFAULT_WALLET_PASSWORD: &str = "P@ssw0rd";

mock! {
    #[derive(Debug, Default)]
    CommitmentPoolContracts {}

    #[async_trait]
    impl CommitmentPoolContractHandler for CommitmentPoolContracts {
        type Error = anyhow::Error;
        async fn is_historic_commitment(&self, options: IsHistoricCommitmentOptions) -> Result<bool>;
        async fn is_spent_nullifier(&self, options: IsSpentNullifierOptions) -> Result<bool>;
        async fn is_known_root(&self, options: IsKnownRootOptions) -> Result<bool>;
        async fn min_rollup_fee(&self, options: MinRollupFeeOptions) -> Result<U256>;
        async fn get_commitment_included_count(&self, options: IncludedCountOptions) -> Result<U256>;
        async fn auditor_public_keys(&self, options: AuditorPublicKeysOptions) -> Result<Vec<U256>>;
        async fn transact<T, S>(&self, options: TransactOptions<T, S>) -> Result<TxHash>
        where
            T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static,
            S: TransactionSigner + 'static;
    }
}

mock! {
    #[derive(Debug)]
    SequencerClient {}
    #[async_trait]
    impl mystiko_sequencer_client::SequencerClient<FetchChainRequest, FetchChainResponse, ProtosCommitment, ProtosNullifier> for SequencerClient {
        type Error = SequencerClientError;
        async fn chain_loaded_block(&self, chain_id: u64, with_contracts: bool) ->Result<ChainLoadedBlock, SequencerClientError>;
        async fn contract_loaded_block(&self, chain_id: u64, contract_address: &Address) -> Result<u64, SequencerClientError>;
        async fn fetch_chain(&self, request: FetchChainRequest) -> Result<FetchChainResponse, SequencerClientError>;
        async fn get_commitments(
            &self,
            chain_id: u64,
            contract_address: &Address,
            commitment_hashes: &[BigUint],
        ) -> Result<Vec<ProtosCommitment>, SequencerClientError>;
        async fn get_commitments_by_tx_hash(
            &self,
            chain_id: u64,
            tx_hash: &TxHash) -> Result<CommitmentsWithContract<ProtosCommitment>, SequencerClientError>;
        async fn get_commitment_hashes(
            &self,
            options: &GetCommitmentHashesOptions) -> Result<CommitmentHashes, SequencerClientError>;
        async fn get_nullifiers(
            &self,
            chain_id: u64,
            contract_address: &Address,
            nullifier_hashes: &[BigUint],
        ) -> Result<Vec<ProtosNullifier>, SequencerClientError>;
        async fn get_nullifiers_by_tx_hash(
            &self,
            chain_id: u64,
            tx_hash: &TxHash) -> Result<NullifiersWithContract<ProtosNullifier>, SequencerClientError>;
        async fn health_check(&self) -> Result<(), SequencerClientError>;
    }
}

#[derive(Debug, TypedBuilder)]
pub struct TestAccount {
    shielded_address: ShieldedAddress,
    v_sk: VerifySk,
}

async fn create_scanner(
    account_count: usize,
    mnemonic_phrase: Option<String>,
    provider: HashMap<u64, MockProvider>,
    commitment_pool_contracts: Option<MockCommitmentPoolContracts>,
    sequencer_client: Option<MockSequencerClient>,
) -> (
    Scanner<SqlStatementFormatter, SqliteStorage, MockCommitmentPoolContracts, MockSequencerClient, MockProviders>,
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
        .mnemonic_phrase(mnemonic_phrase)
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
    let raw_providers = provider
        .into_iter()
        .map(|(chain_id, provider)| {
            let wrapped_provider = Arc::new(Provider::new(ProviderWrapper::new(Box::new(provider))));
            (chain_id, wrapped_provider.clone())
        })
        .collect::<HashMap<_, _>>();
    let mut providers = MockProviders::new();
    providers.expect_get_provider().returning(move |chain_id| {
        raw_providers
            .get(&chain_id)
            .cloned()
            .ok_or(anyhow::anyhow!("No provider for chain_id {}", chain_id))
    });
    let pool_contracts = commitment_pool_contracts.unwrap_or_default();
    let sequencer = sequencer_client.unwrap_or_default();
    let options = ScannerOptions::builder()
        .db(db.clone())
        .providers(providers)
        .commitment_pool_contracts(pool_contracts)
        .sequencer(sequencer)
        .build();
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
                cm.commitment_hash.clone_from(&pcm.commitment_hash);
                cm.encrypted_note = Some(encode_hex(pcm.encrypted_note));

                let nullifier = compute_nullifier(&account.v_sk, &pcm.note.random_p).unwrap();
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

fn build_mock_provider_with_queued_event(log_count: u32) -> MockProvider {
    let mut mock_provider = MockProvider::new();
    mock_provider.expect_request().returning(move |method, _| match method {
        "eth_getTransactionReceipt" => {
            let logs = vec![
                Log {
                    address: Address::from_str("0x00b73dbC8C370CA7e5F00b778280596383b62929").unwrap(),
                    block_hash: Some(H256::from_str("0x224ac34e68f04a2d134affb0bf9181bae2cc4e7376a60687c072119247fb0e78").unwrap()),
                    block_number: Some(U64::from(10000000)),
                    data: Bytes::from_str("0x00000000000000000000000000000000000000000000000000000000000f4240000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000d1d254df768aad077345bcca57768ba1df0424264ca8f76a2fab3549b051e5d0339f73f26c5d4f2e0e1e2575829a0a7afeb0a380f67b100b866b0bfbc84713de7b92d622a98cdbec15690d203cc33b428591d8f549ff7e48e44475f14c3d04b4c9c1c16c3c8beac96ae05db6762e01466d49e04422b9d84a7335b798bb7fbccb2b2fe7732bd991c9a29c861b6c48a692da6e02af21b6a0410aedbdf5f4a3094729873d58f42cc59ac41c54a2b7b2b392277ed72a1b7121efc24d039d7830e20e1f3cb60de6e96e33ed7f344962635832c173000000000000000000000000000000").unwrap(),
                    log_index: Some(U256::from(250)),
                    transaction_log_index: None,
                    log_type: None,
                    removed: Some(false),
                    topics: vec![
                        H256::from_str("0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9").unwrap(),
                        H256::from_str("0x1db84c1b0bd7877f4cddd3f5b0a8ae202b017234f84dc75face85b7556951fc4").unwrap(),
                    ],
                    transaction_hash: Some(H256::from_str("0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2").unwrap()),
                    transaction_index: Some(U64::from(51)),
                },
                Log {
                    address: Address::from_str("0x00b73dbC8C370CA7e5F00b778280596383b62929").unwrap(),
                    block_hash: Some(H256::from_str("0x224ac34e68f04a2d134affb0bf9181bae2cc4e7376a60687c072119247fb0e78").unwrap()),
                    block_number: Some(U64::from(10000000)),
                    data: Bytes::from_str("0x00000000000000000000000000000000000000000000000000000000000f4240000000000000000000000000000000000000000000000000000000000000000b000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000d161e3ffa2a53509de01250f2b8af2e0d50433e50f647361b877918a55745d075c9bbf7aa8a1f41bd3641a606a0cbcd00d459fcf231acafdec6eca9b7aacfd5f67e1b63529a0564e82cdde6754d6b946f5bae36f748e3295276df68b8b19d1d8bccd8105d7feca23c5f6288a78f9f038c5d3594d2ef903fc145b07b203f98796c58a9210a5f7c37e07215f9b1ac273c51688c6b72dad855e7008d81f69c28762b289caf7d93d44b6416f7fce60b53669624a9ec0d3289f2a8ee8b8811661fc73d9ece7376574c15baa8a8b21083aa49f8d54000000000000000000000000000000").unwrap(),
                    log_index: Some(U256::from(251)),
                    transaction_log_index: None,
                    log_type: None,
                    removed: Some(false),
                    topics: vec![
                        H256::from_str("0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9").unwrap(),
                        H256::from_str("0x18812c5d6d451a1c7396e04aaaed04ddbbe8a3908d3db55a890a9527ba4ea8c3").unwrap(),
                    ],
                    transaction_hash: Some(H256::from_str("0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2").unwrap()),
                    transaction_index: Some(U64::from(51)),
                }
            ];
            let receipt = TransactionReceipt {
                transaction_hash: H256::from_str("0x224ac34e68f04a2d134affb0bf9181bae2cc4e7376a60687c072119247fb0e78")
                    .unwrap(),
                block_number: Some(U64::from(10000000)),
                logs: logs.split_at(log_count as usize).0.to_vec(),
                ..Default::default()
            };
            let receipt = serde_json::json!(receipt);
            Ok(receipt)
        }
        _ => Err(ethers_providers::ProviderError::CustomError(
            "mock provider not support".to_string(),
        )),
    });

    mock_provider
}

fn build_mock_provider_with_cross_chain_event(log_count: u32) -> MockProvider {
    let mut mock_provider = MockProvider::new();
    mock_provider.expect_request().returning(move |method, _| match method {
        "eth_getTransactionReceipt" => {
            let logs = vec![
                Log {
                    address: Address::from_str("0x961F315A836542e603A3df2E0dd9d4ECd06ebC67").unwrap(),
                    block_hash: Some(
                        H256::from_str("0x224ac34e68f04a2d134affb0bf9181bae2cc4e7376a60687c072119247fb0e78").unwrap(),
                    ),
                    block_number: Some(U64::from(10000000)),
                    data: vec![].into(),
                    log_index: Some(U256::from(200)),
                    transaction_log_index: None,
                    log_type: None,
                    removed: Some(false),
                    topics: vec![
                        H256::from_str("0xd106eb38b3368b7c294e36fae5513fdefe880be5abfad529b37b044f2fdd2dbe").unwrap(),
                        H256::from_str("0x1db84c1b0bd7877f4cddd3f5b0a8ae202b017234f84dc75face85b7556951fc4").unwrap(),
                    ],
                    transaction_hash: Some(
                        H256::from_str("0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2").unwrap(),
                    ),
                    transaction_index: Some(U64::from(51)),
                },
                Log {
                    address: Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap(),
                    block_hash: Some(
                        H256::from_str("0x224ac34e68f04a2d134affb0bf9181bae2cc4e7376a60687c072119247fb0e78").unwrap(),
                    ),
                    block_number: Some(U64::from(10000001)),
                    data: vec![].into(),
                    log_index: Some(U256::from(201)),
                    transaction_log_index: None,
                    log_type: None,
                    removed: Some(false),
                    topics: vec![
                        H256::from_str("0xd106eb38b3368b7c294e36fae5513fdefe880be5abfad529b37b044f2fdd2dbe").unwrap(),
                        H256::from_str("0x18812c5d6d451a1c7396e04aaaed04ddbbe8a3908d3db55a890a9527ba4ea8c3").unwrap(),
                    ],
                    transaction_hash: Some(
                        H256::from_str("0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2").unwrap(),
                    ),
                    transaction_index: Some(U64::from(51)),
                },
            ];
            let receipt = TransactionReceipt {
                transaction_hash: H256::from_str("0x224ac34e68f04a2d134affb0bf9181bae2cc4e7376a60687c072119247fb0e78")
                    .unwrap(),
                block_number: Some(U64::from(10000000)),
                logs: logs.split_at(log_count as usize).0.to_vec(),
                ..Default::default()
            };
            let receipt = serde_json::json!(receipt);
            Ok(receipt)
        }
        _ => Err(ethers_providers::ProviderError::CustomError(
            "mock provider not support".to_string(),
        )),
    });

    mock_provider
}
