use crate::handler::spend::setup::{
    create_wallet, generate_commitments, setup, CommitmentOptions, DatabaseType, MockOptions, SpendsType,
};
use crate::handler::{MockCommitmentPoolContracts, MockDataPackerClient, MockRelayerClient};
use mystiko_config::{MystikoConfig, PoolContractConfig};
use mystiko_core::{Commitment, SpendHandler, SpendsError};
use mystiko_datapacker_client::DataPackerClient;
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::document::v1::Account;
use mystiko_protos::core::handler::v1::{AmountRange, CreateSpendOptions, SpendQuote, SpendSummary};
use mystiko_protos::core::v1::SpendType;
use mystiko_protos::data::v1::{ChainData, CommitmentStatus, MerkleTree as ProtoMerkleTree};
use mystiko_relayer_client::types::register::RegisterInfo;
use mystiko_relayer_client::RelayerClient;
use mystiko_relayer_types::ContractInfo;
use mystiko_storage::Document;
use mystiko_types::CircuitType;
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::{biguint_to_u256, number_to_biguint_decimal};
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[tokio::test]
async fn test_withdraw_summary() {
    let test_options: SummaryTestOptions = SummaryTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x223903804Ee95e264F74C88B4F8583429524593c".to_string())
        .build();
    let context = SummaryTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;
    let mut summary_options = CreateSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Withdraw as i32)
        .amount(12)
        .recipient("0xFE500c274F72f1d1a9978c903d97E6d45CD9121B".to_string())
        .build();
    let mut expected_summary = SpendSummary {
        asset_symbol: "MTT".to_string(),
        asset_decimals: 16,
        current_balance: 17.0,
        current_decimal_balance: "170000000000000000".to_string(),
        new_balance: 5.0,
        new_decimal_balance: "50000000000000000".to_string(),
        amount: 12.0,
        decimal_amount: "120000000000000000".to_string(),
        recipient: "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B".to_string(),
        rollup_fee_amount: 4.0,
        rollup_fee_decimal_amount: "40000000000000000".to_string(),
        rollup_fee_total_amount: 4.0,
        rollup_fee_total_decimal_amount: "40000000000000000".to_string(),
        rollup_fee_asset_symbol: "MTT".to_string(),
        rollup_fee_asset_decimals: 16,
        ..Default::default()
    };
    let mut summary = context.handler.summary(summary_options.clone()).await.unwrap();
    assert_eq!(summary, expected_summary);

    summary_options.amount = 14.0;
    summary_options.rollup_fee_amount = Some(0.0);
    expected_summary.amount = 14.0;
    expected_summary.decimal_amount = "140000000000000000".to_string();
    expected_summary.new_balance = 3.0;
    expected_summary.new_decimal_balance = "30000000000000000".to_string();
    expected_summary.rollup_fee_amount = 0.0;
    expected_summary.rollup_fee_decimal_amount = "0".to_string();
    expected_summary.rollup_fee_total_amount = 0.0;
    expected_summary.rollup_fee_total_decimal_amount = "0".to_string();
    summary = context.handler.summary(summary_options).await.unwrap();
    assert_eq!(summary, expected_summary);
}

#[tokio::test]
async fn test_transfer_summary() {
    let test_options: SummaryTestOptions = SummaryTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x223903804Ee95e264F74C88B4F8583429524593c".to_string())
        .build();
    let context = SummaryTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;
    let summary_options = CreateSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Transfer as i32)
        .amount(12)
        .rollup_fee_amount(4.0)
        .recipient(context.account.shielded_address.clone())
        .build();
    let expected_summary = SpendSummary {
        asset_symbol: "MTT".to_string(),
        asset_decimals: 16,
        current_balance: 17.0,
        current_decimal_balance: "170000000000000000".to_string(),
        new_balance: 5.0,
        new_decimal_balance: "50000000000000000".to_string(),
        amount: 12.0,
        decimal_amount: "120000000000000000".to_string(),
        recipient: context.account.shielded_address.clone(),
        rollup_fee_amount: 4.0,
        rollup_fee_decimal_amount: "40000000000000000".to_string(),
        rollup_fee_total_amount: 8.0,
        rollup_fee_total_decimal_amount: "80000000000000000".to_string(),
        rollup_fee_asset_symbol: "MTT".to_string(),
        rollup_fee_asset_decimals: 16,
        ..Default::default()
    };
    let summary = context.handler.summary(summary_options).await.unwrap();
    assert_eq!(summary, expected_summary);
}

#[tokio::test]
async fn test_summary_with_gas_relayer() {
    let mut relayer_client = MockRelayerClient::new();
    let gas_relayers = vec![
        RegisterInfo::builder()
            .chain_id(5_u64)
            .name("test_relayer_1".to_string())
            .url("https://test_relayer1.mystiko.network".to_string())
            .relayer_address("0x357c6Fd2cEE77bA5de49e0bB9d49444781A8f0cc".to_string())
            .relayer_contract_address("0x3f4a3378852887b81dFE593Ee1A68Be4adcd888d".to_string())
            .available(true)
            .support(true)
            .contracts(vec![ContractInfo::builder()
                .asset_symbol("MTT".to_string())
                .relayer_fee_of_ten_thousandth(10_u32)
                .minimum_gas_fee("100000000000000".to_string())
                .build()])
            .build(),
        RegisterInfo::builder()
            .chain_id(5_u64)
            .name("test_relayer_2".to_string())
            .url("https://test_relayer2.mystiko.network".to_string())
            .relayer_address("0x3ca553F90c156B086f60C639E957B469464C8924".to_string())
            .relayer_contract_address("0x3c11F6265Ddec22f4d049Dde480615735f451646".to_string())
            .available(true)
            .support(true)
            .contracts(vec![ContractInfo::builder()
                .asset_symbol("MTT".to_string())
                .relayer_fee_of_ten_thousandth(9_u32)
                .minimum_gas_fee("110000000000000".to_string())
                .build()])
            .build(),
    ];
    relayer_client
        .expect_register_info()
        .withf(move |options| {
            let more_options = options.options.as_ref().unwrap();
            options.chain_id == 5_u64
                && options.name.is_none()
                && more_options.asset_symbol == "MTT"
                && more_options.circuit_type == CircuitType::Transaction2x1
                && !more_options.show_unavailable
        })
        .returning(move |_| Ok(gas_relayers.clone()));
    let test_options: SummaryTestOptions = SummaryTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x223903804Ee95e264F74C88B4F8583429524593c".to_string())
        .relayer_client(relayer_client)
        .build();
    let context = SummaryTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;

    let summary_options = CreateSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Withdraw as i32)
        .amount(12)
        .rollup_fee_amount(4.0)
        .recipient("0xFE500c274F72f1d1a9978c903d97E6d45CD9121B".to_string())
        .gas_relayer("test_relayer_2".to_string())
        .build();
    let expected_summary = SpendSummary {
        asset_symbol: "MTT".to_string(),
        asset_decimals: 16,
        current_balance: 17.0,
        current_decimal_balance: "170000000000000000".to_string(),
        new_balance: 5.0,
        new_decimal_balance: "50000000000000000".to_string(),
        amount: 12.0,
        decimal_amount: "120000000000000000".to_string(),
        recipient: "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B".to_string(),
        rollup_fee_amount: 4.0,
        rollup_fee_decimal_amount: "40000000000000000".to_string(),
        rollup_fee_total_amount: 4.0,
        rollup_fee_total_decimal_amount: "40000000000000000".to_string(),
        rollup_fee_asset_symbol: "MTT".to_string(),
        rollup_fee_asset_decimals: 16,
        gas_relayer_fee_amount: Some(0.0218),
        gas_relayer_fee_decimal_amount: Some("218000000000000".to_string()),
        gas_relayer_fee_asset_symbol: Some("MTT".to_string()),
        gas_relayer_fee_asset_decimals: Some(16),
        gas_relayer_address: Some("0x3ca553F90c156B086f60C639E957B469464C8924".to_string()),
        gas_relayer_name: Some("test_relayer_2".to_string()),
        gas_relayer_url: Some("https://test_relayer2.mystiko.network".to_string()),
    };
    let summary = context.handler.summary(summary_options).await.unwrap();
    assert_eq!(summary, expected_summary);
}

#[tokio::test]
async fn test_summary_with_quote() {
    let test_options: SummaryTestOptions = SummaryTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x223903804Ee95e264F74C88B4F8583429524593c".to_string())
        .build();
    let context = SummaryTestContext::new(test_options).await;
    let quote = SpendQuote {
        valid: true,
        asset_symbol: "MTT".to_string(),
        asset_decimals: 16,
        current_balance: 17.0,
        current_decimal_balance: "170000000000000000".to_string(),
        num_of_inputs: 2,
        num_of_outputs: 1,
        min_rollup_fee: 4.0,
        min_rollup_fee_decimal: "40000000000000000".to_string(),
        rollup_fee_asset_symbol: "MTT".to_string(),
        rollup_fee_asset_decimals: 16,
        fixed_amounts: vec![3.0, 4.0],
        fixed_decimal_amounts: vec!["30000000000000000".to_string(), "40000000000000000".to_string()],
        amount_range: Some(AmountRange {
            min: 4.0,
            decimal_min: "40000000000000000".to_string(),
            max: 14.0,
            decimal_max: "140000000000000000".to_string(),
        }),
        selected_commitments: vec!["1".to_string(), "2".to_string()],
        ..Default::default()
    };
    let summary_options = CreateSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Withdraw as i32)
        .amount(12)
        .rollup_fee_amount(4.0)
        .recipient("0xFE500c274F72f1d1a9978c903d97E6d45CD9121B".to_string())
        .spend_quote(quote)
        .build();
    let expected_summary = SpendSummary {
        asset_symbol: "MTT".to_string(),
        asset_decimals: 16,
        current_balance: 17.0,
        current_decimal_balance: "170000000000000000".to_string(),
        new_balance: 5.0,
        new_decimal_balance: "50000000000000000".to_string(),
        amount: 12.0,
        decimal_amount: "120000000000000000".to_string(),
        recipient: "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B".to_string(),
        rollup_fee_amount: 4.0,
        rollup_fee_decimal_amount: "40000000000000000".to_string(),
        rollup_fee_total_amount: 4.0,
        rollup_fee_total_decimal_amount: "40000000000000000".to_string(),
        rollup_fee_asset_symbol: "MTT".to_string(),
        rollup_fee_asset_decimals: 16,
        ..Default::default()
    };
    let summary = context.handler.summary(summary_options).await.unwrap();
    assert_eq!(summary, expected_summary);
}

#[tokio::test]
async fn test_summary_invalid_recipient_address() {
    let test_options: SummaryTestOptions = SummaryTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x223903804Ee95e264F74C88B4F8583429524593c".to_string())
        .build();
    let context = SummaryTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;

    let mut summary_options = CreateSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Withdraw as i32)
        .amount(12)
        .rollup_fee_amount(4.0)
        .recipient("0x1234".to_string())
        .build();
    let mut summary = context.handler.summary(summary_options.clone()).await;
    assert_eq!(summary.unwrap_err().to_string(), "invalid public address 0x1234");

    summary_options.spend_type = Some(SpendType::Transfer as i32);
    summary = context.handler.summary(summary_options).await;
    assert_eq!(summary.unwrap_err().to_string(), "invalid mystiko address 0x1234");
}

#[tokio::test]
async fn test_summary_invalid_amount() {
    let mut relayer_client = MockRelayerClient::new();
    let gas_relayers = vec![RegisterInfo::builder()
        .chain_id(5_u64)
        .name("test_relayer_1".to_string())
        .url("https://test_relayer1.mystiko.network".to_string())
        .relayer_address("0x357c6Fd2cEE77bA5de49e0bB9d49444781A8f0cc".to_string())
        .relayer_contract_address("0x3f4a3378852887b81dFE593Ee1A68Be4adcd888d".to_string())
        .available(true)
        .support(true)
        .contracts(vec![ContractInfo::builder()
            .asset_symbol("MTT".to_string())
            .relayer_fee_of_ten_thousandth(10_u32)
            .minimum_gas_fee("30000000000000000".to_string())
            .build()])
        .build()];
    relayer_client
        .expect_register_info()
        .withf(move |options| {
            let more_options = options.options.as_ref().unwrap();
            options.chain_id == 5_u64
                && options.name.is_none()
                && more_options.asset_symbol == "MTT"
                && !more_options.show_unavailable
        })
        .returning(move |_| Ok(gas_relayers.clone()));
    let test_options: SummaryTestOptions = SummaryTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x223903804Ee95e264F74C88B4F8583429524593c".to_string())
        .relayer_client(relayer_client)
        .build();
    let context = SummaryTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;

    let mut summary_options = CreateSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Withdraw as i32)
        .amount(3.0)
        .recipient("0xFE500c274F72f1d1a9978c903d97E6d45CD9121B".to_string())
        .gas_relayer("test_relayer_1".to_string())
        .build();
    let mut summary = context.handler.summary(summary_options.clone()).await;
    assert_eq!(summary.unwrap_err().to_string(), "invalid amount: 3");

    summary_options.amount = 12.0;
    summary_options.rollup_fee_amount = Some(1.0);
    summary = context.handler.summary(summary_options.clone()).await;
    assert_eq!(summary.unwrap_err().to_string(), "invalid rollup fee amount: 1");

    summary_options.amount = 14.0;
    summary_options.rollup_fee_amount = Some(4.0);
    summary = context.handler.summary(summary_options).await;
    assert_eq!(summary.unwrap_err().to_string(), "invalid rollup fee amount: 4");
}

#[tokio::test]
async fn test_summary_invalid_quote() {
    let test_options: SummaryTestOptions = SummaryTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x223903804Ee95e264F74C88B4F8583429524593c".to_string())
        .build();
    let context = SummaryTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;

    let summary_options = CreateSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Transfer as i32)
        .amount(1.0)
        .rollup_fee_amount(4.0)
        .recipient("0xFE500c274F72f1d1a9978c903d97E6d45CD9121B".to_string())
        .build();
    let summary = context.handler.summary(summary_options).await;
    assert_eq!(
        summary.unwrap_err().to_string(),
        "invalid create options with code: InvalidAmount"
    );
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
struct SummaryTestOptions<R: Default = MockRelayerClient, K: Default = MockDataPackerClient> {
    chain_id: u64,
    contract_address: String,
    #[builder(default)]
    relayer_client: R,
    #[builder(default)]
    data_packer_client: K,
}

struct SummaryTestContext<R = MockRelayerClient, K = MockDataPackerClient> {
    db: Arc<DatabaseType>,
    handler: SpendsType<R, K>,
    account: Account,
    chain_id: u64,
    contract_config: PoolContractConfig,
}

impl<R, K> SummaryTestContext<R, K>
where
    R: RelayerClient + Default,
    K: DataPackerClient<ChainData, ProtoMerkleTree> + Default,
    SpendsError: From<R::Error>,
{
    async fn new(options: SummaryTestOptions<R, K>) -> Self {
        let SummaryTestOptions {
            chain_id,
            contract_address,
            relayer_client,
            data_packer_client,
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
            .data_packer_client(data_packer_client)
            .build();
        let (_, db, handler) = setup::<R, K>(mock_options).await;
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
