use std::collections::HashMap;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::common::{AssetType, BridgeType, CircuitType};
use mystiko_config::raw::asset::RawAssetConfig;
use mystiko_config::raw::base::RawConfig;
use mystiko_config::raw::contract::base::RawContractConfigTrait;
use mystiko_config::raw::contract::deposit::RawDepositContractConfig;
use mystiko_config::raw::mystiko::RawMystikoConfig;
use mystiko_config::wrapper::asset::{AssetConfig, MAIN_ASSET_ADDRESS};
use mystiko_config::wrapper::circuit::CircuitConfig;
use mystiko_config::wrapper::contract::deposit::{AuxData, DepositContractConfig};
use mystiko_config::wrapper::contract::pool;
use mystiko_config::wrapper::contract::pool::PoolContractConfig;
use mystiko_config::wrapper::mystiko::MystikoConfig;

async fn raw_mystiko_config() -> RawMystikoConfig {
    RawConfig::create_from_file::<RawMystikoConfig>("tests/files/mystiko.valid.json").await
}

async fn circuit_configs() -> (HashMap<String, CircuitConfig>, HashMap<CircuitType, CircuitConfig>) {
    let mut circuit_configs_by_name = HashMap::new();
    let mut default_circuit_configs = HashMap::new();
    let raw_mystiko_config = raw_mystiko_config().await;
    for circuit in raw_mystiko_config.circuits {
        let circuit_config = CircuitConfig::new(circuit.clone());
        circuit_configs_by_name.insert(circuit.name.clone(), circuit_config.clone());
        if circuit.is_default {
            default_circuit_configs.insert(
                circuit.circuit_type.clone(),
                circuit_config.clone(),
            );
        }
    }
    (circuit_configs_by_name, default_circuit_configs)
}

async fn main_asset_config() -> AssetConfig {
    let raw_mystiko_config = raw_mystiko_config().await;
    let asset_symbol =
        raw_mystiko_config.chains.get(0).unwrap().clone().asset_symbol;
    let asset_decimals =
        raw_mystiko_config.chains.get(0).unwrap().clone().asset_decimals;
    let recommended_amounts =
        raw_mystiko_config.chains.get(0).unwrap().clone().recommended_amounts;
    AssetConfig::new(
        RawAssetConfig::new(
            AssetType::Main,
            asset_symbol,
            asset_decimals,
            "0x0000000000000000000000000000000000000000".to_string(),
            recommended_amounts,
        )
    )
}

async fn asset_configs() -> HashMap<String, AssetConfig> {
    let mut asset_configs = HashMap::new();
    let raw_mystiko_config = raw_mystiko_config().await;
    let raw_asset_config =
        raw_mystiko_config.chains.get(0).unwrap().assets.get(0).unwrap();
    asset_configs.insert(
        raw_asset_config.asset_address.clone(),
        AssetConfig::new(raw_asset_config.clone()),
    );
    asset_configs
}

async fn default_raw_config() -> RawDepositContractConfig {
    let mut raw_config =
        RawConfig::create_from_file::<RawDepositContractConfig>(
            "tests/files/contract/deposit.valid.json"
        ).await;
    raw_config.bridge_type = BridgeType::Loop;
    raw_config.peer_chain_id = None;
    raw_config.peer_contract_address = None;
    raw_config
}

async fn default_deposit_config() -> DepositContractConfig {
    let raw_mystiko_config = raw_mystiko_config().await;
    let pool_contract =
        raw_mystiko_config.chains.get(0).unwrap().pool_contracts.get(0).unwrap();
    let mut pool_contract_configs = HashMap::new();
    let (
        circuit_configs_by_name,
        default_circuit_configs
    ) = circuit_configs().await;
    pool_contract_configs.insert(
        pool_contract.address().to_string(),
        PoolContractConfig::new(
            pool_contract.clone(),
            Some(
                pool::AuxData::new(
                    default_circuit_configs,
                    circuit_configs_by_name,
                    main_asset_config().await,
                    asset_configs().await,
                )
            ),
        ),
    );
    let aux_data = Some(AuxData::new(
        pool_contract_configs,
        main_asset_config().await,
        asset_configs().await,
        None,
    ));
    DepositContractConfig::new(default_raw_config().await, aux_data)
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<DepositContractConfig> = AsyncOnce::new(async {
        default_deposit_config().await
    });
    static ref RAW_CONFIG_CREATER: AsyncOnce<RawDepositContractConfig> = AsyncOnce::new(async {
        default_raw_config().await
    });
}

#[tokio::test]
async fn test_equality() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    let asset_configs = asset_configs().await;
    assert_eq!(config.bridge_type(), raw_config.bridge_type);
    assert_eq!(config.pool_address(), &raw_config.pool_address);
    assert_eq!(config.disabled(), raw_config.disabled);
    assert_eq!(config.min_amount().to_string(), raw_config.min_amount);
    assert_eq!(config.min_amount_number(), 1f64);
    assert_eq!(config.max_amount().to_string(), raw_config.max_amount);
    assert_eq!(config.max_amount_number(), 10f64);
    assert_eq!(config.min_bridge_fee().to_string(), raw_config.min_bridge_fee);
    assert_eq!(config.min_bridge_fee_number(), 2f64);
    assert_eq!(config.min_executor_fee().to_string(), raw_config.min_executor_fee);
    assert_eq!(config.min_executor_fee_number(), 3f64);
    assert_eq!(config.asset(), asset_configs.get("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a").unwrap().clone());
    assert_eq!(config.asset_type(), AssetType::Erc20);
    assert_eq!(config.asset_symbol(), "MTT".to_string());
    assert_eq!(config.asset_decimals(), 16);
    assert_eq!(config.asset_address().unwrap(), "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string());
    assert_eq!(
        config.recommended_amounts().iter().map(|a| a.to_string()).collect::<Vec<String>>(),
        vec![
            String::from("10000000000000000"),
            String::from("100000000000000000"),
        ]
    );
    assert_eq!(config.recommended_amounts_number(), vec![1f64, 10f64]);
    assert_eq!(config.min_rollup_fee().to_string(), "40000000000000000".to_string());
    assert_eq!(config.min_rollup_fee_number(), 4f64);

    let circuit_names =
        config.circuits().iter().map(|conf| conf.name().clone()).collect::<Vec<String>>();
    assert_eq!(circuit_names.contains(&String::from("zokrates-2.0-rollup1")), true);
    assert_eq!(config.pool_contract().base.base.data.address(), raw_config.pool_address);
    assert_eq!(config.peer_contract().is_none(), true);
    assert_eq!(
        config.bridge_fee_asset(),
        asset_configs.get("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a").unwrap().clone(),
    );
    assert_eq!(
        config.executor_fee_asset(),
        asset_configs.get("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a").unwrap().clone()
    );
    assert_eq!(config.service_fee(), 2);
    assert_eq!(config.service_fee_divider(), 1000);
}

#[tokio::test]
#[should_panic(
expected = "bridge fee asset address=0xBc28029D248FC60bce0bAC01cF41A53aEEaE06F9 config has not \
    been defined for deposit contract address=0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
)]
async fn test_bridge_fee_asset() {
    let mut raw_config = default_raw_config().await;
    let mut config = default_deposit_config().await;
    let main_asset_config = main_asset_config().await;
    raw_config.bridge_fee_asset_address = None;
    config = config.mutate(Some(raw_config.clone()), None);
    assert_eq!(config.bridge_fee_asset(), main_asset_config);
    assert_eq!(config.min_bridge_fee_number(), 0.02f64);
    raw_config.bridge_fee_asset_address = Some(MAIN_ASSET_ADDRESS.to_string());
    config.mutate(Some(raw_config.clone()), None);
    assert_eq!(config.bridge_fee_asset(), main_asset_config);
    raw_config.bridge_fee_asset_address = Some("0xBc28029D248FC60bce0bAC01cF41A53aEEaE06F9".to_string());
    config.mutate(Some(raw_config), None);
}

#[tokio::test]
#[should_panic(expected = "executor fee asset address=0xBc28029D248FC60bce0bAC01cF41A53aEEaE06F9 \
     config has not been defined for deposit contract address=0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
)]
async fn test_executor_fee_asset() {
    let mut raw_config = default_raw_config().await;
    let mut config = default_deposit_config().await;
    raw_config.executor_fee_asset_address = None;
    config = config.mutate(Some(raw_config.clone()), None);
    assert_eq!(config.executor_fee_asset(), config.asset());
    raw_config.executor_fee_asset_address = Some(MAIN_ASSET_ADDRESS.to_string());
    config = config.mutate(Some(raw_config.clone()), None);
    assert_eq!(config.min_executor_fee_number(), 0.03f64);
    assert_eq!(config.executor_fee_asset(), main_asset_config().await);
    raw_config.executor_fee_asset_address = Some("0xBc28029D248FC60bce0bAC01cF41A53aEEaE06F9".to_string());
    config.mutate(Some(raw_config), None);
}

#[tokio::test]
async fn test_peer_contract() {
    let raw_mystiko_config = raw_mystiko_config().await;
    let raw_asset_config =
        raw_mystiko_config.chains.get(1).unwrap().assets.get(0).unwrap();
    let mut raw_config = default_raw_config().await;
    let mut peer_assets_config = HashMap::new();
    peer_assets_config.insert(
        raw_asset_config.asset_address.clone(),
        AssetConfig::new(raw_asset_config.clone()),
    );
    let (circuit_configs_by_name, default_circuit_configs) = circuit_configs().await;
    let mut pool_contract_configs: HashMap<String, PoolContractConfig> = HashMap::new();
    let main_asset_config = main_asset_config().await;
    let pool_contract = raw_mystiko_config.chains.get(1).unwrap().pool_contracts.get(0).unwrap();
    pool_contract_configs.insert(
        pool_contract.address().to_string(),
        PoolContractConfig::new(
            pool_contract.clone(),
            Some(
                pool::AuxData::new(
                    default_circuit_configs,
                    circuit_configs_by_name,
                    main_asset_config.clone(),
                    peer_assets_config.clone(),
                )
            ),
        ),
    );
    let asset_configs = asset_configs().await;
    let aux_data = Some(AuxData::new(
        pool_contract_configs,
        main_asset_config.clone(),
        asset_configs.clone(),
        None,
    ));
    let _peer_contract_config =
        DepositContractConfig::new(
            raw_mystiko_config.chains.get(1).unwrap().deposit_contracts.get(0).unwrap().clone(),
            aux_data,
        );
    raw_config.bridge_type = BridgeType::Tbridge;
    raw_config.peer_chain_id = Some(97);
    raw_config.peer_contract_address = Some(String::from("0xd791049D0a154bC7860804e1A18ACD148Eb0afD9"));
    let mystiko_config = MystikoConfig::new(raw_mystiko_config.clone());
    let config = DepositContractConfig::new(raw_config.clone(), Some(
        AuxData::new(
            Default::default(),
            main_asset_config.clone(),
            asset_configs.clone(),
            Some(mystiko_config.get_chain_configs()),
        )
    ));
    assert_eq!(
        config.peer_contract().unwrap().base.base.data.address(),
        "0xd791049D0a154bC7860804e1A18ACD148Eb0afD9"
    );
    assert_eq!(config.min_rollup_fee().to_string(), "12345".to_string());
    assert_eq!(config.min_rollup_fee_number(), 1.2345e-12);
}

#[tokio::test]
#[should_panic]
async fn test_invalid_raw_config_0() {
    let mut raw_config = default_raw_config().await;
    raw_config.bridge_type = BridgeType::Tbridge;
    DepositContractConfig::new(
        raw_config,
        Some(AuxData::new(
            Default::default(),
            main_asset_config().await,
            asset_configs().await,
            None,
        )),
    );
}

#[tokio::test]
#[should_panic]
async fn test_invalid_raw_config_1() {
    let mut raw_config = default_raw_config().await;
    raw_config.bridge_type = BridgeType::Loop;
    raw_config.peer_contract_address = Some(String::from("0xd791049D0a154bC7860804e1A18ACD148Eb0afD9"));
    DepositContractConfig::new(
        raw_config,
        Some(AuxData::new(
            Default::default(),
            main_asset_config().await,
            asset_configs().await,
            None,
        )),
    );
}

#[tokio::test]
#[should_panic]
async fn test_invalid_raw_config_2() {
    let mut raw_config = default_raw_config().await;
    raw_config.bridge_type = BridgeType::Loop;
    raw_config.peer_contract_address = None;
    raw_config.peer_chain_id = Some(97);
    DepositContractConfig::new(
        raw_config,
        Some(AuxData::new(
            Default::default(),
            main_asset_config().await,
            asset_configs().await,
            None,
        )),
    );
}

#[tokio::test]
#[should_panic(expected = "no poolContract definition found for deposit contract=\"0x961f315a836542e603a3df2e0dd9d4ecd06ebc67\"")]
async fn test_invalid_raw_config_3() {
    let mut raw_config = default_raw_config().await;
    raw_config.bridge_type = BridgeType::Loop;
    raw_config.peer_contract_address = None;
    raw_config.peer_chain_id = None;
    let config = DepositContractConfig::new(
        raw_config,
        Some(AuxData::new(
            Default::default(),
            main_asset_config().await,
            asset_configs().await,
            None,
        )),
    );
    config.pool_contract();
}

#[tokio::test]
#[should_panic(expected = "deposit contract=0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 maxAmount is less than minAmount")]
async fn test_invalid_max_amount() {
    let mut raw_config= default_raw_config().await;
    let config = CONFIG_CREATER.get().await;
    raw_config.max_amount = String::from("10000");
    config.mutate(Some(raw_config.clone()), None);
}

#[tokio::test]
async fn test_copy() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    assert_eq!(&config.base.base.copy_data(), raw_config);
}

#[tokio::test]
async fn test_mutate() {
    let mut raw_config= default_raw_config().await;
    let config = CONFIG_CREATER.get().await;
    assert_eq!(&config.mutate(None, None), config);
    raw_config.base.name = "another name".to_string();
    let new_config = config.mutate(Some(raw_config.clone()), None);
    assert_eq!(new_config.base.name(), "another name".to_string());
}

#[tokio::test]
async fn test_to_json_string() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    let json_string = config.base.base.to_json_string();
    let loaded_raw_config =
        RawConfig::create_from_json_string::<RawDepositContractConfig>(json_string.as_str()).await;
    assert_eq!(&loaded_raw_config, raw_config);
}