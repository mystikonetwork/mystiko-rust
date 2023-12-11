use crate::common::{create_database, MockProvider, MockProviders};
use crate::handler::{
    MockCommitmentPoolContracts, MockPublicAssets, MockRelayerClient, MockStaticCache, MockTransactions, MockZKProver,
};
use mystiko_config::{MystikoConfig, PoolContractConfig};
use mystiko_core::{
    AccountHandler, Accounts, Commitment, Database, Spends, SpendsError, SpendsOptions, WalletHandler, Wallets,
};
use mystiko_crypto::crypto::encrypt_asymmetric;
use mystiko_ethers::{Provider, ProviderWrapper};
use mystiko_protocol::address::ShieldedAddress;
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::document::v1::{Account, Wallet};
use mystiko_protos::core::handler::v1::{CreateAccountOptions, CreateWalletOptions};
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_relayer_client::RelayerClient;
use mystiko_static_cache::StaticCache;
use mystiko_storage::{Document, SqlStatementFormatter};
use mystiko_storage_sqlite::SqliteStorage;
use mystiko_utils::hex::encode_hex_with_prefix;
use num_bigint::BigUint;
use std::collections::HashMap;
use std::sync::Arc;
use typed_builder::TypedBuilder;

pub(crate) type DatabaseType = Database<SqlStatementFormatter, SqliteStorage>;

pub(crate) type SpendsOptionsType<R = MockRelayerClient> = SpendsOptions<
    SqlStatementFormatter,
    SqliteStorage,
    MockPublicAssets,
    MockCommitmentPoolContracts,
    MockTransactions,
    MockProviders,
    R,
    MockZKProver,
>;

pub(crate) type SpendsType<R = MockRelayerClient> = Spends<
    SqlStatementFormatter,
    SqliteStorage,
    MockPublicAssets,
    MockCommitmentPoolContracts,
    MockTransactions,
    MockProviders,
    R,
    MockZKProver,
>;

#[derive(Debug, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub(crate) struct MockOptions<R: Default = MockRelayerClient> {
    pub(crate) config: Option<MystikoConfig>,
    pub(crate) assets: MockPublicAssets,
    pub(crate) commitment_pool_contracts: MockCommitmentPoolContracts,
    pub(crate) transactions: MockTransactions,
    pub(crate) static_cache: MockStaticCache,
    pub(crate) relayer_client: R,
    pub(crate) prover: MockZKProver,
    pub(crate) providers: HashMap<u64, MockProvider>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub(crate) struct CommitmentOptions {
    pub(crate) status: CommitmentStatus,
    #[builder(default)]
    pub(crate) spent: bool,
    #[builder(default)]
    pub(crate) shielded_address: Option<String>,
    #[builder(default)]
    pub(crate) amount: Option<BigUint>,
    #[builder(default)]
    pub(crate) leaf_index: Option<u64>,
}

pub(crate) async fn setup<R>(options: MockOptions<R>) -> (Arc<MystikoConfig>, Arc<DatabaseType>, SpendsType<R>)
where
    R: RelayerClient + Default,
    SpendsError: From<R::Error>,
{
    let _ = env_logger::builder()
        .filter_module("mystiko_core", log::LevelFilter::Info)
        .try_init();
    let config = Arc::new(
        options.config.unwrap_or(
            MystikoConfig::from_json_file("tests/files/mystiko/config.json")
                .await
                .unwrap(),
        ),
    );
    let database = Arc::new(create_database().await);
    database.migrate().await.unwrap();
    let mut raw_providers = options
        .providers
        .into_iter()
        .map(|(chain_id, provider)| {
            let provider = Arc::new(Provider::new(ProviderWrapper::new(Box::new(provider))));
            (chain_id, provider)
        })
        .collect::<HashMap<_, _>>();
    let mut providers = MockProviders::new();
    providers.expect_get_provider().returning(move |chain_id| {
        raw_providers
            .remove(&chain_id)
            .ok_or(anyhow::anyhow!("No provider for chain_id {}", chain_id))
    });
    let handler_options = SpendsOptionsType::builder()
        .db(database.clone())
        .config(config.clone())
        .commitment_pool_contracts(options.commitment_pool_contracts)
        .assets(options.assets)
        .transactions(options.transactions)
        .static_cache(Box::new(options.static_cache) as Box<dyn StaticCache>)
        .relayers(options.relayer_client)
        .prover(options.prover)
        .signer_providers(providers)
        .build();
    let handler = SpendsType::new(handler_options);
    (config, database, handler)
}

pub(crate) async fn create_wallet(db: Arc<DatabaseType>) -> (Wallet, Account) {
    let wallets = Wallets::new(db.clone());
    let accounts = Accounts::new(db);
    let wallet = wallets
        .create(&CreateWalletOptions::builder().password("P@ssw0rd").build())
        .await
        .unwrap();
    let account = accounts
        .create(&CreateAccountOptions::builder().wallet_password("P@ssw0rd").build())
        .await
        .unwrap();
    (wallet, account)
}

pub(crate) async fn generate_commitments(
    db: Arc<DatabaseType>,
    chain_id: u64,
    contract_config: &PoolContractConfig,
    options: &[CommitmentOptions],
) -> Vec<Document<Commitment>> {
    let commitments = options
        .iter()
        .enumerate()
        .map(|(index, option)| {
            let encrypted_note = option.shielded_address.as_ref().map(|shielded_address| {
                let shielded_address = ShieldedAddress::from_string(shielded_address).unwrap();
                let (_, pk_enc) = shielded_address.public_key().unwrap();
                let note = mystiko_protocol::commitment::Note::new(option.amount.clone(), None).unwrap();
                let encrypted_note = encrypt_asymmetric(&pk_enc, &note.to_vec()).unwrap();
                encode_hex_with_prefix(encrypted_note)
            });
            Commitment {
                chain_id,
                contract_address: contract_config.address().to_string(),
                bridge_type: BridgeType::Loop as i32,
                commitment_hash: BigUint::from(index),
                asset_symbol: contract_config.asset_symbol().to_string(),
                asset_decimals: contract_config.asset_decimals(),
                asset_address: contract_config.asset_address().map(|address| address.to_string()),
                status: option.status as i32,
                spent: option.spent,
                block_number: index as u64,
                src_chain_block_number: None,
                included_block_number: None,
                rollup_fee_amount: None,
                encrypted_note,
                leaf_index: option.leaf_index,
                amount: option.amount.clone(),
                nullifier: Some(BigUint::from(index)),
                shielded_address: option.shielded_address.clone(),
                queued_transaction_hash: None,
                included_transaction_hash: None,
                src_chain_transaction_hash: None,
            }
        })
        .collect::<Vec<_>>();
    db.commitments.insert_batch(&commitments).await.unwrap()
}
