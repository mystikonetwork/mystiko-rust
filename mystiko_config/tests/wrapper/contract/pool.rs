use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::common::{AssetType, CircuitType};
use mystiko_config::raw::asset::RawAssetConfig;
use mystiko_config::raw::base::RawConfig;
use mystiko_config::raw::contract::pool::RawPoolContractConfig;
use mystiko_config::raw::mystiko::RawMystikoConfig;
use mystiko_config::wrapper::asset::AssetConfig;
use mystiko_config::wrapper::circuit::CircuitConfig;
use mystiko_config::wrapper::contract::pool::{AuxData, PoolContractConfig};
use std::collections::HashMap;
use std::rc::Rc;

async fn raw_mystiko_config() -> RawMystikoConfig {
    RawConfig::create_from_file::<RawMystikoConfig>("tests/files/mystiko.valid.json")
        .await
        .unwrap()
}

async fn circuit_configs_by_name() -> Rc<HashMap<String, CircuitConfig>> {
    let raw_mystiko_config = raw_mystiko_config().await;
    let mut configs = HashMap::new();
    for circuit in raw_mystiko_config.circuits {
        let circuit_config = CircuitConfig::new(circuit.clone());
        configs.insert(circuit.name.clone(), circuit_config);
    }
    Rc::new(configs)
}

async fn default_circuit_configs(
    circuit_configs_by_name: &HashMap<String, CircuitConfig>,
) -> Rc<HashMap<CircuitType, CircuitConfig>> {
    let mut default_configs = HashMap::new();
    for (_, circuit_config) in circuit_configs_by_name {
        if circuit_config.is_default() {
            default_configs.insert(
                circuit_config.circuit_type().clone(),
                circuit_config.clone(),
            );
        }
    }
    Rc::new(default_configs)
}

async fn default_raw_config() -> RawPoolContractConfig {
    RawConfig::create_from_file::<RawPoolContractConfig>("tests/files/contract/pool.valid.json")
        .await
        .unwrap()
}

async fn main_asset_config() -> Rc<AssetConfig> {
    let raw_mystiko_config = raw_mystiko_config().await;
    Rc::new(
        AssetConfig::new(RawAssetConfig::new(
            AssetType::Main,
            raw_mystiko_config
                .chains
                .get(0)
                .unwrap()
                .asset_symbol
                .clone(),
            raw_mystiko_config
                .chains
                .get(0)
                .unwrap()
                .asset_decimals
                .clone(),
            "0x0000000000000000000000000000000000000000".to_string(),
            raw_mystiko_config
                .chains
                .get(0)
                .unwrap()
                .recommended_amounts
                .clone(),
        ))
        .unwrap(),
    )
}

async fn asset_configs() -> Rc<HashMap<String, AssetConfig>> {
    let raw_mystiko_config = raw_mystiko_config().await;
    Rc::new(HashMap::from_iter([(
        raw_mystiko_config
            .chains
            .get(0)
            .unwrap()
            .assets
            .get(0)
            .unwrap()
            .asset_address
            .clone(),
        AssetConfig::new(
            raw_mystiko_config
                .chains
                .get(0)
                .unwrap()
                .assets
                .get(0)
                .unwrap()
                .clone(),
        )
        .unwrap(),
    )]))
}

async fn default_config() -> PoolContractConfig {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let circuit_configs_by_names = circuit_configs_by_name().await;
    PoolContractConfig::new(
        raw_config.clone(),
        Some(AuxData::new(
            default_circuit_configs(&circuit_configs_by_names).await,
            circuit_configs_by_names,
            main_asset_config().await,
            asset_configs().await,
        )),
    )
    .unwrap()
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<PoolContractConfig> =
        AsyncOnce::new(async { default_config().await });
    static ref RAW_CONFIG_CREATER: AsyncOnce<RawPoolContractConfig> =
        AsyncOnce::new(async { default_raw_config().await });
}

#[tokio::test]
async fn test_equality() {
    let config = CONFIG_CREATER.get().await;
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let asset_configs = asset_configs().await;
    assert_eq!(config.pool_name(), &raw_config.pool_name);
    assert_eq!(config.bridge_type(), &raw_config.bridge_type);
    assert_eq!(
        &*config.asset(),
        asset_configs
            .get("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")
            .unwrap()
    );
    assert_eq!(&config.asset_type(), config.asset().asset_type());
    assert_eq!(config.asset_symbol(), config.asset().asset_symbol());
    assert_eq!(config.asset_decimals(), config.asset().asset_decimals());
    assert_eq!(
        config.asset_address().unwrap(),
        raw_config.asset_address.clone().unwrap()
    );
    assert_eq!(
        config.recommended_amounts(),
        config.asset().recommended_amounts()
    );
    assert_eq!(
        config.recommended_amounts_number(),
        config.asset().recommended_amounts_number()
    );
    assert_eq!(
        config.min_rollup_fee().to_string(),
        raw_config.min_rollup_fee
    );
    assert_eq!(config.min_rollup_fee_number(), 12f64);
}

#[tokio::test]
async fn test_asset_address_is_none() {
    let mut raw_config = default_raw_config().await;
    raw_config.asset_address = None;
    let circuit_configs_by_name = circuit_configs_by_name().await;
    let default_circuit_configs = default_circuit_configs(&circuit_configs_by_name).await;
    let main_asset_config = main_asset_config().await;
    let asset_configs = asset_configs().await;
    let config = PoolContractConfig::new(
        raw_config,
        Some(AuxData::new(
            default_circuit_configs,
            circuit_configs_by_name,
            main_asset_config.clone(),
            asset_configs,
        )),
    )
    .unwrap();
    assert_eq!(config.asset(), main_asset_config);
}

#[tokio::test]
async fn test_circuit_overwrite() {
    let mut config = default_config().await;
    assert_eq!(
        config.circuit_config(CircuitType::Rollup1).unwrap().name(),
        "zokrates-1.0-rollup1"
    );
    let mut raw_config = default_raw_config().await;
    raw_config.circuits = vec![String::from("zokrates-2.0-rollup1")];
    let circuit_configs_by_name = circuit_configs_by_name().await;
    let default_circuit_configs = default_circuit_configs(&circuit_configs_by_name).await;
    let main_asset_config = main_asset_config().await;
    let asset_configs = asset_configs().await;
    config = PoolContractConfig::new(
        raw_config,
        Some(AuxData::new(
            default_circuit_configs,
            circuit_configs_by_name,
            main_asset_config,
            asset_configs,
        )),
    )
    .unwrap();
    assert_eq!(
        config.circuit_config(CircuitType::Rollup1).unwrap().name(),
        "zokrates-2.0-rollup1"
    );
    assert_eq!(
        config.circuit_config(CircuitType::Rollup4).unwrap().name(),
        "zokrates-1.0-rollup4"
    );
}

#[tokio::test]
async fn test_copy() {
    let config = CONFIG_CREATER.get().await;
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let circuit_configs_by_name = circuit_configs_by_name().await;
    let default_circuit_configs = default_circuit_configs(&circuit_configs_by_name).await;
    let main_asset_config = main_asset_config().await;
    let asset_configs = asset_configs().await;
    assert_eq!(
        &PoolContractConfig::new(
            raw_config.clone(),
            Some(AuxData::new(
                default_circuit_configs,
                circuit_configs_by_name,
                main_asset_config,
                asset_configs,
            ))
        )
        .unwrap(),
        config
    )
}

#[tokio::test]
async fn test_mutate() {
    let config = CONFIG_CREATER.get().await;
    assert_eq!(&config.mutate(None, None).unwrap(), config);
    let mut raw_config = default_raw_config().await;
    raw_config.base.name = "another name".to_string();
    let circuit_configs_by_name = circuit_configs_by_name().await;
    let default_circuit_configs = default_circuit_configs(&circuit_configs_by_name).await;
    let main_asset_config = main_asset_config().await;
    let asset_configs = asset_configs().await;
    let new_config = config
        .mutate(
            Some(raw_config.clone()),
            Some(AuxData::new(
                default_circuit_configs,
                circuit_configs_by_name,
                main_asset_config,
                asset_configs,
            )),
        )
        .unwrap();
    assert_eq!(new_config.copy_data(), raw_config);
}

#[tokio::test]
async fn test_to_json_string() {
    let config = CONFIG_CREATER.get().await;
    let json_string = config.to_json_string();
    let loaded_raw_config =
        RawConfig::create_from_json_string::<RawPoolContractConfig>(json_string.as_str())
            .await
            .unwrap();
    assert_eq!(&loaded_raw_config, RAW_CONFIG_CREATER.get().await);
}
