use ethers_core::types::U256;
use mystiko_server_utils::token_price::utils::calc_token_precision;

#[tokio::test]
async fn test_calc_token_precision() {
    let p1 = calc_token_precision(100.0, 6, 4);
    assert_eq!(p1, U256::from(1));

    let p1 = calc_token_precision(10.0, 6, 4);
    assert_eq!(p1, U256::from(1));

    let p1 = calc_token_precision(1.0, 6, 4);
    assert_eq!(p1, U256::from(10));

    let p1 = calc_token_precision(0.1, 6, 4);
    assert_eq!(p1, U256::from(100));

    let p1 = calc_token_precision(0.01, 6, 4);
    assert_eq!(p1, U256::from(100));

    let p1 = calc_token_precision(0.001, 6, 4);
    assert_eq!(p1, U256::from(100));

    let p1 = calc_token_precision(0.0001, 6, 4);
    assert_eq!(p1, U256::from(100));

    let p1 = calc_token_precision(100.0, 6, 10);
    assert_eq!(p1, U256::from(1));

    let p1 = calc_token_precision(10.0, 6, 10);
    assert_eq!(p1, U256::from(1));

    let p1 = calc_token_precision(1.0, 6, 10);
    assert_eq!(p1, U256::from(1));

    let p1 = calc_token_precision(0.1, 6, 10);
    assert_eq!(p1, U256::from(1));

    let p1 = calc_token_precision(0.01, 6, 10);
    assert_eq!(p1, U256::from(1));

    let p1 = calc_token_precision(0.001, 6, 10);
    assert_eq!(p1, U256::from(1));

    let p1 = calc_token_precision(0.0001, 6, 10);
    assert_eq!(p1, U256::from(1));

    let p1 = calc_token_precision(0.00001, 6, 10);
    assert_eq!(p1, U256::from(1));
}
