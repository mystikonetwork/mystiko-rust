use crate::handler::spend::setup::{
    create_wallet, generate_commitments, setup, CommitmentOptions, DatabaseType, MockOptions, SpendsType,
};
use crate::handler::{MockCommitmentPoolContracts, MockRelayerClient, TimeoutRelayerClient};
use mystiko_config::{MystikoConfig, PoolContractConfig};
use mystiko_core::{Commitment, SpendHandler, SpendsError};
use mystiko_crypto::ecies::generate_secret_key;
use mystiko_protocol::address::ShieldedAddress;
use mystiko_protocol::key::{encryption_public_key, verification_public_key};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::document::v1::Account;
use mystiko_protos::core::handler::v1::{AmountRange, GasRelayer, QuoteSpendOptions, SpendInvalidCode, SpendQuote};
use mystiko_protos::core::v1::SpendType;
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_relayer_client::types::register::RegisterInfo;
use mystiko_relayer_client::RelayerClient;
use mystiko_relayer_types::ContractInfo;
use mystiko_storage::Document;
use mystiko_types::CircuitType;
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::{biguint_to_u256, number_to_biguint_decimal, number_to_u256_decimal};
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[tokio::test]
async fn test_quote_withdraw_without_amount() {
    let test_options: QuoteTestOptions = QuoteTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x223903804Ee95e264F74C88B4F8583429524593c".to_string())
        .min_rollup_fee(3.0)
        .build();
    let context = QuoteTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;
    let quote_options = QuoteSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Withdraw as i32)
        .build();
    let expected_quote = SpendQuote {
        valid: true,
        asset_symbol: "MTT".to_string(),
        asset_decimals: 16,
        current_balance: 17.0,
        current_decimal_balance: "170000000000000000".to_string(),
        num_of_inputs: 2,
        num_of_outputs: 0,
        min_rollup_fee: 3.0,
        min_rollup_fee_decimal: "30000000000000000".to_string(),
        rollup_fee_asset_symbol: "MTT".to_string(),
        rollup_fee_asset_decimals: 16,
        fixed_amounts: vec![3.0],
        fixed_decimal_amounts: vec!["30000000000000000".to_string()],
        amount_range: Some(AmountRange {
            min: 3.0,
            decimal_min: "30000000000000000".to_string(),
            max: 14.0,
            decimal_max: "140000000000000000".to_string(),
        }),
        ..Default::default()
    };
    let quote = context.handler.quote(quote_options.clone()).await.unwrap();
    assert_eq!(quote, expected_quote);
}

#[tokio::test]
async fn test_quote_transfer_without_amount() {
    let test_options: QuoteTestOptions = QuoteTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x223903804Ee95e264F74C88B4F8583429524593c".to_string())
        .min_rollup_fee(3.0)
        .build();
    let context = QuoteTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;
    let quote_options = QuoteSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Transfer as i32)
        .build();
    let expected_quote = SpendQuote {
        valid: true,
        asset_symbol: "MTT".to_string(),
        asset_decimals: 16,
        current_balance: 17.0,
        current_decimal_balance: "170000000000000000".to_string(),
        num_of_inputs: 2,
        num_of_outputs: 1,
        min_rollup_fee: 3.0,
        min_rollup_fee_decimal: "30000000000000000".to_string(),
        rollup_fee_asset_symbol: "MTT".to_string(),
        rollup_fee_asset_decimals: 16,
        fixed_amounts: vec![4.0],
        fixed_decimal_amounts: vec!["40000000000000000".to_string()],
        amount_range: Some(AmountRange {
            min: 6.0,
            decimal_min: "60000000000000000".to_string(),
            max: 14.0,
            decimal_max: "140000000000000000".to_string(),
        }),
        ..Default::default()
    };
    let quote = context.handler.quote(quote_options.clone()).await.unwrap();
    assert_eq!(quote, expected_quote);
}

#[tokio::test]
async fn test_quote_without_amount_no_available_assets() {
    let test_options: QuoteTestOptions = QuoteTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x223903804Ee95e264F74C88B4F8583429524593c".to_string())
        .build();
    let context = QuoteTestContext::new(test_options).await;
    let quote_options = QuoteSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Withdraw as i32)
        .build();
    let quote = context.handler.quote(quote_options.clone()).await.unwrap();
    assert!(!quote.valid);
    assert_eq!(quote.invalid_code(), SpendInvalidCode::NoAvailableAssets);
}

#[tokio::test]
async fn test_quote_without_amount_with_min_rollup_fee_error() {
    let test_options: QuoteTestOptions = QuoteTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x223903804Ee95e264F74C88B4F8583429524593c".to_string())
        .min_rollup_fee_raise_error(true)
        .min_rollup_fee(3.0)
        .build();
    let context = QuoteTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;
    let quote_options = QuoteSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Withdraw as i32)
        .build();
    let expected_quote = SpendQuote {
        valid: true,
        asset_symbol: "MTT".to_string(),
        asset_decimals: 16,
        current_balance: 17.0,
        current_decimal_balance: "170000000000000000".to_string(),
        num_of_inputs: 2,
        num_of_outputs: 0,
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
        ..Default::default()
    };
    let quote = context.handler.quote(quote_options.clone()).await.unwrap();
    assert_eq!(quote, expected_quote);
}

#[tokio::test]
async fn test_quote_with_invalid_amount() {
    let test_options: QuoteTestOptions = QuoteTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x223903804Ee95e264F74C88B4F8583429524593c".to_string())
        .build();
    let context = QuoteTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;
    let mut quote_options = QuoteSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Withdraw as i32)
        .amount(2.0)
        .build();
    let mut quote = context.handler.quote(quote_options.clone()).await.unwrap();
    assert!(!quote.valid);
    assert_eq!(quote.invalid_code(), SpendInvalidCode::InvalidAmount);

    quote_options.spend_type = Some(SpendType::Transfer as i32);
    quote_options.amount = Some(4.0);
    quote = context.handler.quote(quote_options.clone()).await.unwrap();
    assert!(!quote.valid);
    assert_eq!(quote.invalid_code(), SpendInvalidCode::InvalidAmount);

    quote_options.amount = Some(8.0);
    quote = context.handler.quote(quote_options.clone()).await.unwrap();
    assert!(!quote.valid);
    assert_eq!(quote.invalid_code(), SpendInvalidCode::InvalidAmount);

    quote_options.amount = Some(14.1);
    quote = context.handler.quote(quote_options.clone()).await.unwrap();
    assert!(!quote.valid);
    assert_eq!(quote.invalid_code(), SpendInvalidCode::InvalidAmount);
}

#[tokio::test]
async fn test_quote_withdraw_with_amount() {
    let test_options: QuoteTestOptions = QuoteTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x223903804Ee95e264F74C88B4F8583429524593c".to_string())
        .build();
    let context = QuoteTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;

    let mut quote_options = QuoteSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Withdraw as i32)
        .amount(3.0)
        .build();
    let mut expected_quote = SpendQuote {
        valid: true,
        asset_symbol: "MTT".to_string(),
        asset_decimals: 16,
        current_balance: 17.0,
        current_decimal_balance: "170000000000000000".to_string(),
        num_of_inputs: 1,
        num_of_outputs: 0,
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
        selected_commitments: vec!["0".to_string()],
        ..Default::default()
    };
    let mut quote = context.handler.quote(quote_options.clone()).await.unwrap();
    assert_eq!(quote, expected_quote);

    quote_options.amount = Some(7.0);
    expected_quote.num_of_inputs = 2;
    expected_quote.selected_commitments = vec!["0".to_string(), "4".to_string()];
    quote = context.handler.quote(quote_options.clone()).await.unwrap();
    quote.selected_commitments.sort();
    assert_eq!(quote, expected_quote);

    quote_options.amount = Some(12.0);
    expected_quote.num_of_inputs = 2;
    expected_quote.num_of_outputs = 1;
    expected_quote.selected_commitments = vec!["4".to_string(), "8".to_string()];
    quote = context.handler.quote(quote_options.clone()).await.unwrap();
    quote.selected_commitments.sort();
    assert_eq!(quote, expected_quote);
}

#[tokio::test]
async fn test_quote_transfer_with_amount() {
    let test_options: QuoteTestOptions = QuoteTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x223903804Ee95e264F74C88B4F8583429524593c".to_string())
        .build();
    let context = QuoteTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;

    let mut quote_options = QuoteSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Transfer as i32)
        .amount(10.0)
        .build();
    let mut expected_quote = SpendQuote {
        valid: true,
        asset_symbol: "MTT".to_string(),
        asset_decimals: 16,
        current_balance: 17.0,
        current_decimal_balance: "170000000000000000".to_string(),
        num_of_inputs: 1,
        num_of_outputs: 1,
        min_rollup_fee: 4.0,
        min_rollup_fee_decimal: "40000000000000000".to_string(),
        rollup_fee_asset_symbol: "MTT".to_string(),
        rollup_fee_asset_decimals: 16,
        fixed_amounts: vec![7.0],
        fixed_decimal_amounts: vec!["70000000000000000".to_string()],
        amount_range: Some(AmountRange {
            min: 8.0,
            decimal_min: "80000000000000000".to_string(),
            max: 14.0,
            decimal_max: "140000000000000000".to_string(),
        }),
        selected_commitments: vec!["8".to_string()],
        ..Default::default()
    };
    let mut quote = context.handler.quote(quote_options.clone()).await.unwrap();
    assert_eq!(quote, expected_quote);

    quote_options.amount = Some(7.0);
    expected_quote.num_of_inputs = 2;
    expected_quote.selected_commitments = vec!["0".to_string(), "4".to_string()];
    quote = context.handler.quote(quote_options.clone()).await.unwrap();
    quote.selected_commitments.sort();
    assert_eq!(quote, expected_quote);

    quote_options.amount = Some(12.0);
    expected_quote.num_of_inputs = 2;
    expected_quote.num_of_outputs = 2;
    expected_quote.selected_commitments = vec!["4".to_string(), "8".to_string()];
    quote = context.handler.quote(quote_options.clone()).await.unwrap();
    quote.selected_commitments.sort();
    assert_eq!(quote, expected_quote);
}

#[tokio::test]
async fn test_quote_with_gas_relayer() {
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
                && more_options.circuit_type == CircuitType::Transaction2x2
                && !more_options.show_unavailable
        })
        .returning(move |_| Ok(gas_relayers.clone()));
    let test_options: QuoteTestOptions = QuoteTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x223903804Ee95e264F74C88B4F8583429524593c".to_string())
        .relayer_client(relayer_client)
        .build();
    let context = QuoteTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;

    let quote_options = QuoteSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Transfer as i32)
        .amount(12.0)
        .use_relayer(true)
        .build();
    let expected_quote = SpendQuote {
        valid: true,
        asset_symbol: "MTT".to_string(),
        asset_decimals: 16,
        current_balance: 17.0,
        current_decimal_balance: "170000000000000000".to_string(),
        num_of_inputs: 2,
        num_of_outputs: 2,
        min_rollup_fee: 4.0,
        min_rollup_fee_decimal: "40000000000000000".to_string(),
        rollup_fee_asset_symbol: "MTT".to_string(),
        rollup_fee_asset_decimals: 16,
        fixed_amounts: vec![7.0],
        fixed_decimal_amounts: vec!["70000000000000000".to_string()],
        amount_range: Some(AmountRange {
            min: 8.0,
            decimal_min: "80000000000000000".to_string(),
            max: 14.0,
            decimal_max: "140000000000000000".to_string(),
        }),
        selected_commitments: vec!["4".to_string(), "8".to_string()],
        max_gas_relayer_fee: Some(4.0),
        max_gas_relayer_fee_decimal: Some("40000000000000000".to_string()),
        gas_relayer_fee_asset_symbol: Some("MTT".to_string()),
        gas_relayer_fee_asset_decimals: Some(16),
        gas_relayers: vec![
            GasRelayer::builder()
                .name("test_relayer_1".to_string())
                .address("0x357c6Fd2cEE77bA5de49e0bB9d49444781A8f0cc".to_string())
                .url("https://test_relayer1.mystiko.network".to_string())
                .min_gas_fee(0.01)
                .min_gas_fee_decimal("100000000000000".to_string())
                .service_fee_of_ten_thousandth(10_u32)
                .service_fee_ratio(0.001)
                .build(),
            GasRelayer::builder()
                .name("test_relayer_2".to_string())
                .address("0x3ca553F90c156B086f60C639E957B469464C8924".to_string())
                .url("https://test_relayer2.mystiko.network".to_string())
                .min_gas_fee(0.011)
                .min_gas_fee_decimal("110000000000000".to_string())
                .service_fee_of_ten_thousandth(9_u32)
                .service_fee_ratio(0.0009)
                .build(),
        ],
        ..Default::default()
    };
    let mut quote = context.handler.quote(quote_options.clone()).await.unwrap();
    quote.selected_commitments.sort();
    quote.gas_relayers.sort_by_key(|relayer| relayer.name.clone());
    assert_eq!(quote, expected_quote);
}

#[tokio::test]
async fn test_quote_with_relayers_timeout_error() {
    let relayer_client = TimeoutRelayerClient::builder().timeout_ms(1000_u64).build();
    let test_options: QuoteTestOptions<TimeoutRelayerClient> = QuoteTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x223903804Ee95e264F74C88B4F8583429524593c".to_string())
        .relayer_client(relayer_client)
        .build();
    let context = QuoteTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;

    let quote_options = QuoteSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Transfer as i32)
        .amount(12.0)
        .use_relayer(true)
        .query_timeout_ms(1_u64)
        .build();
    let expected_quote = SpendQuote {
        valid: true,
        asset_symbol: "MTT".to_string(),
        asset_decimals: 16,
        current_balance: 17.0,
        current_decimal_balance: "170000000000000000".to_string(),
        num_of_inputs: 2,
        num_of_outputs: 2,
        min_rollup_fee: 4.0,
        min_rollup_fee_decimal: "40000000000000000".to_string(),
        rollup_fee_asset_symbol: "MTT".to_string(),
        rollup_fee_asset_decimals: 16,
        fixed_amounts: vec![7.0],
        fixed_decimal_amounts: vec!["70000000000000000".to_string()],
        amount_range: Some(AmountRange {
            min: 8.0,
            decimal_min: "80000000000000000".to_string(),
            max: 14.0,
            decimal_max: "140000000000000000".to_string(),
        }),
        selected_commitments: vec!["4".to_string(), "8".to_string()],
        max_gas_relayer_fee: Some(4.0),
        max_gas_relayer_fee_decimal: Some("40000000000000000".to_string()),
        gas_relayer_fee_asset_symbol: Some("MTT".to_string()),
        gas_relayer_fee_asset_decimals: Some(16),
        ..Default::default()
    };
    let mut quote = context.handler.quote(quote_options.clone()).await.unwrap();
    quote.selected_commitments.sort();
    assert_eq!(quote, expected_quote);
}

#[tokio::test]
async fn test_quote_with_disabled_contract() {
    let test_options: QuoteTestOptions = QuoteTestOptions::builder()
        .chain_id(5_u64)
        .contract_address("0x00b73dbC8C370CA7e5F00b778280596383b62929".to_string())
        .build();
    let context = QuoteTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;

    let quote_options = QuoteSpendOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .version(1)
        .spend_type(SpendType::Transfer as i32)
        .amount(7.0)
        .build();
    let quote = context.handler.quote(quote_options.clone()).await.unwrap();
    assert!(!quote.valid);
    assert_eq!(quote.invalid_code.unwrap(), SpendInvalidCode::SplitDisabled as i32);
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
struct QuoteTestOptions<R: Default = MockRelayerClient> {
    chain_id: u64,
    contract_address: String,
    #[builder(default)]
    min_rollup_fee: Option<f64>,
    #[builder(default)]
    min_rollup_fee_raise_error: bool,
    #[builder(default)]
    relayer_client: R,
}

struct QuoteTestContext<R = MockRelayerClient> {
    db: Arc<DatabaseType>,
    handler: SpendsType<R>,
    account: Account,
    chain_id: u64,
    contract_config: PoolContractConfig,
}

impl<R> QuoteTestContext<R>
where
    R: RelayerClient + Default,
    SpendsError: From<R::Error>,
{
    async fn new(options: QuoteTestOptions<R>) -> Self {
        let QuoteTestOptions {
            chain_id,
            contract_address,
            min_rollup_fee,
            min_rollup_fee_raise_error,
            relayer_client,
        } = options;
        let config = MystikoConfig::from_json_file("tests/files/mystiko/config.json")
            .await
            .unwrap();
        let contract_config = config
            .find_pool_contract_by_address(chain_id, &contract_address)
            .unwrap()
            .clone();
        let min_rollup_fee = if let Some(min_rollup_fee) = min_rollup_fee {
            number_to_u256_decimal(min_rollup_fee, Some(contract_config.asset_decimals())).unwrap()
        } else {
            biguint_to_u256(&contract_config.min_rollup_fee().unwrap())
        };
        let contract_ethereum_address = ethers_address_from_string(&contract_address).unwrap();
        let mut commitment_pool_contracts = MockCommitmentPoolContracts::new();
        if min_rollup_fee_raise_error {
            commitment_pool_contracts
                .expect_min_rollup_fee()
                .withf(move |options| {
                    options.chain_id == chain_id && options.contract_address == contract_ethereum_address
                })
                .returning(|_| Err(anyhow::anyhow!("min_rollup_fee error")));
        } else {
            commitment_pool_contracts
                .expect_min_rollup_fee()
                .withf(move |options| {
                    options.chain_id == chain_id && options.contract_address == contract_ethereum_address
                })
                .returning(move |_| Ok(min_rollup_fee));
        }

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
            .flat_map(|amount| {
                vec![
                    CommitmentOptions::builder()
                        .status(CommitmentStatus::Included)
                        .shielded_address(self.account.shielded_address.clone())
                        .amount(
                            number_to_biguint_decimal(*amount, Some(self.contract_config.asset_decimals())).unwrap(),
                        )
                        .build(),
                    CommitmentOptions::builder()
                        .status(CommitmentStatus::Queued)
                        .shielded_address(self.account.shielded_address.clone())
                        .amount(
                            number_to_biguint_decimal(*amount, Some(self.contract_config.asset_decimals())).unwrap(),
                        )
                        .build(),
                    CommitmentOptions::builder()
                        .status(CommitmentStatus::Included)
                        .shielded_address(generate_shielded_address())
                        .amount(
                            number_to_biguint_decimal(*amount, Some(self.contract_config.asset_decimals())).unwrap(),
                        )
                        .build(),
                    CommitmentOptions::builder()
                        .status(CommitmentStatus::Included)
                        .shielded_address(self.account.shielded_address.clone())
                        .spent(true)
                        .amount(
                            number_to_biguint_decimal(*amount, Some(self.contract_config.asset_decimals())).unwrap(),
                        )
                        .build(),
                ]
            })
            .collect::<Vec<_>>();
        generate_commitments(self.db.clone(), self.chain_id, &self.contract_config, &options).await
    }
}

fn generate_shielded_address() -> String {
    let sk_verify = generate_secret_key().unwrap();
    let sk_enc = generate_secret_key().unwrap();
    let pk_verify = verification_public_key(&sk_verify).unwrap();
    let pk_enc = encryption_public_key(&sk_enc).unwrap();
    let shielded_address = ShieldedAddress::from_public_key(&pk_verify, &pk_enc);
    shielded_address.address()
}
