use mystiko_config::raw::asset::RawAssetConfig;
use mystiko_config::raw::circuit::RawCircuitConfig;
use mystiko_config::raw::contract::pool::RawPoolContractConfig;
use mystiko_config::raw::create_raw_from_file;
use mystiko_config::types::{AssetType, BridgeType, CircuitType, ContractType};
use mystiko_config::wrapper::asset::{AssetConfig, MAIN_ASSET_ADDRESS};
use mystiko_config::wrapper::circuit::CircuitConfig;
use mystiko_config::wrapper::contract::pool::PoolContractConfig;
use num_bigint::BigInt;
use std::str::FromStr;
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/contract/pool.valid.json";

#[tokio::test]
async fn test_create() {
    let raw_config = Arc::new(
        create_raw_from_file::<RawPoolContractConfig>(VALID_CONFIG_FILE)
            .await
            .unwrap(),
    );
    let main_asset_config = Arc::new(AssetConfig::new(Arc::new(default_raw_main_asset_config())));
    let asset_config = Arc::new(AssetConfig::new(Arc::new(default_raw_asset_config(
        raw_config.asset_address.as_ref().unwrap(),
    ))));
    let circuit_configs: Vec<Arc<CircuitConfig>> = default_raw_circuit_configs()
        .into_iter()
        .map(|r| Arc::new(CircuitConfig::new(Arc::new(r))))
        .collect();
    let config = PoolContractConfig::new(
        raw_config.clone(),
        main_asset_config.clone(),
        asset_config.clone(),
        circuit_configs.clone(),
    );
    config.validate().unwrap();
    assert_eq!(config.version(), 2);
    assert_eq!(config.name(), "CommitmentPool");
    assert_eq!(config.pool_name(), "A Pool(since 07/20/2022)");
    assert_eq!(config.bridge_type(), &BridgeType::Tbridge);
    assert_eq!(
        config.address(),
        "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
    );
    assert_eq!(config.contract_type(), &ContractType::Pool);
    assert_eq!(config.start_block(), 1000000);
    assert!(config.event_filter_size().is_none());
    assert!(config.indexer_filter_size().is_none());
    assert_eq!(
        config.asset().asset_address(),
        "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"
    );
    assert_eq!(
        config.asset_address().as_ref().unwrap(),
        "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"
    );
    assert_eq!(config.asset_type(), &AssetType::Erc20);
    assert_eq!(config.asset_decimals(), 18);
    assert_eq!(config.asset_symbol(), "MTT");
    assert_eq!(
        config.recommended_amounts().unwrap(),
        vec![
            BigInt::from_str("1000000000000000000").unwrap(),
            BigInt::from_str("10000000000000000000").unwrap(),
        ]
    );
    assert_eq!(
        config.recommended_amounts_number::<u32>().unwrap(),
        vec![1, 10]
    );
    assert_eq!(
        config.min_rollup_fee().unwrap(),
        BigInt::from_str("120000000000000000").unwrap()
    );
    assert_eq!(config.min_rollup_fee_number::<f64>().unwrap(), 0.12);
    assert_eq!(config.circuits_names(), &vec![String::from("circuit-1.0")]);
    assert_eq!(config.circuits().len(), circuit_configs.len());
    assert_eq!(
        config
            .circuit_by_type(&CircuitType::Rollup1)
            .unwrap()
            .name(),
        "circuit-1.0"
    );
    assert_eq!(
        config
            .circuit_by_name("circuit-1.0")
            .unwrap()
            .circuit_type(),
        &CircuitType::Rollup1
    );
    let mut raw_config1 = raw_config.as_ref().clone();
    raw_config1.event_filter_size = Some(1000);
    raw_config1.indexer_filter_size = Some(10000);
    let config1 = PoolContractConfig::new(
        Arc::new(raw_config1),
        main_asset_config.clone(),
        asset_config.clone(),
        circuit_configs.clone(),
    );
    assert_eq!(config1.event_filter_size().unwrap(), 1000);
    assert_eq!(config1.indexer_filter_size().unwrap(), 10000);
}

#[tokio::test]
async fn test_validate_asset_address_mismatch() {
    let raw_config = Arc::new(
        create_raw_from_file::<RawPoolContractConfig>(VALID_CONFIG_FILE)
            .await
            .unwrap(),
    );
    let main_asset_config = Arc::new(AssetConfig::new(Arc::new(default_raw_main_asset_config())));
    let asset_config = Arc::new(AssetConfig::new(Arc::new(default_raw_asset_config(
        MAIN_ASSET_ADDRESS,
    ))));
    let circuit_configs: Vec<Arc<CircuitConfig>> = default_raw_circuit_configs()
        .into_iter()
        .map(|r| Arc::new(CircuitConfig::new(Arc::new(r))))
        .collect();
    let config =
        PoolContractConfig::new(raw_config, main_asset_config, asset_config, circuit_configs);
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "the given asset_config's address 0x0000000000000000000000000000000000000000 \
        is different with pool config 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 \
        asset_address 0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"
    );
}

#[tokio::test]
async fn test_validate_missing_circuit_name() {
    let raw_config = Arc::new(
        create_raw_from_file::<RawPoolContractConfig>(VALID_CONFIG_FILE)
            .await
            .unwrap(),
    );
    let main_asset_config = Arc::new(AssetConfig::new(Arc::new(default_raw_main_asset_config())));
    let asset_config = Arc::new(AssetConfig::new(Arc::new(default_raw_asset_config(
        raw_config.asset_address.as_ref().unwrap(),
    ))));
    let mut raw_circuit_configs = default_raw_circuit_configs();
    let mut raw_circuit_config = raw_circuit_configs.get_mut(0).unwrap();
    raw_circuit_config.name = String::from("no_existing_name");
    let circuit_configs = raw_circuit_configs
        .into_iter()
        .map(|r| Arc::new(CircuitConfig::new(Arc::new(r))))
        .collect();
    let config =
        PoolContractConfig::new(raw_config, main_asset_config, asset_config, circuit_configs);
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "circuit config is missing for circuit_name circuit-1.0"
    );
}

#[tokio::test]
async fn test_validate_missing_circuit_type() {
    let raw_config = Arc::new(
        create_raw_from_file::<RawPoolContractConfig>(VALID_CONFIG_FILE)
            .await
            .unwrap(),
    );
    let main_asset_config = Arc::new(AssetConfig::new(Arc::new(default_raw_main_asset_config())));
    let asset_config = Arc::new(AssetConfig::new(Arc::new(default_raw_asset_config(
        raw_config.asset_address.as_ref().unwrap(),
    ))));
    let mut raw_circuit_configs = default_raw_circuit_configs();
    raw_circuit_configs.remove(1);
    let circuit_configs = raw_circuit_configs
        .into_iter()
        .map(|r| Arc::new(CircuitConfig::new(Arc::new(r))))
        .collect();
    let config =
        PoolContractConfig::new(raw_config, main_asset_config, asset_config, circuit_configs);
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "circuit config is missing for circuit_type Rollup2"
    );
}

#[tokio::test]
async fn test_validate_duplicate_circuit_types() {
    let raw_config = Arc::new(
        create_raw_from_file::<RawPoolContractConfig>(VALID_CONFIG_FILE)
            .await
            .unwrap(),
    );
    let main_asset_config = Arc::new(AssetConfig::new(Arc::new(default_raw_main_asset_config())));
    let asset_config = Arc::new(AssetConfig::new(Arc::new(default_raw_asset_config(
        raw_config.asset_address.as_ref().unwrap(),
    ))));
    let mut raw_circuit_configs = default_raw_circuit_configs();
    let mut raw_circuit_config = raw_circuit_configs.get(1).unwrap().clone();
    raw_circuit_config.name = String::from("duplicate_rollup2");
    raw_circuit_configs.push(raw_circuit_config);
    let circuit_configs = raw_circuit_configs
        .into_iter()
        .map(|r| Arc::new(CircuitConfig::new(Arc::new(r))))
        .collect();
    let config =
        PoolContractConfig::new(raw_config, main_asset_config, asset_config, circuit_configs);
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "multiple circuit configs of circuit_type Rollup2"
    );
}

#[tokio::test]
async fn test_validate_asset_type_mismatch() {
    let raw_config = Arc::new(
        create_raw_from_file::<RawPoolContractConfig>(VALID_CONFIG_FILE)
            .await
            .unwrap(),
    );
    let main_asset_config = Arc::new(AssetConfig::new(Arc::new(default_raw_main_asset_config())));
    let mut raw_asset_config = default_raw_asset_config(raw_config.asset_address.as_ref().unwrap());
    raw_asset_config.asset_type = AssetType::Main;
    let asset_config = Arc::new(AssetConfig::new(Arc::new(raw_asset_config)));
    let circuit_configs: Vec<Arc<CircuitConfig>> = default_raw_circuit_configs()
        .into_iter()
        .map(|r| Arc::new(CircuitConfig::new(Arc::new(r))))
        .collect();
    let config = PoolContractConfig::new(
        raw_config.clone(),
        main_asset_config.clone(),
        asset_config.clone(),
        circuit_configs.clone(),
    );
    assert_eq!(
        config.validate().err().unwrap().to_string(),
        "pool contract 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 \
        asset_address should be None when asset_type is MAIN"
    );
    let mut raw_config1 = raw_config.as_ref().clone();
    raw_config1.asset_address = None;
    let mut raw_main_asset_config1 = default_raw_main_asset_config();
    raw_main_asset_config1.asset_type = AssetType::Erc20;
    let main_asset_config1 = Arc::new(AssetConfig::new(Arc::new(raw_main_asset_config1)));
    let config1 = PoolContractConfig::new(
        Arc::new(raw_config1),
        main_asset_config1,
        asset_config.clone(),
        circuit_configs.clone(),
    );
    assert_eq!(
        config1.validate().err().unwrap().to_string(),
        "pool contract 0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 \
        asset_address should NOT be None when asset_type is not MAIN"
    );
}

fn default_raw_main_asset_config() -> RawAssetConfig {
    RawAssetConfig {
        asset_address: MAIN_ASSET_ADDRESS.to_string(),
        asset_type: AssetType::Main,
        asset_decimals: 18,
        asset_symbol: String::from("ETH"),
        recommended_amounts: vec![String::from("1000000000000000000")],
    }
}

fn default_raw_asset_config(address: &str) -> RawAssetConfig {
    RawAssetConfig {
        asset_address: address.to_string(),
        asset_type: AssetType::Erc20,
        asset_decimals: 18,
        asset_symbol: String::from("MTT"),
        recommended_amounts: vec![
            String::from("1000000000000000000"),
            String::from("10000000000000000000"),
        ],
    }
}

fn default_raw_circuit_configs() -> Vec<RawCircuitConfig> {
    vec![
        RawCircuitConfig {
            name: String::from("circuit-1.0"),
            circuit_type: CircuitType::Rollup1,
            is_default: false,
            program_file: vec![String::from("./Rollup1_1.program.gz")],
            abi_file: vec![String::from("./Rollup1_1.abi.json")],
            proving_key_file: vec![String::from("./Rollup1_1.pkey.gz")],
            verifying_key_file: vec![String::from("./Rollup1_1.vkey.gz")],
        },
        RawCircuitConfig {
            name: String::from("zokrates-1.0-rollup2"),
            circuit_type: CircuitType::Rollup2,
            is_default: true,
            program_file: vec![String::from("./Rollup2.program.gz")],
            abi_file: vec![String::from("./Rollup2.abi.json")],
            proving_key_file: vec![String::from("./Rollup2.pkey.gz")],
            verifying_key_file: vec![String::from("./Rollup2.vkey.gz")],
        },
        RawCircuitConfig {
            name: String::from("zokrates-1.0-rollup4"),
            circuit_type: CircuitType::Rollup4,
            is_default: true,
            program_file: vec![String::from("./Rollup4.program.gz")],
            abi_file: vec![String::from("./Rollup4.abi.json")],
            proving_key_file: vec![String::from("./Rollup4.pkey.gz")],
            verifying_key_file: vec![String::from("./Rollup4.vkey.gz")],
        },
        RawCircuitConfig {
            name: String::from("zokrates-1.0-rollup8"),
            circuit_type: CircuitType::Rollup8,
            is_default: true,
            program_file: vec![String::from("./Rollup8.program.gz")],
            abi_file: vec![String::from("./Rollup8.abi.json")],
            proving_key_file: vec![String::from("./Rollup8.pkey.gz")],
            verifying_key_file: vec![String::from("./Rollup8.vkey.gz")],
        },
        RawCircuitConfig {
            name: String::from("zokrates-1.0-rollup16"),
            circuit_type: CircuitType::Rollup16,
            is_default: true,
            program_file: vec![String::from("./Rollup16.program.gz")],
            abi_file: vec![String::from("./Rollup16.abi.json")],
            proving_key_file: vec![String::from("./Rollup16.pkey.gz")],
            verifying_key_file: vec![String::from("./Rollup16.vkey.gz")],
        },
        RawCircuitConfig {
            name: String::from("zokrates-1.0-transaction1x0"),
            circuit_type: CircuitType::Transaction1x0,
            is_default: true,
            program_file: vec![String::from("./Transaction1x0.program.gz")],
            abi_file: vec![String::from("./Transaction1x0.abi.json")],
            proving_key_file: vec![String::from("./Transaction1x0.pkey.gz")],
            verifying_key_file: vec![String::from("./Transaction1x0.vkey.gz")],
        },
        RawCircuitConfig {
            name: String::from("zokrates-1.0-transaction1x1"),
            circuit_type: CircuitType::Transaction1x1,
            is_default: true,
            program_file: vec![String::from("./Transaction1x1.program.gz")],
            abi_file: vec![String::from("./Transaction1x1.abi.json")],
            proving_key_file: vec![String::from("./Transaction1x1.pkey.gz")],
            verifying_key_file: vec![String::from("./Transaction1x1.vkey.gz")],
        },
        RawCircuitConfig {
            name: String::from("zokrates-1.0-transaction1x2"),
            circuit_type: CircuitType::Transaction1x2,
            is_default: true,
            program_file: vec![String::from("./Transaction1x2.program.gz")],
            abi_file: vec![String::from("./Transaction1x2.abi.json")],
            proving_key_file: vec![String::from("./Transaction1x2.pkey.gz")],
            verifying_key_file: vec![String::from("./Transaction1x2.vkey.gz")],
        },
        RawCircuitConfig {
            name: String::from("zokrates-1.0-transaction2x0"),
            circuit_type: CircuitType::Transaction2x0,
            is_default: true,
            program_file: vec![String::from("./Transaction2x0.program.gz")],
            abi_file: vec![String::from("./Transaction2x0.abi.json")],
            proving_key_file: vec![String::from("./Transaction2x0.pkey.gz")],
            verifying_key_file: vec![String::from("./Transaction2x0.vkey.gz")],
        },
        RawCircuitConfig {
            name: String::from("zokrates-1.0-transaction2x1"),
            circuit_type: CircuitType::Transaction2x1,
            is_default: true,
            program_file: vec![String::from("./Transaction2x1.program.gz")],
            abi_file: vec![String::from("./Transaction2x1.abi.json")],
            proving_key_file: vec![String::from("./Transaction2x1.pkey.gz")],
            verifying_key_file: vec![String::from("./Transaction2x1.vkey.gz")],
        },
        RawCircuitConfig {
            name: String::from("zokrates-1.0-transaction2x2"),
            circuit_type: CircuitType::Transaction2x2,
            is_default: true,
            program_file: vec![String::from("./Transaction2x2.program.gz")],
            abi_file: vec![String::from("./Transaction2x2.abi.json")],
            proving_key_file: vec![String::from("./Transaction2x2.pkey.gz")],
            verifying_key_file: vec![String::from("./Transaction2x2.vkey.gz")],
        },
    ]
}
