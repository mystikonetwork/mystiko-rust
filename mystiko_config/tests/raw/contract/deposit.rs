use mystiko_config::common::{BridgeType, ContractType};
use mystiko_config::raw::base::{RawConfig, Validator};
use mystiko_config::raw::contract::base::RawContractConfig;
use mystiko_config::raw::contract::deposit::RawDepositContractConfig;

async fn default_config() -> RawDepositContractConfig {
    RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
        RawContractConfig::new(
            2,
            "MystikoWithPolyERC20".to_string(),
            "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67".to_string(),
            ContractType::Deposit,
            1000000,
            None,
            None,
        ),
        BridgeType::Tbridge,
        "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d".to_string(),
        true,
        Some(97),
        Some(String::from("0x98bF2d9e3bA2A8515E660BD4104432ce3e2D7547")),
        "10000000000000000".to_string(),
        "100000000000000000".to_string(),
        "20000000000000000".to_string(),
        "30000000000000000".to_string(),
        Some(String::from("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")),
        Some(String::from("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")),
        Some(2),
        Some(1000),
    ))
    .await
    .unwrap()
}

#[tokio::test]
async fn test_invalid_type() {
    let mut config = default_config().await;
    config.contract_type = ContractType::Pool;
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_pool_address_0() {
    let mut config = default_config().await;
    config.pool_address = String::from("0xdeadbeef");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_pool_address_1() {
    let mut config = default_config().await;
    config.pool_address = String::from("");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_peer_chain_id() {
    let mut config = default_config().await;
    config.peer_chain_id = Some(0);
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_peer_contract_address_0() {
    let mut config = default_config().await;
    config.peer_contract_address = Some(String::from(""));
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_peer_contract_address_1() {
    let mut config = default_config().await;
    config.peer_contract_address = Some(String::from("0xdeadbeef"));
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_min_amount_0() {
    let mut config = default_config().await;
    config.min_amount = String::from("");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_min_amount_1() {
    let mut config = default_config().await;
    config.min_amount = String::from("0xdeadbeef");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_min_amount_2() {
    let mut config = default_config().await;
    config.min_amount = String::from("-1");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_min_amount_3() {
    let mut config = default_config().await;
    config.min_amount = String::from("1.2");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_max_amount_0() {
    let mut config = default_config().await;
    config.max_amount = String::from("");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_max_amount_1() {
    let mut config = default_config().await;
    config.max_amount = String::from("0xdeadbeef");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_max_amount_2() {
    let mut config = default_config().await;
    config.max_amount = String::from("-1");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_max_amount_3() {
    let mut config = default_config().await;
    config.max_amount = String::from("1.2");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_min_bridge_fee_0() {
    let mut config = default_config().await;
    config.min_bridge_fee = String::from("");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_min_bridge_fee_1() {
    let mut config = default_config().await;
    config.min_bridge_fee = String::from("0xdeadbeef");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_min_bridge_fee_2() {
    let mut config = default_config().await;
    config.min_bridge_fee = String::from("-1");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_min_bridge_fee_3() {
    let mut config = default_config().await;
    config.min_bridge_fee = String::from("1.2");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_min_executor_fee_0() {
    let mut config = default_config().await;
    config.min_executor_fee = String::from("");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_min_executor_fee_1() {
    let mut config = default_config().await;
    config.min_executor_fee = String::from("0xdeadbeef");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_min_executor_fee_2() {
    let mut config = default_config().await;
    config.min_executor_fee = String::from("-1");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_min_executor_fee_3() {
    let mut config = default_config().await;
    config.min_executor_fee = String::from("1.2");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_bridge_fee_asset_address_0() {
    let mut config = default_config().await;
    config.bridge_fee_asset_address = Some(String::from("0xdeadbeef"));
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_bridge_fee_asset_address_1() {
    let mut config = default_config().await;
    config.bridge_fee_asset_address = Some(String::from(""));
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_executor_fee_asset_address_0() {
    let mut config = default_config().await;
    config.executor_fee_asset_address = Some(String::from("0xdeadbeef"));
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_executor_fee_asset_address_1() {
    let mut config = default_config().await;
    config.executor_fee_asset_address = Some(String::from(""));
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config = RawConfig::create_from_file::<RawDepositContractConfig>(
        "tests/files/contract/deposit.valid.json",
    )
    .await
    .unwrap();
    assert_eq!(file_config, default_config().await);
    assert_eq!(file_config.contract_type, file_config.base.contract_type);
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config = RawConfig::create_from_file::<RawDepositContractConfig>(
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
    let str_config = RawConfig::create_from_json_string::<RawDepositContractConfig>(json_str)
        .await
        .unwrap();
    assert_eq!(str_config.contract_type, ContractType::Deposit);
    assert_eq!(str_config.contract_type, str_config.base.contract_type);
}
