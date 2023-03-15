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
use std::collections::HashMap;
use std::rc::Rc;

async fn raw_mystiko_config() -> RawMystikoConfig {
    RawConfig::create_from_file::<RawMystikoConfig>("tests/files/mystiko.valid.json")
        .await
        .unwrap()
}

async fn circuit_configs() -> (
    Rc<HashMap<String, CircuitConfig>>,
    Rc<HashMap<CircuitType, CircuitConfig>>,
) {
    let mut circuit_configs_by_name = HashMap::new();
    let mut default_circuit_configs = HashMap::new();
    let raw_mystiko_config = raw_mystiko_config().await;
    for circuit in raw_mystiko_config.circuits {
        let circuit_config = CircuitConfig::new(circuit.clone());
        circuit_configs_by_name.insert(circuit.name.clone(), circuit_config.clone());
        if circuit.is_default {
            default_circuit_configs.insert(circuit.circuit_type.clone(), circuit_config.clone());
        }
    }
    (
        Rc::new(circuit_configs_by_name),
        Rc::new(default_circuit_configs),
    )
}

async fn main_asset_config() -> Rc<AssetConfig> {
    let raw_mystiko_config = raw_mystiko_config().await;
    let asset_symbol = raw_mystiko_config
        .chains
        .get(0)
        .unwrap()
        .clone()
        .asset_symbol;
    let asset_decimals = raw_mystiko_config
        .chains
        .get(0)
        .unwrap()
        .clone()
        .asset_decimals;
    let recommended_amounts = raw_mystiko_config
        .chains
        .get(0)
        .unwrap()
        .clone()
        .recommended_amounts;
    Rc::new(
        AssetConfig::new(RawAssetConfig::new(
            AssetType::Main,
            asset_symbol,
            asset_decimals,
            "0x0000000000000000000000000000000000000000".to_string(),
            recommended_amounts,
        ))
        .unwrap(),
    )
}

async fn asset_configs() -> Rc<HashMap<String, AssetConfig>> {
    let mut asset_configs = HashMap::new();
    let raw_mystiko_config = raw_mystiko_config().await;
    let raw_asset_config = raw_mystiko_config
        .chains
        .get(0)
        .unwrap()
        .assets
        .get(0)
        .unwrap();
    asset_configs.insert(
        raw_asset_config.asset_address.clone(),
        AssetConfig::new(raw_asset_config.clone()).unwrap(),
    );
    Rc::new(asset_configs)
}

async fn default_raw_config() -> RawDepositContractConfig {
    let mut raw_config = RawConfig::create_from_file::<RawDepositContractConfig>(
        "tests/files/contract/deposit.valid.json",
    )
    .await
    .unwrap();
    raw_config.bridge_type = BridgeType::Loop;
    raw_config.peer_chain_id = None;
    raw_config.peer_contract_address = None;
    raw_config
}

async fn default_deposit_config() -> DepositContractConfig {
    let raw_mystiko_config = raw_mystiko_config().await;
    let pool_contract = raw_mystiko_config
        .chains
        .get(0)
        .unwrap()
        .pool_contracts
        .get(0)
        .unwrap();
    let mut pool_contract_configs = HashMap::new();
    let (circuit_configs_by_name, default_circuit_configs) = circuit_configs().await;
    pool_contract_configs.insert(
        pool_contract.address().to_string(),
        PoolContractConfig::new(
            pool_contract.clone(),
            Some(pool::AuxData::new(
                default_circuit_configs,
                circuit_configs_by_name,
                main_asset_config().await,
                asset_configs().await,
            )),
        )
        .unwrap(),
    );
    let aux_data = Some(AuxData::new(
        Rc::new(pool_contract_configs),
        main_asset_config().await,
        asset_configs().await,
        Rc::new(None),
    ));
    DepositContractConfig::new(default_raw_config().await, aux_data).unwrap()
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<DepositContractConfig> =
        AsyncOnce::new(async { default_deposit_config().await });
    static ref RAW_CONFIG_CREATER: AsyncOnce<RawDepositContractConfig> =
        AsyncOnce::new(async { default_raw_config().await });
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
    assert_eq!(config.min_amount_number().unwrap(), 1f64);
    assert_eq!(config.max_amount().to_string(), raw_config.max_amount);
    assert_eq!(config.max_amount_number().unwrap(), 10f64);
    assert_eq!(
        config.min_bridge_fee().to_string(),
        raw_config.min_bridge_fee
    );
    assert_eq!(config.min_bridge_fee_number(), 2f64);
    assert_eq!(
        config.min_executor_fee().to_string(),
        raw_config.min_executor_fee
    );
    assert_eq!(config.min_executor_fee_number().unwrap(), 3f64);
    assert_eq!(
        &*config.asset().unwrap(),
        asset_configs
            .get("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")
            .unwrap()
    );
    assert_eq!(config.asset_type().unwrap(), AssetType::Erc20);
    assert_eq!(config.asset_symbol().unwrap(), "MTT".to_string());
    assert_eq!(config.asset_decimals().unwrap(), 16);
    assert_eq!(
        config.asset_address().unwrap().unwrap(),
        "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string()
    );
    assert_eq!(
        config
            .recommended_amounts()
            .unwrap()
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>(),
        vec![
            String::from("10000000000000000"),
            String::from("100000000000000000"),
        ]
    );
    assert_eq!(
        config.recommended_amounts_number().unwrap(),
        vec![1f64, 10f64]
    );
    assert_eq!(
        config.min_rollup_fee().unwrap().to_string(),
        "40000000000000000".to_string()
    );
    assert_eq!(config.min_rollup_fee_number().unwrap(), 4f64);

    let circuit_names = config
        .circuits()
        .unwrap()
        .iter()
        .map(|conf| conf.name().clone())
        .collect::<Vec<String>>();
    assert_eq!(
        circuit_names.contains(&String::from("zokrates-2.0-rollup1")),
        true
    );
    assert_eq!(
        config.pool_contract().unwrap().address(),
        raw_config.pool_address
    );
    assert_eq!(config.peer_contract().is_none(), true);
    assert_eq!(
        config.bridge_fee_asset(),
        &asset_configs
            .get("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")
            .unwrap()
            .clone(),
    );
    assert_eq!(
        &*config.executor_fee_asset().unwrap(),
        asset_configs
            .get("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")
            .unwrap()
    );
    assert_eq!(config.service_fee(), 2);
    assert_eq!(config.service_fee_divider(), 1000);
}

#[tokio::test]
async fn test_bridge_fee_asset() {
    let mut raw_config = default_raw_config().await;
    let mut config = default_deposit_config().await;
    let main_asset_config = main_asset_config().await;
    raw_config.bridge_fee_asset_address = None;
    config = config.mutate(Some(raw_config.clone()), None).unwrap();
    assert_eq!(config.bridge_fee_asset(), &*main_asset_config);
    assert_eq!(config.min_bridge_fee_number(), 0.02f64);
    raw_config.bridge_fee_asset_address = Some(MAIN_ASSET_ADDRESS.to_string());
    config.mutate(Some(raw_config.clone()), None).unwrap();
    assert_eq!(config.bridge_fee_asset(), &*main_asset_config);
    raw_config.bridge_fee_asset_address =
        Some("0xBc28029D248FC60bce0bAC01cF41A53aEEaE06F9".to_string());
    let validate = config.mutate(Some(raw_config), None);
    assert_eq!(validate.is_err(), true);
    assert_eq!(
        validate.unwrap_err().errors.contains(
            &"bridge fee asset address=0xBc28029D248FC60bce0bAC01cF41A53aEEaE06F9 \
            config has not been defined for deposit contract \
            address=0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
                .to_string()
        ),
        true
    );
}

#[tokio::test]
async fn test_executor_fee_asset() {
    let mut raw_config = default_raw_config().await;
    let mut config = default_deposit_config().await;
    raw_config.executor_fee_asset_address = None;
    config = config.mutate(Some(raw_config.clone()), None).unwrap();
    assert_eq!(
        config.executor_fee_asset().unwrap(),
        config.asset().unwrap()
    );
    raw_config.executor_fee_asset_address = Some(MAIN_ASSET_ADDRESS.to_string());
    config = config.mutate(Some(raw_config.clone()), None).unwrap();
    assert_eq!(config.min_executor_fee_number().unwrap(), 0.03f64);
    assert_eq!(
        config.executor_fee_asset().unwrap(),
        main_asset_config().await
    );
    raw_config.executor_fee_asset_address =
        Some("0xBc28029D248FC60bce0bAC01cF41A53aEEaE06F9".to_string());
    let validate = config.mutate(Some(raw_config), None);
    assert_eq!(validate.is_err(), true);
    assert_eq!(
        validate.unwrap_err().errors.contains(
            &"executor fee asset address=0xBc28029D248FC60bce0bAC01cF41A53aEEaE06F9 config has \
            not been defined for deposit contract \
            address=0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
                .to_string()
        ),
        true
    );
}

#[tokio::test]
async fn test_peer_contract() {
    let raw_mystiko_config = raw_mystiko_config().await;
    let raw_asset_config = raw_mystiko_config
        .chains
        .get(1)
        .unwrap()
        .assets
        .get(0)
        .unwrap();
    let mut raw_config = default_raw_config().await;
    let mut peer_assets_config = HashMap::new();
    peer_assets_config.insert(
        raw_asset_config.asset_address.clone(),
        AssetConfig::new(raw_asset_config.clone()).unwrap(),
    );
    let (circuit_configs_by_name, default_circuit_configs) = circuit_configs().await;
    let mut pool_contract_configs: HashMap<String, PoolContractConfig> = HashMap::new();
    let main_asset_config = main_asset_config().await;
    let pool_contract = raw_mystiko_config
        .chains
        .get(1)
        .unwrap()
        .pool_contracts
        .get(0)
        .unwrap();
    pool_contract_configs.insert(
        pool_contract.address().to_string(),
        PoolContractConfig::new(
            pool_contract.clone(),
            Some(pool::AuxData::new(
                default_circuit_configs,
                circuit_configs_by_name,
                main_asset_config.clone(),
                Rc::new(peer_assets_config.clone()),
            )),
        )
        .unwrap(),
    );
    let asset_configs = asset_configs().await;
    let aux_data = Some(AuxData::new(
        Rc::new(pool_contract_configs),
        main_asset_config.clone(),
        asset_configs.clone(),
        Rc::new(None),
    ));
    let _peer_contract_config = DepositContractConfig::new(
        raw_mystiko_config
            .chains
            .get(1)
            .unwrap()
            .deposit_contracts
            .get(0)
            .unwrap()
            .clone(),
        aux_data,
    );
    raw_config.bridge_type = BridgeType::Tbridge;
    raw_config.peer_chain_id = Some(97);
    raw_config.peer_contract_address =
        Some(String::from("0xd791049D0a154bC7860804e1A18ACD148Eb0afD9"));
    let mystiko_config = MystikoConfig::new(raw_mystiko_config.clone()).unwrap();
    let config = DepositContractConfig::new(
        raw_config.clone(),
        Some(AuxData::new(
            Default::default(),
            main_asset_config.clone(),
            asset_configs.clone(),
            Rc::new(Some(mystiko_config.get_chain_configs())),
        )),
    )
    .unwrap();
    assert_eq!(
        config.peer_contract().unwrap().data().address(),
        "0xd791049D0a154bC7860804e1A18ACD148Eb0afD9"
    );
    assert_eq!(
        config.min_rollup_fee().unwrap().to_string(),
        "12345".to_string()
    );
    assert_eq!(config.min_rollup_fee_number().unwrap(), 1.2345e-12);
}

#[tokio::test]
async fn test_invalid_raw_config_0() {
    let mut raw_config = default_raw_config().await;
    raw_config.bridge_type = BridgeType::Tbridge;
    assert!(DepositContractConfig::new(
        raw_config,
        Some(AuxData::new(
            Default::default(),
            main_asset_config().await,
            asset_configs().await,
            Rc::new(None),
        )),
    )
    .is_err());
}

#[tokio::test]
async fn test_invalid_raw_config_1() {
    let mut raw_config = default_raw_config().await;
    raw_config.bridge_type = BridgeType::Loop;
    raw_config.peer_contract_address =
        Some(String::from("0xd791049D0a154bC7860804e1A18ACD148Eb0afD9"));
    assert!(DepositContractConfig::new(
        raw_config,
        Some(AuxData::new(
            Default::default(),
            main_asset_config().await,
            asset_configs().await,
            Rc::new(None),
        )),
    )
    .is_err());
}

#[tokio::test]
async fn test_invalid_raw_config_2() {
    let mut raw_config = default_raw_config().await;
    raw_config.bridge_type = BridgeType::Loop;
    raw_config.peer_contract_address = None;
    raw_config.peer_chain_id = Some(97);
    assert!(DepositContractConfig::new(
        raw_config,
        Some(AuxData::new(
            Default::default(),
            main_asset_config().await,
            asset_configs().await,
            Rc::new(None),
        )),
    )
    .is_err());
}

#[tokio::test]
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
            Rc::new(None),
        )),
    )
    .unwrap();
    let pool_contract = config.pool_contract();
    assert!(pool_contract.is_err());
    assert_eq!(
        pool_contract.unwrap_err().to_string(),
        "no poolContract definition found for deposit \
        contract=0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
            .to_string()
    );
}

#[tokio::test]
async fn test_invalid_max_amount() {
    let mut raw_config = default_raw_config().await;
    let config = CONFIG_CREATER.get().await;
    raw_config.max_amount = String::from("10000");
    let validate = config.mutate(Some(raw_config.clone()), None);
    assert!(validate.is_err());
    assert!(validate.unwrap_err().errors.contains(
        &"deposit contract=0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 maxAmount is less than \
            minAmount"
            .to_string()
    ));
}

#[tokio::test]
async fn test_copy() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    assert_eq!(&config.copy_data(), raw_config);
}

#[tokio::test]
async fn test_mutate() {
    let mut raw_config = default_raw_config().await;
    let config = CONFIG_CREATER.get().await;
    assert_eq!(&config.mutate(None, None).unwrap(), config);
    raw_config.base.name = "another name".to_string();
    let new_config = config.mutate(Some(raw_config.clone()), None).unwrap();
    assert_eq!(new_config.name(), "another name".to_string());
}

#[tokio::test]
async fn test_to_json_string() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    let json_string = config.to_json_string();
    let loaded_raw_config =
        RawConfig::create_from_json_string::<RawDepositContractConfig>(json_string.as_str())
            .await
            .unwrap();
    assert_eq!(&loaded_raw_config, raw_config);
}
