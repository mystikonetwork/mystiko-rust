use mystiko_config::raw::contract::deposit::RawDepositContractConfig;
use mystiko_config::raw::{create_raw, create_raw_from_file, create_raw_from_json};
use mystiko_config::types::{BridgeType, ContractType};
use validator::Validate;

fn default_config() -> RawDepositContractConfig {
    let raw_deposit_contract_config = RawDepositContractConfig::builder()
        .version(2)
        .name("MystikoWithPolyERC20".to_string())
        .address("0x961f315a836542e603a3df2e0dd9d4ecd06ebc67".to_string())
        .contract_type(ContractType::Deposit)
        .start_block(1000000)
        .bridge_type(BridgeType::Tbridge)
        .pool_address("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d".to_string())
        .disabled(true)
        .peer_chain_id(Some(97))
        .peer_contract_address(Some(
            "0x98bF2d9e3bA2A8515E660BD4104432ce3e2D7547".to_string(),
        ))
        .min_amount("10000000000000000".to_string())
        .max_amount("100000000000000000".to_string())
        .min_bridge_fee("20000000000000000".to_string())
        .min_executor_fee("30000000000000000".to_string())
        .bridge_fee_asset_address(Some(
            "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string(),
        ))
        .executor_fee_asset_address(Some(
            "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string(),
        ))
        .service_fee(2)
        .service_fee_divider(1000)
        .build();
    create_raw::<RawDepositContractConfig>(raw_deposit_contract_config).unwrap()
}

#[test]
fn test_default_values() {
    let raw_config = RawDepositContractConfig::builder()
        .version(2)
        .name("MystikoWithPolyERC20".to_string())
        .address("0x961f315a836542e603a3df2e0dd9d4ecd06ebc67".to_string())
        .contract_type(ContractType::Deposit)
        .start_block(1000000)
        .bridge_type(BridgeType::Tbridge)
        .pool_address("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d".to_string())
        .peer_chain_id(Some(97))
        .peer_contract_address(Some(
            "0x98bF2d9e3bA2A8515E660BD4104432ce3e2D7547".to_string(),
        ))
        .min_amount("10000000000000000".to_string())
        .max_amount("100000000000000000".to_string())
        .bridge_fee_asset_address(Some(
            "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string(),
        ))
        .executor_fee_asset_address(Some(
            "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string(),
        ))
        .service_fee(2)
        .service_fee_divider(1000)
        .build();
    assert_eq!(raw_config.disabled, false);
    assert_eq!(raw_config.min_bridge_fee, "0");
    assert_eq!(raw_config.min_executor_fee, "0");
}

#[test]
fn test_invalid_type() {
    let mut config = default_config();
    config.contract_type = ContractType::Pool;
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_pool_address_0() {
    let mut config = default_config();
    config.pool_address = String::from("0xdeadbeef");
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_pool_address_1() {
    let mut config = default_config();
    config.pool_address = String::from("");
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_peer_chain_id() {
    let mut config = default_config();
    config.peer_chain_id = Some(0);
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_peer_contract_address_0() {
    let mut config = default_config();
    config.peer_contract_address = Some(String::from(""));
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_peer_contract_address_1() {
    let mut config = default_config();
    config.peer_contract_address = Some(String::from("0xdeadbeef"));
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_min_amount_0() {
    let mut config = default_config();
    config.min_amount = String::from("");
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_min_amount_1() {
    let mut config = default_config();
    config.min_amount = String::from("0xdeadbeef");
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_min_amount_2() {
    let mut config = default_config();
    config.min_amount = String::from("-1");
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_min_amount_3() {
    let mut config = default_config();
    config.min_amount = String::from("1.2");
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_max_amount_0() {
    let mut config = default_config();
    config.max_amount = String::from("");
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_max_amount_1() {
    let mut config = default_config();
    config.max_amount = String::from("0xdeadbeef");
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_max_amount_2() {
    let mut config = default_config();
    config.max_amount = String::from("-1");
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_max_amount_3() {
    let mut config = default_config();
    config.max_amount = String::from("1.2");
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_min_bridge_fee_0() {
    let mut config = default_config();
    config.min_bridge_fee = String::from("");
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_min_bridge_fee_1() {
    let mut config = default_config();
    config.min_bridge_fee = String::from("0xdeadbeef");
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_min_bridge_fee_2() {
    let mut config = default_config();
    config.min_bridge_fee = String::from("-1");
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_min_bridge_fee_3() {
    let mut config = default_config();
    config.min_bridge_fee = String::from("1.2");
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_min_executor_fee_0() {
    let mut config = default_config();
    config.min_executor_fee = String::from("");
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_min_executor_fee_1() {
    let mut config = default_config();
    config.min_executor_fee = String::from("0xdeadbeef");
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_min_executor_fee_2() {
    let mut config = default_config();
    config.min_executor_fee = String::from("-1");
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_min_executor_fee_3() {
    let mut config = default_config();
    config.min_executor_fee = String::from("1.2");
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_bridge_fee_asset_address_0() {
    let mut config = default_config();
    config.bridge_fee_asset_address = Some(String::from("0xdeadbeef"));
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_bridge_fee_asset_address_1() {
    let mut config = default_config();
    config.bridge_fee_asset_address = Some(String::from(""));
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_executor_fee_asset_address_0() {
    let mut config = default_config();
    config.executor_fee_asset_address = Some(String::from("0xdeadbeef"));
    assert_eq!(config.validate().is_err(), true);
}

#[test]
fn test_invalid_executor_fee_asset_address_1() {
    let mut config = default_config();
    config.executor_fee_asset_address = Some(String::from(""));
    assert_eq!(config.validate().is_err(), true);
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config =
        create_raw_from_file::<RawDepositContractConfig>("tests/files/contract/deposit.valid.json")
            .await
            .unwrap();
    assert_eq!(file_config, default_config());
    assert_eq!(file_config.contract_type, ContractType::Deposit);
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config = create_raw_from_file::<RawDepositContractConfig>(
        "tests/files/contract/deposit.invalid.json",
    )
    .await;
    assert_eq!(file_config.is_err(), true);
}

#[tokio::test]
async fn test_import_valid_json_str() {
    let json_str = r#"
            {
              "version": 2,
              "name": "MystikoWithPolyERC20",
              "address": "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67",
              "startBlock": 1000000,
              "bridgeType": "tbridge",
              "poolAddress": "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d",
              "disabled": true,
              "peerChainId":  97,
              "peerContractAddress": "0x98bF2d9e3bA2A8515E660BD4104432ce3e2D7547",
              "minAmount": "10000000000000000",
              "maxAmount": "100000000000000000",
              "minBridgeFee": "20000000000000000",
              "minExecutorFee": "30000000000000000",
              "bridgeFeeAssetAddress": "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a",
              "executorFeeAssetAddress": "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a",
              "serviceFee": 2,
              "serviceFeeDivider": 1000
            }
        "#;
    let str_config = create_raw_from_json::<RawDepositContractConfig>(json_str).unwrap();
    assert_eq!(str_config.contract_type, ContractType::Deposit);
}
