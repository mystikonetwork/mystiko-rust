use mystiko_protos::core::handler::v1::{DepositQuote, DepositSummary};
use num_bigint::BigUint;
use std::collections::HashMap;

#[test]
fn test_deposit_quote_min_decimal_amount_as_biguint() {
    let quote = DepositQuote::builder().min_decimal_amount("10000".to_string()).build();
    assert_eq!(quote.min_decimal_amount_as_biguint().unwrap(), BigUint::from(10000_u64));
}

#[test]
fn test_deposit_quote_max_decimal_amount_as_biguint() {
    let quote = DepositQuote::builder().max_decimal_amount("10000".to_string()).build();
    assert_eq!(quote.max_decimal_amount_as_biguint().unwrap(), BigUint::from(10000_u64));
}

#[test]
fn test_deposit_quote_recommend_decimal_amounts_as_biguint() {
    let quote = DepositQuote::builder()
        .recommended_decimal_amounts(vec!["10000".to_string(), "20000".to_string()])
        .build();
    assert_eq!(
        quote.recommended_decimal_amounts_as_biguint().unwrap(),
        vec![BigUint::from(10000_u64), BigUint::from(20000_u64)]
    );
}

#[test]
fn test_deposit_quote_min_rollup_fee_decimal_amount_as_biguint() {
    let quote = DepositQuote::builder()
        .min_rollup_fee_decimal_amount("10000".to_string())
        .build();
    assert_eq!(
        quote.min_rollup_fee_decimal_amount_as_biguint().unwrap(),
        BigUint::from(10000_u64)
    );
}

#[test]
fn test_deposit_quote_min_bridge_fee_decimal_amount_as_biguint() {
    let quote = DepositQuote::builder()
        .min_bridge_fee_decimal_amount("10000".to_string())
        .build();
    assert_eq!(
        quote.min_bridge_fee_decimal_amount_as_biguint().unwrap(),
        Some(BigUint::from(10000_u64))
    );
}

#[test]
fn test_deposit_quote_min_executor_fee_decimal_amount_as_biguint() {
    let quote = DepositQuote::builder()
        .min_executor_fee_decimal_amount("10000".to_string())
        .build();
    assert_eq!(
        quote.min_executor_fee_decimal_amount_as_biguint().unwrap(),
        Some(BigUint::from(10000_u64))
    );
}

#[test]
fn test_deposit_summary_decimal_amount_as_biguint() {
    let summary = DepositSummary::builder().decimal_amount("10000".to_string()).build();
    assert_eq!(summary.decimal_amount_as_biguint().unwrap(), BigUint::from(10000_u64));
}

#[test]
fn test_deposit_summary_rollup_fee_decimal_amount_as_biguint() {
    let summary = DepositSummary::builder()
        .rollup_fee_decimal_amount("10000".to_string())
        .build();
    assert_eq!(
        summary.rollup_fee_decimal_amount_as_biguint().unwrap(),
        BigUint::from(10000_u64)
    );
}

#[test]
fn test_deposit_summary_bridge_fee_decimal_amount_as_biguint() {
    let summary = DepositSummary::builder()
        .bridge_fee_decimal_amount("10000".to_string())
        .build();
    assert_eq!(
        summary.bridge_fee_decimal_amount_as_biguint().unwrap(),
        Some(BigUint::from(10000_u64))
    );
}

#[test]
fn test_deposit_summary_executor_fee_decimal_amount_as_biguint() {
    let summary = DepositSummary::builder()
        .executor_fee_decimal_amount("10000".to_string())
        .build();
    assert_eq!(
        summary.executor_fee_decimal_amount_as_biguint().unwrap(),
        Some(BigUint::from(10000_u64))
    );
}

#[test]
fn test_deposit_total_decimal_amounts_as_biguint() {
    let summary = DepositSummary::builder()
        .total_decimal_amounts(HashMap::from([
            ("ETH".to_string(), "10000".to_string()),
            ("DAI".to_string(), "20000".to_string()),
        ]))
        .build();
    let expected = HashMap::from([
        ("ETH".to_string(), BigUint::from(10000_u64)),
        ("DAI".to_string(), BigUint::from(20000_u64)),
    ]);
    assert_eq!(summary.total_decimal_amounts_as_biguint().unwrap(), expected);
}
