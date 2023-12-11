use mystiko_protos::core::handler::v1::{AmountRange, SpendQuote, SpendSummary};
use num_bigint::BigUint;

#[test]
fn test_amount_range_decimal_min_as_biguint() {
    let range = AmountRange::builder().decimal_min("1000").build();
    assert_eq!(range.decimal_min_as_biguint().unwrap(), BigUint::from(1000_u64));
}

#[test]
fn test_amount_range_decimal_max_as_biguint() {
    let range = AmountRange::builder().decimal_max("1000").build();
    assert_eq!(range.decimal_max_as_biguint().unwrap(), BigUint::from(1000_u64));
}

#[test]
fn test_spend_quote_current_decimal_balance_as_biguint() {
    let quote = SpendQuote::builder().current_decimal_balance("1000").build();
    assert_eq!(
        quote.current_decimal_balance_as_biguint().unwrap(),
        BigUint::from(1000_u64)
    );
}

#[test]
fn test_spend_quote_min_rollup_fee_decimal_as_biguint() {
    let quote = SpendQuote::builder().min_rollup_fee_decimal("1000").build();
    assert_eq!(
        quote.min_rollup_fee_decimal_as_biguint().unwrap(),
        BigUint::from(1000_u64)
    );
}

#[test]
fn test_spend_quote_max_gas_relayer_fee_decimal_as_biguint() {
    let quote = SpendQuote::builder()
        .max_gas_relayer_fee_decimal("1000".to_string())
        .build();
    assert_eq!(
        quote.max_gas_relayer_fee_decimal_as_biguint().unwrap().unwrap(),
        BigUint::from(1000_u64)
    );
}

#[test]
fn test_spend_quote_fixed_decimal_amounts_as_biguint() {
    let quote = SpendQuote::builder()
        .fixed_decimal_amounts(vec!["1000".to_string(), "2000".to_string()])
        .build();
    assert_eq!(
        quote.fixed_decimal_amounts_as_biguint().unwrap(),
        vec![BigUint::from(1000_u64), BigUint::from(2000_u64)]
    );
}

#[test]
fn test_spend_quote_selected_commitments_as_biguint() {
    let quote = SpendQuote::builder()
        .selected_commitments(vec!["1000".to_string(), "2000".to_string()])
        .build();
    assert_eq!(
        quote.selected_commitments_as_biguint().unwrap(),
        vec![BigUint::from(1000_u64), BigUint::from(2000_u64)]
    );
}

#[test]
fn test_spend_summary_current_decimal_balance_as_biguint() {
    let summary = SpendSummary::builder().current_decimal_balance("1000").build();
    assert_eq!(
        summary.current_decimal_balance_as_biguint().unwrap(),
        BigUint::from(1000_u64)
    );
}

#[test]
fn test_spend_summary_new_decimal_balance_as_biguint() {
    let summary = SpendSummary::builder().new_decimal_balance("1000").build();
    assert_eq!(
        summary.new_decimal_balance_as_biguint().unwrap(),
        BigUint::from(1000_u64)
    );
}

#[test]
fn test_spend_summary_decimal_amount_as_biguint() {
    let summary = SpendSummary::builder().decimal_amount("1000").build();
    assert_eq!(summary.decimal_amount_as_biguint().unwrap(), BigUint::from(1000_u64));
}

#[test]
fn test_spend_summary_rollup_fee_decimal_amount_as_biguint() {
    let summary = SpendSummary::builder().rollup_fee_decimal_amount("1000").build();
    assert_eq!(
        summary.rollup_fee_decimal_amount_as_biguint().unwrap(),
        BigUint::from(1000_u64)
    );
}

#[test]
fn test_spend_summary_rollup_fee_total_decimal_amount_as_biguint() {
    let summary = SpendSummary::builder()
        .rollup_fee_total_decimal_amount("1000".to_string())
        .build();
    assert_eq!(
        summary.rollup_fee_total_decimal_amount_as_biguint().unwrap(),
        BigUint::from(1000_u64)
    );
}

#[test]
fn test_spend_summary_gas_relayer_fee_decimal_amount_as_biguint() {
    let summary = SpendSummary::builder()
        .gas_relayer_fee_decimal_amount("1000".to_string())
        .build();
    assert_eq!(
        summary.gas_relayer_fee_decimal_amount_as_biguint().unwrap(),
        Some(BigUint::from(1000_u64))
    );
}
