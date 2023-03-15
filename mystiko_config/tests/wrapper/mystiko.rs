use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::common::{BridgeType, CircuitType};
use mystiko_config::raw::base::RawConfig;
use mystiko_config::raw::bridge::celer::RawCelerBridgeConfig;
use mystiko_config::raw::bridge::poly::RawPolyBridgeConfig;
use mystiko_config::raw::bridge::tbridge::RawTBridgeConfig;
use mystiko_config::raw::chain::RawChainConfig;
use mystiko_config::raw::circuit::RawCircuitConfig;
use mystiko_config::raw::indexer::RawIndexerConfig;
use mystiko_config::raw::mystiko::{RawBridgeConfigType, RawMystikoConfig};
use mystiko_config::raw::provider::RawProviderConfig;
use mystiko_config::wrapper::contract::deposit;
use mystiko_config::wrapper::mystiko::{BridgeConfigType, MystikoConfig};

async fn default_raw_config() -> RawMystikoConfig {
    RawConfig::create_from_file::<RawMystikoConfig>("tests/files/mystiko.valid01.json")
        .await
        .unwrap()
}

async fn default_mystiko_config() -> MystikoConfig {
    MystikoConfig::create_from_raw(default_raw_config().await)
        .await
        .unwrap()
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<MystikoConfig> =
        AsyncOnce::new(async { default_mystiko_config().await });
    static ref RAW_CONFIG_CREATER: AsyncOnce<RawMystikoConfig> =
        AsyncOnce::new(async { default_raw_config().await });
}

#[tokio::test]
async fn test_equality() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    assert_eq!(config.version(), raw_config.version);

    let a = config
        .chains()
        .iter()
        .map(|conf| conf.copy_data())
        .collect::<Vec<RawChainConfig>>();
    let b = raw_config.chains.clone();
    assert_eq!(a.len(), b.len());
    for x in a {
        assert_eq!(b.contains(&x), true);
    }

    let a = config
        .circuits()
        .iter()
        .map(|conf| conf.copy_data())
        .collect::<Vec<RawCircuitConfig>>();
    let b = raw_config.circuits.clone();
    assert_eq!(a.len(), b.len());
    for x in a {
        assert_eq!(b.contains(&x), true);
    }

    let mut a: Vec<RawBridgeConfigType> = Vec::new();
    let b = raw_config.bridges.clone();
    let bridges = config.bridges();
    for bridge in bridges {
        match bridge {
            BridgeConfigType::AxelarBridgeConfig(v) => {
                a.push(RawBridgeConfigType::Axelar(v.copy_data()));
            }
            BridgeConfigType::CelerBridgeConfig(v) => {
                a.push(RawBridgeConfigType::Celer(v.copy_data()));
            }
            BridgeConfigType::PolyBridgeConfig(v) => {
                a.push(RawBridgeConfigType::Poly(v.copy_data()));
            }
            BridgeConfigType::LayerZeroBridgeConfig(v) => {
                a.push(RawBridgeConfigType::LayerZero(v.copy_data()));
            }
            BridgeConfigType::TBridgeConfig(v) => {
                a.push(RawBridgeConfigType::Tbridge(v.copy_data()));
            }
        }
    }
    assert_eq!(a.len(), b.len());
    for x in a {
        assert_eq!(b.contains(&x), true);
    }

    assert_eq!(config.indexer().is_none(), true);
}

#[tokio::test]
async fn test_get_chain_config() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    assert_eq!(
        config.get_chain_config(3).unwrap().copy_data(),
        raw_config.chains[0]
    );
    assert_eq!(
        config.get_chain_config(97).unwrap().copy_data(),
        raw_config.chains[1]
    );
    assert_eq!(config.get_chain_config(1024).is_none(), true);
}

#[tokio::test]
async fn test_get_peer_chain_configs() {
    let config = CONFIG_CREATER.get().await;
    let mut a = config
        .get_peer_chain_configs(3)
        .iter()
        .map(|conf| conf.chain_id().clone())
        .collect::<Vec<u32>>();
    a.sort();
    assert_eq!(a, vec![3, 97]);
    assert_eq!(
        config
            .get_peer_chain_configs(97)
            .iter()
            .map(|conf| conf.chain_id().clone())
            .collect::<Vec<u32>>(),
        vec![3]
    );
    assert_eq!(config.get_peer_chain_configs(1024).len(), 0);
}

#[tokio::test]
async fn test_get_asset_symbols() {
    let config = CONFIG_CREATER.get().await;
    assert_eq!(config.get_asset_symbols(3, 97).unwrap(), vec!["MTT"]);
    assert_eq!(config.get_asset_symbols(3, 3).unwrap(), vec!["ETH"]);
    assert_eq!(config.get_asset_symbols(97, 3).unwrap(), vec!["MTT"]);
    assert_eq!(config.get_asset_symbols(97, 97).unwrap().len(), 0);
    assert_eq!(config.get_asset_symbols(3, 1024).unwrap().len(), 0);
    assert_eq!(config.get_asset_symbols(1024, 97).unwrap().len(), 0);
}

#[tokio::test]
async fn test_get_bridges() {
    let config = CONFIG_CREATER.get().await;
    let a = config
        .get_bridges(3, 97, "MTT")
        .unwrap()
        .iter()
        .map(|conf| match conf {
            BridgeConfigType::AxelarBridgeConfig(v) => v.bridge_type().clone(),
            BridgeConfigType::CelerBridgeConfig(v) => v.bridge_type().clone(),
            BridgeConfigType::PolyBridgeConfig(v) => v.bridge_type().clone(),
            BridgeConfigType::LayerZeroBridgeConfig(v) => v.bridge_type().clone(),
            BridgeConfigType::TBridgeConfig(v) => v.bridge_type().clone(),
        })
        .collect::<Vec<BridgeType>>();
    let b = vec![
        BridgeType::Axelar,
        BridgeType::Celer,
        BridgeType::LayerZero,
        BridgeType::Tbridge,
    ];
    assert_eq!(a.len(), b.len());
    for x in a {
        assert_eq!(b.contains(&x), true);
    }
    assert_eq!(config.get_bridges(1024, 97, "MTT").unwrap().len(), 0);
    assert_eq!(config.get_bridges(3, 1024, "MTT").unwrap().len(), 0);
    assert_eq!(config.get_bridges(3, 97, "ETH").unwrap().len(), 0);
}

#[tokio::test]
async fn test_get_deposit_contract_config() {
    let config = CONFIG_CREATER.get().await;
    assert_eq!(
        &config
            .get_deposit_contract_config(3, 97, "MTT", BridgeType::Celer,)
            .unwrap()
            .unwrap(),
        config
            .get_deposit_contract_config_by_address(
                3,
                "0xe6394a06905d83B19Dbd51804Ca84677a2054FA6".to_string(),
            )
            .unwrap()
    );
    assert_eq!(
        &config
            .get_deposit_contract_config(3, 97, "MTT", BridgeType::Tbridge,)
            .unwrap()
            .unwrap(),
        config
            .get_deposit_contract_config_by_address(
                3,
                "0xbF5605f5Ed6d18ed957cBA80dbA8838dFcb9A69f".to_string(),
            )
            .unwrap()
    );
    assert_eq!(
        &config
            .get_deposit_contract_config(3, 3, "ETH", BridgeType::Loop,)
            .unwrap()
            .unwrap(),
        config
            .get_deposit_contract_config_by_address(
                3,
                "0x390d485f4d43212d4ae8cdd967a711514ed5a54f".to_string(),
            )
            .unwrap()
    );
    assert_eq!(
        &config
            .get_deposit_contract_config(97, 3, "MTT", BridgeType::Celer,)
            .unwrap()
            .unwrap(),
        config
            .get_deposit_contract_config_by_address(
                97,
                "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".to_string(),
            )
            .unwrap()
    );
    assert_eq!(
        &config
            .get_deposit_contract_config(97, 3, "MTT", BridgeType::Tbridge,)
            .unwrap()
            .unwrap(),
        config
            .get_deposit_contract_config_by_address(
                97,
                "0x9C33eaCc2F50E39940D3AfaF2c7B8246B681A374".to_string(),
            )
            .unwrap()
    );
    assert_eq!(
        config
            .get_deposit_contract_config(1024, 3, "MTT", BridgeType::Tbridge,)
            .unwrap()
            .is_none(),
        true
    );
    assert_eq!(
        config
            .get_deposit_contract_config_by_address(
                1024,
                "0x9C33eaCc2F50E39940D3AfaF2c7B8246B681A374".to_string(),
            )
            .is_none(),
        true
    );
    assert_eq!(
        config
            .get_deposit_contract_config_by_address(
                3,
                "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67".to_string(),
            )
            .is_some(),
        true
    );
    assert_eq!(
        config
            .get_deposit_contract_config_by_address(
                97,
                "0xd791049D0a154bC7860804e1A18ACD148Eb0afD9".to_string(),
            )
            .is_some(),
        true
    );
    let deposit_contract_config = config
        .get_deposit_contract_config_by_address(
            3,
            "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67".to_string(),
        )
        .unwrap();
    let deposit_config1 = &deposit_contract_config.peer_contract().unwrap();
    let deposit_config2 = config
        .get_deposit_contract_config_by_address(
            97,
            "0xd791049D0a154bC7860804e1A18ACD148Eb0afD9".to_string(),
        )
        .unwrap();

    assert_eq!(
        deposit_config1
            .mutate(None, Some(deposit::AuxData::default()))
            .unwrap(),
        deposit_config2
            .mutate(None, Some(deposit::AuxData::default()))
            .unwrap(),
    );
}

#[tokio::test]
async fn test_get_pool_contract_config() {
    let config = CONFIG_CREATER.get().await;
    assert_eq!(
        config.get_pool_contract_config(3, "MTT", BridgeType::Celer, 2,),
        config
            .get_pool_contract_config_by_address(3, "0x20Eb345870059E688c59e89523442ade33C7c813",)
    );
    assert_eq!(
        config.get_pool_contract_config(3, "MTT", BridgeType::Tbridge, 2,),
        config
            .get_pool_contract_config_by_address(3, "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d",)
    );
    assert_eq!(
        config.get_pool_contract_config(3, "ETH", BridgeType::Loop, 2,),
        config
            .get_pool_contract_config_by_address(3, "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",)
    );
    assert_eq!(
        config.get_pool_contract_config(97, "MTT", BridgeType::Celer, 2,),
        config
            .get_pool_contract_config_by_address(97, "0x6B8a4ea37C72F1992626eb9bD48d4aA6aa077c47",)
    );
    assert_eq!(
        config.get_pool_contract_config(97, "MTT", BridgeType::Tbridge, 2,),
        config
            .get_pool_contract_config_by_address(97, "0xBe2C9c8a00951662dF3a978b25F448968F0595AE",)
    );
    assert_eq!(
        config
            .get_pool_contract_config(1024, "MTT", BridgeType::Celer, 2,)
            .is_none(),
        true
    );
    assert_eq!(
        config.get_pool_contract_config_by_address(
            1024,
            "0xBe2C9c8a00951662dF3a978b25F448968F0595AE",
        ).is_none(),
        true
    );
}

#[tokio::test]
async fn test_get_pool_contract_configs() {
    let config = CONFIG_CREATER.get().await;
    assert_eq!(
        config.get_pool_contract_configs(3, "MTT", BridgeType::Celer,),
        vec![config
            .get_pool_contract_config_by_address(3, "0x20Eb345870059E688c59e89523442ade33C7c813",)
            .unwrap()
            .clone()]
    );
    let pool_contract_configs = config.get_pool_contract_configs(3, "MTT", BridgeType::Tbridge);
    assert_eq!(pool_contract_configs.len(), 2);
    for pool_contract_config in pool_contract_configs {
        assert_eq!(
            &pool_contract_config,
            config
                .get_pool_contract_config_by_address(3, pool_contract_config.address(),)
                .unwrap(),
        );
    }
    assert_eq!(
        config.get_pool_contract_configs(3, "ETH", BridgeType::Loop,),
        vec![config
            .get_pool_contract_config_by_address(3, "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",)
            .unwrap()
            .clone()]
    );
    assert_eq!(
        config.get_pool_contract_configs(97, "MTT", BridgeType::Celer,),
        vec![config
            .get_pool_contract_config_by_address(97, "0x6B8a4ea37C72F1992626eb9bD48d4aA6aa077c47",)
            .unwrap()
            .clone()]
    );
    assert_eq!(
        config.get_pool_contract_configs(97, "MTT", BridgeType::Tbridge,),
        vec![config
            .get_pool_contract_config_by_address(97, "0xBe2C9c8a00951662dF3a978b25F448968F0595AE",)
            .unwrap()
            .clone()]
    );
    assert_eq!(
        config
            .get_pool_contract_configs(1024, "MTT", BridgeType::Celer,)
            .len(),
        0
    );
}

#[tokio::test]
async fn test_get_bridge_config() {
    let config = CONFIG_CREATER.get().await;
    let raw_config = default_raw_config().await;

    let poly_bridge_config = config.get_bridge_config(BridgeType::Poly).unwrap();
    let copy_data = match poly_bridge_config {
        BridgeConfigType::PolyBridgeConfig(conf) => conf.copy_data(),
        _ => RawPolyBridgeConfig::default(),
    };
    let conf = match raw_config.bridges.get(0).unwrap() {
        RawBridgeConfigType::Poly(conf) => conf.clone(),
        _ => RawPolyBridgeConfig::default(),
    };
    assert_eq!(copy_data, conf);

    let tbridge_config = config.get_bridge_config(BridgeType::Tbridge).unwrap();
    let copy_data = match tbridge_config {
        BridgeConfigType::TBridgeConfig(conf) => conf.copy_data(),
        _ => RawTBridgeConfig::default(),
    };
    let conf = match raw_config.bridges.get(1).unwrap() {
        RawBridgeConfigType::Tbridge(conf) => conf.clone(),
        _ => RawTBridgeConfig::default(),
    };
    assert_eq!(copy_data, conf);

    let celer_config = config.get_bridge_config(BridgeType::Celer).unwrap();
    let copy_data = match celer_config {
        BridgeConfigType::CelerBridgeConfig(conf) => conf.copy_data(),
        _ => RawCelerBridgeConfig::default(),
    };
    let conf = match raw_config.bridges.get(2).unwrap() {
        RawBridgeConfigType::Celer(conf) => conf.clone(),
        _ => RawCelerBridgeConfig::default(),
    };
    assert_eq!(copy_data, conf);
}

#[tokio::test]
async fn test_get_default_circuit_config() {
    let config = CONFIG_CREATER.get().await;
    let name = config
        .get_default_circuit_config(CircuitType::Rollup1)
        .unwrap()
        .name();
    assert_eq!(name, "zokrates-1.0-rollup1");
}

#[tokio::test]
async fn test_get_circuit_config_by_name() {
    let config = CONFIG_CREATER.get().await;
    assert_eq!(
        config
            .get_circuit_config_by_name("zokrates-2.0-rollup1")
            .unwrap()
            .is_default(),
        false
    );
    assert_eq!(
        config
            .get_circuit_config_by_name("zokrates-4.0-rollup1")
            .is_none(),
        true
    );
}

#[tokio::test]
async fn test_create_from_file() {
    let new_config =
        MystikoConfig::create_from_file("tests/files/mystiko.valid01.json".to_string())
            .await
            .unwrap();
    let config = default_mystiko_config().await;
    assert_eq!(new_config.to_json_string(), config.to_json_string());
}

#[tokio::test]
async fn test_duplicate_circuit_type_default() {
    let mut raw_config = default_raw_config().await;
    raw_config.circuits.push(
        RawConfig::create_from_object::<RawCircuitConfig>(RawCircuitConfig::new(
            "zokrates-2.0-transaction2x2".to_string(),
            CircuitType::TRANSACTION2x2,
            true,
            vec!["./Transaction2x2.program.gz".to_string()],
            vec!["./Transaction2x2.abi.json".to_string()],
            vec!["./Transaction2x2.pkey.gz".to_string()],
            vec!["./Transaction2x2.vkey.gz".to_string()],
        ))
        .await
        .unwrap(),
    );
    let validate = MystikoConfig::create_from_raw(raw_config).await;
    assert_eq!(validate.is_err(), true);
    assert_eq!(
        validate.unwrap_err().errors.contains(&String::from(
            "duplicate default circuit type=TRANSACTION2x2 definition"
        )),
        true
    );
}

#[tokio::test]
async fn test_duplicate_circuit_name() {
    let mut raw_config = default_raw_config().await;
    raw_config.circuits.push(
        RawConfig::create_from_object::<RawCircuitConfig>(RawCircuitConfig::new(
            "zokrates-2.0-transaction2x2".to_string(),
            CircuitType::TRANSACTION2x2,
            true,
            vec!["./Transaction2x2.program.gz".to_string()],
            vec!["./Transaction2x2.abi.json".to_string()],
            vec!["./Transaction2x2.pkey.gz".to_string()],
            vec!["./Transaction2x2.vkey.gz".to_string()],
        ))
        .await
        .unwrap(),
    );
    raw_config.circuits.push(
        RawConfig::create_from_object::<RawCircuitConfig>(RawCircuitConfig::new(
            "zokrates-1.0-transaction2x2".to_string(),
            CircuitType::TRANSACTION2x2,
            false,
            vec!["./Transaction2x2.program.gz".to_string()],
            vec!["./Transaction2x2.abi.json".to_string()],
            vec!["./Transaction2x2.pkey.gz".to_string()],
            vec!["./Transaction2x2.vkey.gz".to_string()],
        ))
        .await
        .unwrap(),
    );
    let validate = MystikoConfig::create_from_raw(raw_config).await;
    assert_eq!(validate.is_err(), true);
}

#[tokio::test]
async fn test_missing_default_circuit() {
    let mut raw_config = default_raw_config().await;
    raw_config.circuits = raw_config.circuits[0..1].to_vec();
    let validate = MystikoConfig::create_from_raw(raw_config).await;
    assert_eq!(validate.is_err(), true);
    assert_eq!(
        validate.unwrap_err().errors.contains(&String::from(
            "missing definition of default circuit type=Rollup2"
        )),
        true
    );
}

#[tokio::test]
async fn test_duplicate_bridge_type() {
    let mut raw_config = default_raw_config().await;
    raw_config.bridges.push(RawBridgeConfigType::Tbridge(
        RawConfig::create_from_object::<RawTBridgeConfig>(RawTBridgeConfig::new(
            "TBridge #2".to_string(),
        ))
        .await
        .unwrap(),
    ));
    let validate = MystikoConfig::create_from_raw(raw_config).await;
    assert_eq!(validate.is_err(), true);
}

#[tokio::test]
async fn test_missing_bridge_definition() {
    let mut raw_config = default_raw_config().await;
    raw_config.bridges = raw_config
        .bridges
        .iter()
        .filter(|&raw| raw.bridge_type().ne(&BridgeType::Celer))
        .cloned()
        .collect();
    let validate = MystikoConfig::create_from_raw(raw_config).await;
    assert_eq!(validate.is_err(), true);
    assert_eq!(
        validate.unwrap_err().errors.contains(&String::from(
            "bridge type = Celer definition does not exist"
        )),
        true
    );
}

#[tokio::test]
async fn test_duplicate_chain_config() {
    let mut raw_config = default_raw_config().await;
    raw_config.chains.push(
        RawConfig::create_from_object::<RawChainConfig>(RawChainConfig::new(
            3,
            "Ethereum Ropsten".to_string(),
            "ETH".to_string(),
            18,
            vec![],
            "https://ropsten.etherscan.io".to_string(),
            "/tx/%tx%".to_string(),
            None,
            None,
            vec![RawProviderConfig::new(
                "http://localhost:8545".to_string(),
                None,
                None,
            )],
            "https://ropsten.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161".to_string(),
            vec![],
            vec![],
            vec![],
        ))
        .await
        .unwrap(),
    );
    let validate = MystikoConfig::create_from_raw(raw_config).await;
    assert_eq!(validate.is_err(), true);
}

#[tokio::test]
async fn test_invalid_peer_chain_id() {
    let mut raw_config = default_raw_config().await;
    raw_config.chains[0].deposit_contracts[1].peer_chain_id = Some(1024);
    let validate = MystikoConfig::create_from_raw(raw_config).await;
    assert_eq!(validate.is_err(), true);
}

#[tokio::test]
async fn test_invalid_peer_chain_address() {
    let mut raw_config = default_raw_config().await;
    raw_config.chains[0].deposit_contracts[1].peer_contract_address =
        Some("0x5c7c88e07e3899fff3cc0effe23494591dfe87b6".to_string());
    let validate = MystikoConfig::create_from_raw(raw_config).await;
    assert_eq!(validate.is_err(), true);
}

#[tokio::test]
async fn test_bridge_type_mismatch() {
    let mut raw_config = default_raw_config().await;
    raw_config.chains[0].deposit_contracts[2].bridge_type = BridgeType::Poly;
    raw_config.chains[0].pool_contracts[2].bridge_type = BridgeType::Poly;
    let validate = MystikoConfig::create_from_raw(raw_config).await;
    assert_eq!(validate.is_err(), true);
}

#[tokio::test]
async fn test_peer_chain_id_mismatch() {
    let mut raw_config = default_raw_config().await;
    raw_config.chains[1].deposit_contracts[1].peer_chain_id = Some(1024);
    let validate = MystikoConfig::create_from_raw(raw_config).await;
    assert_eq!(validate.is_err(), true);
}

#[tokio::test]
async fn test_peer_contract_address_mismatch() {
    let mut raw_config = default_raw_config().await;
    raw_config.chains[1].deposit_contracts[1].peer_contract_address =
        Some("0x5c7c88e07e3899fff3cc0effe23494591dfe87b6".to_string());
    let validate = MystikoConfig::create_from_raw(raw_config).await;
    assert_eq!(validate.is_err(), true);
}

#[tokio::test]
async fn test_create_default_testnet_config() {
    let result = MystikoConfig::create_default_testnet_config().await;
    assert!(!result.is_err());
}

#[tokio::test]
async fn test_create_default_mainnet_config() {
    let result = MystikoConfig::create_default_mainnet_config().await;
    assert!(!result.is_err());
}

#[tokio::test]
async fn test_get_transaction_url() {
    let config = CONFIG_CREATER.get().await;
    assert_eq!(
        config
            .get_transaction_url(
                1024,
                "0xbce8d733536ee3b769456cf91bebae1e9e5be6cb89bb7490c6225384e1bc5e3e",
            )
            .is_none(),
        true
    );
    assert_eq!(
        config.get_transaction_url(
            3,
            "0xbce8d733536ee3b769456cf91bebae1e9e5be6cb89bb7490c6225384e1bc5e3e",
        ).unwrap(),
        "https://ropsten.etherscan.io/tx/0xbce8d733536ee3b769456cf91bebae1e9e5be6cb89bb7490c6225384e1bc5e3e"
    );
}

#[tokio::test]
async fn test_get_indexer_config() {
    let mut raw_config = default_raw_config().await;
    raw_config.indexer = Some(
        RawConfig::create_from_file::<RawIndexerConfig>("tests/files/indexer.valid.json")
            .await
            .unwrap(),
    );
    let config = MystikoConfig::create_from_raw(raw_config).await.unwrap();
    let indexer = config.indexer();
    match indexer {
        None => {
            panic!()
        }
        Some(conf) => {
            assert_eq!(conf.url(), "https://example.com");
        }
    }
}

#[tokio::test]
async fn test_mutate() {
    let config = CONFIG_CREATER.get().await;
    let mut raw_config = default_raw_config().await;
    assert_eq!(config.mutate(None).unwrap().copy_data(), raw_config);
    raw_config.version = "1.1.1".to_string();
    let new_config = config.mutate(Some(raw_config)).unwrap();
    assert_eq!(new_config.version(), "1.1.1");
}
