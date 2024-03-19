use crate::{extract_data, setup, DEPOSIT_CONFIG_FILE};
use mystiko_core::DepositColumn;
use mystiko_lib::deposit::{
    count, count_all, create, delete, delete_all, delete_batch, delete_by_filter, find, find_all, find_by_id, find_one,
    quote, summary, update, update_all, update_batch, update_by_filter,
};
use mystiko_lib::{account, wallet};
use mystiko_protos::api::handler::v1::{
    ColumnValuePair, CountDepositRequest, CountDepositResponse, CreateAccountRequest, CreateAccountResponse,
    CreateDepositRequest, CreateDepositResponse, CreateWalletRequest, CreateWalletResponse, DeleteDepositBatchRequest,
    DeleteDepositByFilterRequest, DeleteDepositRequest, FindDepositByIdRequest, FindDepositRequest,
    FindDepositResponse, FindOneDepositResponse, QuoteRequest, QuoteResponse, SummaryRequest, SummaryResponse,
    UpdateAllDepositRequest, UpdateDepositBatchRequest, UpdateDepositBatchResponse, UpdateDepositByFilterRequest,
    UpdateDepositRequest, UpdateDepositResponse,
};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::document::v1::{Account, Deposit, Wallet};
use mystiko_protos::core::handler::v1::{
    CreateAccountOptions, CreateDepositOptions, CreateWalletOptions, DepositQuote, QuoteDepositOptions,
};
use mystiko_protos::core::v1::DepositStatus;
use mystiko_protos::storage::v1::{ColumnValue, QueryFilter, SubFilter};
use mystiko_storage::DocumentColumn;
use serial_test::serial;
use std::collections::HashMap;

#[test]
#[serial]
#[ignore]
fn test_quote() {
    setup(Some(DEPOSIT_CONFIG_FILE.to_string()));
    let quote_options = QuoteDepositOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .build();
    let response = quote(QuoteRequest::builder().options(quote_options).build());
    assert!(response.code.unwrap().success);
    let quote = QuoteResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .quote
        .unwrap();
    assert_eq!(quote.asset_symbol, "MTT");
    assert_eq!(quote.asset_decimals, 16_u32);
    assert_eq!(quote.min_amount, 1_f64);
    assert_eq!(quote.min_decimal_amount, "10000000000000000");
    assert_eq!(quote.max_amount, 1000_f64);
    assert_eq!(quote.max_decimal_amount, "10000000000000000000");
    assert_eq!(quote.min_rollup_fee_amount, 4.0_f64);
    assert_eq!(quote.min_rollup_fee_decimal_amount, "40000000000000000");
    assert_eq!(quote.rollup_fee_asset_symbol, "MTT");
    assert_eq!(quote.rollup_fee_asset_decimals, 16_u32);
    assert!(quote.min_bridge_fee_amount.is_none());
    assert!(quote.min_bridge_fee_decimal_amount.is_none());
    assert!(quote.bridge_fee_asset_symbol.is_none());
    assert!(quote.bridge_fee_asset_decimals.is_none());
    assert!(quote.min_executor_fee_amount.is_none());
    assert!(quote.min_executor_fee_decimal_amount.is_none());
    assert!(quote.executor_fee_asset_symbol.is_none());
    assert!(quote.executor_fee_asset_decimals.is_none());
    assert_eq!(quote.recommended_amounts, [1_f64, 10_f64]);
    assert_eq!(
        quote.recommended_decimal_amounts,
        ["10000000000000000", "100000000000000000"]
    );
}

#[test]
#[serial]
fn test_summary() {
    setup(Some(DEPOSIT_CONFIG_FILE.to_string()));
    let quote = DepositQuote::builder()
        .asset_decimals(16_u32)
        .min_amount(1_f64)
        .min_decimal_amount("10000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("100000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("100000000000000".to_string())
        .rollup_fee_asset_symbol("MTT".to_string())
        .rollup_fee_asset_decimals(16_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .shielded_address("secret address".to_string())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .deposit_quote(quote)
        .build();
    let response = summary(SummaryRequest::builder().options(options).build());
    assert!(response.code.unwrap().success);
    let summary = SummaryResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .summary
        .unwrap();
    assert_eq!(summary.chain_id, 5_u64);
    assert_eq!(summary.asset_symbol, "MTT");
    assert_eq!(summary.asset_decimals, 16_u32);
    assert_eq!(summary.shielded_address, "secret address");
    assert_eq!(summary.amount, 10_f64);
    assert_eq!(summary.decimal_amount, "100000000000000000");
    assert_eq!(summary.rollup_fee_amount, 0.01_f64);
    assert_eq!(summary.rollup_fee_decimal_amount, "100000000000000");
    assert_eq!(summary.rollup_fee_asset_symbol, "MTT");
    assert_eq!(summary.rollup_fee_asset_decimals, 16_u32);
    assert_eq!(summary.bridge_type.unwrap(), BridgeType::Loop as i32);
    assert_eq!(summary.total_amounts, HashMap::from([("MTT".to_string(), 10.01_f64)]));
    assert_eq!(
        summary.total_decimal_amounts,
        HashMap::from([("MTT".to_string(), "100100000000000000".to_string())])
    );
    assert!(summary.dst_chain_id.is_none());
    assert!(summary.bridge_fee_amount.is_none());
    assert!(summary.bridge_fee_decimal_amount.is_none());
    assert!(summary.bridge_fee_asset_symbol.is_none());
    assert!(summary.bridge_fee_asset_decimals.is_none());
    assert!(summary.executor_fee_amount.is_none());
    assert!(summary.executor_fee_decimal_amount.is_none());
    assert!(summary.executor_fee_asset_symbol.is_none());
    assert!(summary.executor_fee_asset_decimals.is_none());
}

#[test]
#[serial]
fn test_create() {
    setup(Some(DEPOSIT_CONFIG_FILE.to_string()));
    let (wallet, account) = create_wallet();
    let quote = DepositQuote::builder()
        .asset_decimals(18_u32)
        .min_amount(1_f64)
        .min_decimal_amount("1000000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("10000000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("10000000000000000".to_string())
        .rollup_fee_asset_symbol("BNB".to_string())
        .rollup_fee_asset_decimals(18_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(97_u64)
        .asset_symbol("BNB".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .shielded_address(account.shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .deposit_quote(quote)
        .build();
    let response = create(CreateDepositRequest::builder().options(options).build());
    assert!(response.code.unwrap().success);
    let deposit = CreateDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .deposit
        .unwrap();
    assert_eq!(deposit.chain_id, 97_u64);
    assert_eq!(deposit.contract_address, "0x390d485F4D43212D4ae8Cdd967a711514ed5a54f");
    assert_eq!(deposit.pool_address, "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
    assert_eq!(deposit.dst_chain_id, 97_u64);
    assert_eq!(
        deposit.dst_chain_contract_address,
        "0x390d485F4D43212D4ae8Cdd967a711514ed5a54f"
    );
    assert_eq!(deposit.dst_pool_address, "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
    assert_eq!(deposit.asset_symbol, "BNB");
    assert_eq!(deposit.asset_decimals, 18_u32);
    assert_eq!(deposit.amount, 10_f64);
    assert_eq!(deposit.decimal_amount, "10000000000000000000");
    assert_eq!(deposit.rollup_fee_amount, 0.01_f64);
    assert_eq!(deposit.rollup_fee_decimal_amount, "10000000000000000");
    assert_eq!(deposit.shielded_address, account.shielded_address);
    assert_eq!(deposit.wallet_id, wallet.id);
    assert_eq!(deposit.status, DepositStatus::Unspecified as i32);
    assert_eq!(deposit.bridge_type, BridgeType::Loop as i32);
    assert!(deposit.asset_address.is_none());
    assert!(deposit.bridge_fee_amount.is_none());
    assert!(deposit.bridge_fee_decimal_amount.is_none());
    assert!(deposit.bridge_fee_asset_symbol.is_none());
    assert!(deposit.bridge_fee_asset_address.is_none());
    assert!(deposit.bridge_fee_asset_decimals.is_none());
    assert!(deposit.executor_fee_amount.is_none());
    assert!(deposit.executor_fee_decimal_amount.is_none());
    assert!(deposit.executor_fee_asset_symbol.is_none());
    assert!(deposit.executor_fee_asset_address.is_none());
    assert!(deposit.executor_fee_asset_decimals.is_none());
    assert!(deposit.src_chain_transaction_hash.is_none());
    assert!(deposit.queued_transaction_hash.is_none());
    assert!(deposit.included_transaction_hash.is_none());
    assert!(deposit.error_message.is_none());
}

#[test]
#[serial]
fn test_crud() {
    setup(Some(DEPOSIT_CONFIG_FILE.to_string()));
    let (_, account) = create_wallet();
    let discarded_deposits = generate_deposits(account.shielded_address);
    let (_, account) = create_wallet();
    let mut deposits = generate_deposits(account.shielded_address);
    deposits.sort_by_key(|deposit| deposit.id.clone());

    let response = find_all();
    assert!(response.code.unwrap().success);
    let mut found_deposits = FindDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .deposits;
    found_deposits.sort_by_key(|deposit| deposit.id.clone());
    assert_eq!(found_deposits, deposits);

    let response = find(
        FindDepositRequest::builder()
            .filter(QueryFilter::from(SubFilter::equal(
                DocumentColumn::Id,
                deposits[0].id.clone(),
            )))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let found_deposits = FindDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .deposits;
    assert_eq!(found_deposits.len(), 1);
    assert_eq!(found_deposits[0], deposits[0]);

    let response = find(
        FindDepositRequest::builder()
            .filter(QueryFilter::from(SubFilter::equal(
                DocumentColumn::Id,
                discarded_deposits[0].id.clone(),
            )))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let found_deposits = FindDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .deposits;
    assert_eq!(found_deposits.len(), 0);

    let response = find_one(
        FindDepositRequest::builder()
            .filter(QueryFilter::from(SubFilter::equal(
                DocumentColumn::Id,
                deposits[0].id.clone(),
            )))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let found_deposit = FindOneDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .deposit;
    assert_eq!(found_deposit.unwrap(), deposits[0]);

    let response = find_one(
        FindDepositRequest::builder()
            .filter(QueryFilter::from(SubFilter::equal(
                DocumentColumn::Id,
                discarded_deposits[0].id.clone(),
            )))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let found_deposit = FindOneDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .deposit;
    assert!(found_deposit.is_none());

    let response = find_by_id(FindDepositByIdRequest::builder().id(deposits[0].id.clone()).build());
    assert!(response.code.unwrap().success);
    let found_deposit = FindOneDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .deposit;
    assert_eq!(found_deposit.unwrap(), deposits[0]);

    let response = count_all();
    assert!(response.code.unwrap().success);
    let c = CountDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .count;
    assert_eq!(c, 4);

    let response = count(
        CountDepositRequest::builder()
            .filter(QueryFilter::from(SubFilter::in_list(
                DocumentColumn::Id,
                vec![deposits[0].id.clone(), deposits[1].id.clone()],
            )))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let c = CountDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .count;
    assert_eq!(c, 2);

    let response = count(
        CountDepositRequest::builder()
            .filter(QueryFilter::from(SubFilter::in_list(
                DocumentColumn::Id,
                vec![discarded_deposits[0].id.clone(), discarded_deposits[1].id.clone()],
            )))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let c = CountDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .count;
    assert_eq!(c, 0);

    deposits[0].status = DepositStatus::Queued as i32;
    let response = update(UpdateDepositRequest::builder().deposit(deposits[0].clone()).build());
    assert!(response.code.unwrap().success);
    let updated_deposit = UpdateDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .deposit;
    assert_eq!(updated_deposit.unwrap().status, DepositStatus::Queued as i32);

    deposits[1].status = DepositStatus::SrcPending as i32;
    deposits[2].status = DepositStatus::SrcSucceeded as i32;
    let response = update_batch(
        UpdateDepositBatchRequest::builder()
            .deposits(vec![deposits[1].clone(), deposits[2].clone()])
            .build(),
    );
    assert!(response.code.unwrap().success);
    let mut updated_deposits = UpdateDepositBatchResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .deposits;
    updated_deposits.sort_by_key(|deposit| deposit.id.clone());
    assert_eq!(updated_deposits[0].status, DepositStatus::SrcPending as i32);
    assert_eq!(updated_deposits[1].status, DepositStatus::SrcSucceeded as i32);

    let response = update_by_filter(
        UpdateDepositByFilterRequest::builder()
            .column_values(vec![ColumnValuePair::builder()
                .column(DepositColumn::Status.to_string())
                .value(ColumnValue::from(DepositStatus::Failed as i32))
                .build()])
            .filter(QueryFilter::from(SubFilter::equal(
                DocumentColumn::Id,
                deposits[3].id.clone(),
            )))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let response = find_by_id(FindDepositByIdRequest::builder().id(deposits[3].id.clone()).build());
    assert!(response.code.unwrap().success);
    let found_deposit = FindOneDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .deposit;
    assert_eq!(found_deposit.unwrap().status, DepositStatus::Failed as i32);

    let response = update_by_filter(
        UpdateDepositByFilterRequest::builder()
            .column_values(vec![ColumnValuePair::builder()
                .column(DepositColumn::Status.to_string())
                .value(ColumnValue::from(DepositStatus::Failed as i32))
                .build()])
            .filter(QueryFilter::from(SubFilter::equal(
                DocumentColumn::Id,
                discarded_deposits[3].id.clone(),
            )))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let response = find_by_id(
        FindDepositByIdRequest::builder()
            .id(discarded_deposits[3].id.clone())
            .build(),
    );
    assert!(response.code.unwrap().success);
    let found_deposit = FindOneDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .deposit;
    assert_eq!(found_deposit.unwrap().status, DepositStatus::Unspecified as i32);

    let response = update_all(
        UpdateAllDepositRequest::builder()
            .column_values(vec![ColumnValuePair::builder()
                .column(DepositColumn::Status.to_string())
                .value(ColumnValue::from(DepositStatus::Failed as i32))
                .build()])
            .build(),
    );
    assert!(response.code.unwrap().success);
    let response = find_all();
    assert!(response.code.unwrap().success);
    let found_deposits = FindDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .deposits;
    found_deposits.iter().for_each(|deposit| {
        assert_eq!(deposit.status, DepositStatus::Failed as i32);
    });
    discarded_deposits.iter().for_each(|deposit| {
        assert_eq!(deposit.status, DepositStatus::Unspecified as i32);
    });

    let response = delete(DeleteDepositRequest::builder().deposit(deposits[0].clone()).build());
    assert!(response.code.unwrap().success);
    let response = count_all();
    assert!(response.code.unwrap().success);
    let c = CountDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .count;
    assert_eq!(c, 3);

    let response = delete_batch(
        DeleteDepositBatchRequest::builder()
            .deposits(vec![deposits[1].clone()])
            .build(),
    );
    assert!(response.code.unwrap().success);
    let response = count_all();
    assert!(response.code.unwrap().success);
    let c = CountDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .count;
    assert_eq!(c, 2);

    let response = delete_by_filter(
        DeleteDepositByFilterRequest::builder()
            .filter(QueryFilter::from(SubFilter::equal(
                DocumentColumn::Id,
                deposits[2].id.clone(),
            )))
            .build(),
    );
    assert!(response.code.unwrap().success);
    let response = count_all();
    assert!(response.code.unwrap().success);
    let c = CountDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .count;
    assert_eq!(c, 1);

    let response = delete_all();
    assert!(response.code.unwrap().success);
    let response = count_all();
    assert!(response.code.unwrap().success);
    let c = CountDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .count;
    assert_eq!(c, 0);
}

fn create_wallet() -> (Wallet, Account) {
    let options = CreateWalletOptions::builder().password("P@ssw0rd".to_string()).build();
    let response = wallet::create(CreateWalletRequest::builder().options(options).build());
    assert!(response.code.unwrap().success);
    let wallet = CreateWalletResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .wallet
        .unwrap();
    // create account
    let options = CreateAccountOptions::builder()
        .wallet_password("P@ssw0rd".to_string())
        .build();
    let response = account::create(CreateAccountRequest::builder().options(options).build());
    assert!(response.code.unwrap().success);
    let account = CreateAccountResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .account
        .unwrap();
    (wallet, account)
}

fn generate_deposits(shielded_address: String) -> Vec<Deposit> {
    let mut deposits = vec![];
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(18_u32)
        .min_amount(1_f64)
        .min_decimal_amount("1000000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("10000000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("10000000000000000".to_string())
        .rollup_fee_asset_symbol("BNB".to_string())
        .rollup_fee_asset_decimals(18_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(97_u64)
        .asset_symbol("BNB".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .shielded_address(shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .deposit_quote(quote)
        .build();
    let response = create(CreateDepositRequest::builder().options(options).build());
    assert!(response.code.unwrap().success);
    let deposit = CreateDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .deposit
        .unwrap();
    deposits.push(deposit);

    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(16_u32)
        .min_amount(1_f64)
        .min_decimal_amount("10000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("100000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("100000000000000".to_string())
        .rollup_fee_asset_symbol("MTT".to_string())
        .rollup_fee_asset_decimals(16_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .shielded_address(shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .deposit_quote(quote)
        .build();
    let response = create(CreateDepositRequest::builder().options(options).build());
    assert!(response.code.unwrap().success);
    let deposit = CreateDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .deposit
        .unwrap();
    deposits.push(deposit);

    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(18_u32)
        .min_amount(1_f64)
        .min_decimal_amount("1000000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("10000000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("10000000000000000".to_string())
        .rollup_fee_asset_symbol("BNB".to_string())
        .rollup_fee_asset_decimals(18_u32)
        .min_bridge_fee_amount(0.00001_f64)
        .min_bridge_fee_decimal_amount("10000000000000".to_string())
        .bridge_fee_asset_symbol("BNB".to_string())
        .bridge_fee_asset_decimals(18_u32)
        .min_executor_fee_amount(0.0001_f64)
        .min_executor_fee_decimal_amount("100000000000000".to_string())
        .executor_fee_asset_symbol("BNB".to_string())
        .executor_fee_asset_decimals(18_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(97_u64)
        .dst_chain_id(5_u64)
        .asset_symbol("BNB".to_string())
        .bridge_type(BridgeType::Tbridge as i32)
        .shielded_address(shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .bridge_fee_amount(0.00001_f64)
        .executor_fee_amount(0.0001_f64)
        .deposit_quote(quote)
        .build();
    let response = create(CreateDepositRequest::builder().options(options).build());
    assert!(response.code.unwrap().success);
    let deposit = CreateDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .deposit
        .unwrap();
    deposits.push(deposit);

    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(18_u32)
        .min_amount(1_f64)
        .min_decimal_amount("1000000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("10000000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("10000000000000000".to_string())
        .rollup_fee_asset_symbol("mBNB".to_string())
        .rollup_fee_asset_decimals(18_u32)
        .min_bridge_fee_amount(0.00001_f64)
        .min_bridge_fee_decimal_amount("10000000000000".to_string())
        .bridge_fee_asset_symbol("mBNB".to_string())
        .bridge_fee_asset_decimals(18_u32)
        .min_executor_fee_amount(0.0001_f64)
        .min_executor_fee_decimal_amount("100000000000000".to_string())
        .executor_fee_asset_symbol("mBNB".to_string())
        .executor_fee_asset_decimals(18_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(5_u64)
        .dst_chain_id(97_u64)
        .asset_symbol("mBNB".to_string())
        .bridge_type(BridgeType::Tbridge as i32)
        .shielded_address(shielded_address)
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .bridge_fee_amount(0.00001_f64)
        .executor_fee_amount(0.0001_f64)
        .deposit_quote(quote)
        .build();
    let response = create(CreateDepositRequest::builder().options(options).build());
    assert!(response.code.unwrap().success);
    let deposit = CreateDepositResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .deposit
        .unwrap();
    deposits.push(deposit);

    deposits
}
