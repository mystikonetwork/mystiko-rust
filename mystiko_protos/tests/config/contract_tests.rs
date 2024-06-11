use mystiko_protos::common::v1::{BridgeType, ContractType};
use mystiko_protos::config::contract::v1::{ContractConfig, DepositContractConfig, PoolContractConfig};

#[tokio::test]
async fn test_deposit_contract_config_to_proto() {
    let mystiko_config = mystiko_config::MystikoConfig::from_json_file("tests/files/config/mystiko.json")
        .await
        .unwrap();
    let deposit_contract_config: DepositContractConfig = mystiko_config
        .find_deposit_contract_by_address(5, "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67")
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(deposit_contract_config.version, 2);
    assert_eq!(&deposit_contract_config.name, "MystikoV2WithTBridgeERC20");
    assert_eq!(
        &deposit_contract_config.address,
        "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
    );
    assert_eq!(deposit_contract_config.contract_type(), ContractType::Deposit);
    assert_eq!(deposit_contract_config.start_block, 1000000);
    assert!(deposit_contract_config.disabled);
    assert_eq!(&deposit_contract_config.min_amount, "10000000000000000");
    assert_eq!(&deposit_contract_config.max_amount, "10000000000000000000");
    assert!(deposit_contract_config.pool_contract_config.is_some());
    assert_eq!(deposit_contract_config.bridge_type(), BridgeType::Tbridge);
    assert_eq!(deposit_contract_config.min_bridge_fee(), "20000000000000000");
    assert_eq!(deposit_contract_config.min_executor_fee(), "30000000000000000");
    assert_eq!(deposit_contract_config.service_fee(), 2);
    assert_eq!(deposit_contract_config.service_fee_divider(), 1000);
    assert!(deposit_contract_config.bridge_fee_asset_config.is_some());
    assert!(deposit_contract_config.executor_fee_asset_config.is_some());
    assert_eq!(deposit_contract_config.peer_chain_id(), 97);
    assert_eq!(
        deposit_contract_config.peer_contract_address(),
        "0xd791049D0a154bC7860804e1A18ACD148Eb0afD9"
    );
}

#[tokio::test]
async fn test_deposit_contract_config_to_proto_contract_config() {
    let mystiko_config = mystiko_config::MystikoConfig::from_json_file("tests/files/config/mystiko.json")
        .await
        .unwrap();
    let contract_config: ContractConfig = (&mystiko_config
        .find_contract_by_address(5, "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67")
        .unwrap())
        .try_into()
        .unwrap();
    assert_eq!(contract_config.version, 2);
    assert_eq!(&contract_config.name, "MystikoV2WithTBridgeERC20");
    assert_eq!(&contract_config.address, "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67");
    assert_eq!(contract_config.start_block, 1000000);
    assert!(contract_config.disabled);
    assert_eq!(contract_config.min_rollup_fee, "40000000000000000");
    assert!(contract_config.asset_config.is_some());
    assert_eq!(contract_config.bridge_type(), BridgeType::Tbridge);
    assert_eq!(contract_config.contract_type(), ContractType::Deposit);
}

#[tokio::test]
async fn test_pool_contract_config_to_proto() {
    let mystiko_config = mystiko_config::MystikoConfig::from_json_file("tests/files/config/mystiko.json")
        .await
        .unwrap();
    let pool_contract_config: PoolContractConfig = mystiko_config
        .find_pool_contract_by_address(5, "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d")
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(pool_contract_config.version, 2);
    assert_eq!(pool_contract_config.name, "CommitmentPool");
    assert_eq!(
        &pool_contract_config.address,
        "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
    );
    assert_eq!(pool_contract_config.start_block, 1000000);
    assert_eq!(&pool_contract_config.pool_name, "A Pool(since 07/20/2022)");
    assert_eq!(&pool_contract_config.min_rollup_fee, "40000000000000000");
    assert_eq!(pool_contract_config.contract_type(), ContractType::Pool);
    assert_eq!(pool_contract_config.bridge_type(), BridgeType::Tbridge);
    assert!(pool_contract_config.asset_config.is_some());
    assert_eq!(pool_contract_config.circuit_configs.len(), 17);
    assert_eq!(
        pool_contract_config.circuits,
        vec![String::from("zokrates-2.0-rollup1")]
    );
}

#[tokio::test]
async fn test_pool_contract_config_to_proto_contract_config() {
    let mystiko_config = mystiko_config::MystikoConfig::from_json_file("tests/files/config/mystiko.json")
        .await
        .unwrap();
    let contract_config: ContractConfig = (&mystiko_config
        .find_contract_by_address(5, "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d")
        .unwrap())
        .try_into()
        .unwrap();
    assert_eq!(contract_config.version, 2);
    assert_eq!(contract_config.name, "CommitmentPool");
    assert_eq!(&contract_config.address, "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d");
    assert_eq!(contract_config.start_block, 1000000);
    assert!(!contract_config.disabled);
    assert_eq!(&contract_config.min_rollup_fee, "40000000000000000");
    assert!(contract_config.asset_config.is_some());
    assert_eq!(contract_config.contract_type(), ContractType::Pool);
    assert_eq!(contract_config.bridge_type(), BridgeType::Tbridge);
}
