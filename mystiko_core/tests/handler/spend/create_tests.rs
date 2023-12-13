use crate::handler::spend::setup::{
    create_wallet, generate_commitments, setup, CommitmentOptions, DatabaseType, MockOptions, SpendsType,
};
use crate::handler::{MockCommitmentPoolContracts, MockRelayerClient};
use mystiko_config::{MystikoConfig, PoolContractConfig};
use mystiko_core::{Commitment, SpendHandler, SpendsError};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::document::v1::Account;
use mystiko_protos::core::handler::v1::CreateSpendOptions;
use mystiko_protos::core::v1::{SpendStatus, SpendType};
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_relayer_client::RelayerClient;
use mystiko_storage::Document;
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::{biguint_to_u256, number_to_biguint_decimal};
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[tokio::test]
async fn test_withdraw_create() {
    let test_options: CreateTestOptions = CreateTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x223903804Ee95e264F74C88B4F8583429524593c".to_string())
        .build();
    let context = CreateTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;

    let create_options = CreateSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Withdraw as i32)
        .amount(12)
        .rollup_fee_amount(4.0)
        .recipient("0xFE500c274F72f1d1a9978c903d97E6d45CD9121B".to_string())
        .wallet_password("P@ssw0rd".to_string())
        .build();
    let mut spend = context.handler.create(create_options).await.unwrap();
    spend.input_commitments.sort();
    assert_eq!(spend.chain_id, 5_u64);
    assert_eq!(spend.contract_address, "0x223903804Ee95e264F74C88B4F8583429524593c");
    assert_eq!(spend.asset_symbol, "MTT".to_string());
    assert_eq!(spend.asset_decimals, 16);
    assert!(spend.root_hash.is_none());
    assert_eq!(spend.amount, 12.0);
    assert_eq!(spend.decimal_amount, "120000000000000000");
    assert_eq!(spend.recipient, "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B");
    assert_eq!(spend.wallet_id, context.account.wallet_id);
    assert_eq!(spend.input_commitments, vec!["1".to_string(), "2".to_string()]);
    assert_eq!(spend.output_commitments.len(), 1);
    assert!(spend.nullifiers.is_empty());
    assert!(spend.signature_public_key_hashes.is_empty());
    assert!(spend.encrypted_auditor_notes.is_empty());
    assert_eq!(spend.rollup_fee_amount, Some(4.0));
    assert_eq!(spend.rollup_fee_decimal_amount, Some("40000000000000000".to_string()));
    assert_eq!(spend.rollup_fee_total_amount, Some(4.0));
    assert_eq!(
        spend.rollup_fee_total_decimal_amount,
        Some("40000000000000000".to_string())
    );
    assert!(spend.gas_relayer_fee_amount.is_none());
    assert!(spend.gas_relayer_fee_decimal_amount.is_none());
    assert!(spend.signature_public_key.is_none());
    assert_eq!(
        spend.asset_address,
        Some("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string())
    );
    assert!(spend.proof.is_none());
    assert!(spend.gas_relayer_address.is_none());
    assert!(spend.gas_relayer_url.is_none());
    assert!(spend.signature.is_none());
    assert!(spend.random_auditing_public_key.is_none());
    assert!(spend.error_message.is_none());
    assert!(spend.transaction_hash.is_none());
    assert_eq!(spend.bridge_type, BridgeType::Loop as i32);
    assert_eq!(spend.spend_type, SpendType::Withdraw as i32);
    assert_eq!(spend.status, SpendStatus::Unspecified as i32);
}

#[tokio::test]
async fn test_transfer_create() {
    let test_options: CreateTestOptions = CreateTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x223903804Ee95e264F74C88B4F8583429524593c".to_string())
        .build();
    let context = CreateTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;

    let create_options = CreateSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Transfer as i32)
        .amount(12)
        .rollup_fee_amount(4.0)
        .recipient(context.account.shielded_address.clone())
        .wallet_password("P@ssw0rd".to_string())
        .build();
    let mut spend = context.handler.create(create_options).await.unwrap();
    spend.input_commitments.sort();
    assert_eq!(spend.chain_id, 5_u64);
    assert_eq!(spend.contract_address, "0x223903804Ee95e264F74C88B4F8583429524593c");
    assert_eq!(spend.asset_symbol, "MTT".to_string());
    assert_eq!(spend.asset_decimals, 16);
    assert!(spend.root_hash.is_none());
    assert_eq!(spend.amount, 12.0);
    assert_eq!(spend.decimal_amount, "120000000000000000");
    assert_eq!(spend.recipient, context.account.shielded_address);
    assert_eq!(spend.wallet_id, context.account.wallet_id);
    assert_eq!(spend.input_commitments, vec!["1".to_string(), "2".to_string()]);
    assert_eq!(spend.output_commitments.len(), 2);
    assert!(spend.nullifiers.is_empty());
    assert!(spend.signature_public_key_hashes.is_empty());
    assert!(spend.encrypted_auditor_notes.is_empty());
    assert_eq!(spend.rollup_fee_amount, Some(4.0));
    assert_eq!(spend.rollup_fee_decimal_amount, Some("40000000000000000".to_string()));
    assert_eq!(spend.rollup_fee_total_amount, Some(8.0));
    assert_eq!(
        spend.rollup_fee_total_decimal_amount,
        Some("80000000000000000".to_string())
    );
    assert!(spend.gas_relayer_fee_amount.is_none());
    assert!(spend.gas_relayer_fee_decimal_amount.is_none());
    assert!(spend.signature_public_key.is_none());
    assert_eq!(
        spend.asset_address,
        Some("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string())
    );
    assert!(spend.proof.is_none());
    assert!(spend.gas_relayer_address.is_none());
    assert!(spend.gas_relayer_url.is_none());
    assert!(spend.signature.is_none());
    assert!(spend.random_auditing_public_key.is_none());
    assert!(spend.error_message.is_none());
    assert!(spend.transaction_hash.is_none());
    assert_eq!(spend.bridge_type, BridgeType::Loop as i32);
    assert_eq!(spend.spend_type, SpendType::Transfer as i32);
    assert_eq!(spend.status, SpendStatus::Unspecified as i32);
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
struct CreateTestOptions<R: Default = MockRelayerClient> {
    chain_id: u64,
    contract_address: String,
    #[builder(default)]
    relayer_client: R,
}

struct CreateTestContext<R = MockRelayerClient> {
    db: Arc<DatabaseType>,
    handler: SpendsType<R>,
    account: Account,
    chain_id: u64,
    contract_config: PoolContractConfig,
}

impl<R> CreateTestContext<R>
where
    R: RelayerClient + Default,
    SpendsError: From<R::Error>,
{
    async fn new(options: CreateTestOptions<R>) -> Self {
        let CreateTestOptions {
            chain_id,
            contract_address,
            relayer_client,
        } = options;
        let config = MystikoConfig::from_json_file("tests/files/mystiko/config.json")
            .await
            .unwrap();
        let contract_config = config
            .find_pool_contract_by_address(chain_id, &contract_address)
            .unwrap()
            .clone();
        let contract_ethereum_address = ethers_address_from_string(&contract_address).unwrap();
        let min_rollup_fee = biguint_to_u256(&contract_config.min_rollup_fee().unwrap());
        let mut commitment_pool_contracts = MockCommitmentPoolContracts::new();
        commitment_pool_contracts
            .expect_min_rollup_fee()
            .withf(move |options| options.chain_id == chain_id && options.contract_address == contract_ethereum_address)
            .returning(move |_| Ok(min_rollup_fee));
        let mock_options = MockOptions::builder()
            .config(config)
            .commitment_pool_contracts(commitment_pool_contracts)
            .relayer_client(relayer_client)
            .build();
        let (_, db, handler) = setup::<R>(mock_options).await;
        let (_, account) = create_wallet(db.clone()).await;
        Self {
            db,
            handler,
            account,
            chain_id,
            contract_config,
        }
    }

    async fn generate_commitments(&self, amounts: &[f64]) -> Vec<Document<Commitment>> {
        let options = amounts
            .iter()
            .map(|amount| {
                CommitmentOptions::builder()
                    .status(CommitmentStatus::Included)
                    .shielded_address(self.account.shielded_address.clone())
                    .amount(number_to_biguint_decimal(*amount, Some(self.contract_config.asset_decimals())).unwrap())
                    .build()
            })
            .collect::<Vec<_>>();
        generate_commitments(self.db.clone(), self.chain_id, &self.contract_config, &options).await
    }
}
