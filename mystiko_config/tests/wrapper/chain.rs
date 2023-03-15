use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::common::{BridgeType, CircuitType, ContractType};
use mystiko_config::raw::base::RawConfig;
use mystiko_config::raw::chain::RawChainConfig;
use mystiko_config::raw::contract::base::RawContractConfig;
use mystiko_config::raw::contract::deposit::RawDepositContractConfig;
use mystiko_config::raw::contract::pool::RawPoolContractConfig;
use mystiko_config::raw::mystiko::RawMystikoConfig;
use mystiko_config::wrapper::chain::{AuxData, ChainConfig};
use mystiko_config::wrapper::circuit::CircuitConfig;
use mystiko_config::wrapper::provider::ProviderConfig;
use num_bigint::BigInt;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;

async fn raw_mystiko_config() -> RawMystikoConfig {
    RawConfig::create_from_file::<RawMystikoConfig>("tests/files/mystiko.valid.json")
        .await
        .unwrap()
}

async fn default_circuit_configs() -> Rc<HashMap<CircuitType, CircuitConfig>> {
    let raw = raw_mystiko_config().await;
    let mut configs = HashMap::new();
    for circuit in raw.circuits {
        let circuit_config = CircuitConfig::new(circuit.clone());
        if circuit.is_default {
            configs.insert(circuit.circuit_type, circuit_config);
        }
    }
    Rc::new(configs)
}

async fn circuit_configs_by_name() -> Rc<HashMap<String, CircuitConfig>> {
    let raw = raw_mystiko_config().await;
    let mut configs = HashMap::new();
    for circuit in raw.circuits {
        let circuit_config = CircuitConfig::new(circuit.clone());
        configs.insert(circuit.name, circuit_config);
    }
    Rc::new(configs)
}

async fn default_raw_config() -> RawChainConfig {
    RawConfig::create_from_file::<RawChainConfig>("tests/files/chain.valid.json")
        .await
        .unwrap()
}

async fn default_chain_config() -> ChainConfig {
    ChainConfig::new(
        default_raw_config().await,
        Some(AuxData::new(
            default_circuit_configs().await,
            circuit_configs_by_name().await,
            Rc::new(None),
        )),
    )
    .unwrap()
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<ChainConfig> =
        AsyncOnce::new(async { default_chain_config().await });
    static ref RAW_CONFIG_CREATER: AsyncOnce<RawChainConfig> =
        AsyncOnce::new(async { default_raw_config().await });
}

#[tokio::test]
async fn test_equality() {
    let config = CONFIG_CREATER.get().await;
    let raw_config = RAW_CONFIG_CREATER.get().await;
    assert_eq!(config.chain_id(), &raw_config.chain_id);
    assert_eq!(config.name(), raw_config.name);
    assert_eq!(config.asset_symbol(), raw_config.asset_symbol);
    assert_eq!(config.asset_decimals(), raw_config.asset_decimals);
    assert_eq!(
        config.recommend_amounts(),
        vec![
            BigInt::from_str("1000000000000000000").unwrap(),
            BigInt::from_str("10000000000000000000").unwrap(),
        ]
    );
    assert_eq!(config.recommend_amounts_numbers(), vec![1f64, 10f64]);
    assert_eq!(config.explorer_url(), raw_config.explorer_url);
    assert_eq!(config.explorer_url_prefix(), raw_config.explorer_prefix);
    assert_eq!(config.signer_endpoint(), raw_config.signer_endpoint);
    assert_eq!(config.event_filter_size(), &raw_config.event_filter_size);
    assert_eq!(
        config.indexer_filter_size(),
        &raw_config.indexer_filter_size
    );
    assert_eq!(
        config.providers(),
        &raw_config
            .providers
            .iter()
            .map(|raw| ProviderConfig::new(raw.clone()))
            .collect::<Vec<ProviderConfig>>()
    );
    assert_eq!(
        config.pool_contracts().len(),
        raw_config.pool_contracts.len()
    );
    let mut a = config
        .pool_contracts()
        .iter()
        .map(|conf| conf.address().to_string())
        .collect::<Vec<String>>();
    let mut b = raw_config
        .pool_contracts
        .iter()
        .map(|conf| conf.base.address.clone())
        .collect::<Vec<String>>();
    a.sort();
    b.sort();
    assert_eq!(a, b);
    assert_eq!(config.deposit_contracts().len(), 0);
    assert_eq!(
        config.deposit_contracts_with_disabled().len(),
        raw_config.deposit_contracts.len()
    );
    let mut a = config
        .deposit_contracts_with_disabled()
        .iter()
        .map(|conf| conf.address().to_string())
        .collect::<Vec<String>>();
    let mut b = raw_config
        .deposit_contracts
        .iter()
        .map(|conf| conf.base.address.clone())
        .collect::<Vec<String>>();
    a.sort();
    b.sort();
    assert_eq!(a, b);
    assert_eq!(config.assets().len(), 1);
}

#[tokio::test]
async fn test_peer_chain_ids() {
    let mut config = default_chain_config().await;
    assert_eq!(config.peer_chain_ids().len(), 0);
    let mut raw_config = default_raw_config().await;
    let mut deposit_contracts = raw_config.deposit_contracts;
    deposit_contracts[0].disabled = false;
    raw_config.deposit_contracts = deposit_contracts;

    let default_circuit_configs = default_circuit_configs().await;
    let circuit_configs_by_name = circuit_configs_by_name().await;
    config = ChainConfig::new(
        raw_config.clone(),
        Some(AuxData::new(
            default_circuit_configs.clone(),
            circuit_configs_by_name.clone(),
            Rc::new(None),
        )),
    )
    .unwrap();
    assert_eq!(config.peer_chain_ids(), vec![97]);

    let loop_deposit_contract_config =
        RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
            RawContractConfig::new(
                2,
                "MystikoWithPolyERC20".to_string(),
                "0x2f0Fe3154C281Cb25D6a615bf524230e57A462e1".to_string(),
                ContractType::Deposit,
                1000000,
                None,
                None,
            ),
            BridgeType::Loop,
            "0x20Eb345870059E688c59e89523442ade33C7c813".to_string(),
            false,
            None,
            None,
            "10000000000000000".to_string(),
            "100000000000000000".to_string(),
            "20000000000000000".to_string(),
            "30000000000000000".to_string(),
            None,
            None,
            None,
            None,
        ))
        .await
        .unwrap();
    let pool_contract_config =
        RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
            RawContractConfig::new(
                2,
                "CommitmentPool".to_string(),
                "0x20Eb345870059E688c59e89523442ade33C7c813".to_string(),
                ContractType::Pool,
                1000000,
                None,
                None,
            ),
            "A Pool".to_string(),
            BridgeType::Loop,
            Some(String::from("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")),
            "40000000000000000".to_string(),
            vec![],
        ))
        .await
        .unwrap();
    raw_config
        .deposit_contracts
        .push(loop_deposit_contract_config);
    raw_config.pool_contracts.push(pool_contract_config);
    config = ChainConfig::new(
        raw_config,
        Some(AuxData::new(
            default_circuit_configs,
            circuit_configs_by_name,
            Rc::new(None),
        )),
    )
    .unwrap();
    let mut a = config.peer_chain_ids();
    a.sort();
    assert_eq!(a, vec![3, 97]);
}

#[tokio::test]
async fn test_get_asset_symbols() {
    let mut config = default_chain_config().await;
    assert_eq!(config.get_asset_symbols(97).unwrap().len(), 0);
    let mut raw_config = default_raw_config().await;
    let mut deposit_contracts = raw_config.deposit_contracts;
    deposit_contracts[0].disabled = false;
    raw_config.deposit_contracts = deposit_contracts;

    let default_circuit_configs = default_circuit_configs().await;
    let circuit_configs_by_name = circuit_configs_by_name().await;
    config = ChainConfig::new(
        raw_config.clone(),
        Some(AuxData::new(
            default_circuit_configs.clone(),
            circuit_configs_by_name.clone(),
            Rc::new(None),
        )),
    )
    .unwrap();
    assert_eq!(
        config.get_asset_symbols(97).unwrap(),
        vec!["MTT".to_string()]
    );

    let pool_contract_config1 =
        RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
            RawContractConfig::new(
                2,
                "CommitmentPool".to_string(),
                "0x954c6c78A2F93E6E19Ff1DE538F720311414530c".to_string(),
                ContractType::Pool,
                1000000,
                None,
                None,
            ),
            "A Pool".to_string(),
            BridgeType::Loop,
            None,
            "40000000000000000".to_string(),
            vec![],
        ))
        .await
        .unwrap();
    let pool_contract_config2 =
        RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
            RawContractConfig::new(
                2,
                "CommitmentPool".to_string(),
                "0x20Eb345870059E688c59e89523442ade33C7c813".to_string(),
                ContractType::Pool,
                1000000,
                None,
                None,
            ),
            "A Pool".to_string(),
            BridgeType::Tbridge,
            None,
            "40000000000000000".to_string(),
            vec![],
        ))
        .await
        .unwrap();
    let loop_deposit_contract_config1 =
        RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
            RawContractConfig::new(
                2,
                "MystikoWithPolyERC20".to_string(),
                "0x2f0Fe3154C281Cb25D6a615bf524230e57A462e1".to_string(),
                ContractType::Deposit,
                1000000,
                None,
                None,
            ),
            BridgeType::Loop,
            "0x954c6c78A2F93E6E19Ff1DE538F720311414530c".to_string(),
            false,
            None,
            None,
            "10000000000000000".to_string(),
            "100000000000000000".to_string(),
            "20000000000000000".to_string(),
            "30000000000000000".to_string(),
            None,
            None,
            None,
            None,
        ))
        .await
        .unwrap();
    let loop_deposit_contract_config2 =
        RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
            RawContractConfig::new(
                2,
                "MystikoWithPolyERC20".to_string(),
                "0x0b1d6565d88f9bf6473e21c2ab58d28a495d7bb5".to_string(),
                ContractType::Deposit,
                1000000,
                None,
                None,
            ),
            BridgeType::Tbridge,
            "0x20Eb345870059E688c59e89523442ade33C7c813".to_string(),
            false,
            Some(97),
            Some(String::from("0x390de26d772d2e2005c6d1d24afc902bae37a4bb")),
            "10000000000000000".to_string(),
            "100000000000000000".to_string(),
            "20000000000000000".to_string(),
            "30000000000000000".to_string(),
            None,
            None,
            None,
            None,
        ))
        .await
        .unwrap();
    raw_config
        .deposit_contracts
        .push(loop_deposit_contract_config1);
    raw_config
        .deposit_contracts
        .push(loop_deposit_contract_config2);
    raw_config.pool_contracts.push(pool_contract_config1);
    raw_config.pool_contracts.push(pool_contract_config2);
    config = ChainConfig::new(
        raw_config,
        Some(AuxData::new(
            default_circuit_configs,
            circuit_configs_by_name,
            Rc::new(None),
        )),
    )
    .unwrap();
    let mut a = config.get_asset_symbols(97).unwrap();
    a.sort();
    assert_eq!(a, vec![String::from("ETH"), String::from("MTT")]);
    assert_eq!(
        config.get_asset_symbols(3).unwrap(),
        vec![String::from("ETH")]
    );
}

#[tokio::test]
async fn test_get_bridges() {
    let mut config = default_chain_config().await;
    let mut raw_config = default_raw_config().await;
    assert_eq!(config.get_bridges(97, "MTT").unwrap().len(), 0);
    let mut deposit_contracts = raw_config.deposit_contracts;
    deposit_contracts[0].disabled = false;
    raw_config.deposit_contracts = deposit_contracts;

    let default_circuit_configs = default_circuit_configs().await;
    let circuit_configs_by_name = circuit_configs_by_name().await;
    config = ChainConfig::new(
        raw_config.clone(),
        Some(AuxData::new(
            default_circuit_configs.clone(),
            circuit_configs_by_name.clone(),
            Rc::new(None),
        )),
    )
    .unwrap();
    assert_eq!(
        config.get_bridges(97, "MTT").unwrap(),
        vec![BridgeType::Tbridge]
    );
    let loop_deposit_contract_config =
        RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
            RawContractConfig::new(
                2,
                "MystikoWithPolyERC20".to_string(),
                "0x2f0Fe3154C281Cb25D6a615bf524230e57A462e1".to_string(),
                ContractType::Deposit,
                1000000,
                None,
                None,
            ),
            BridgeType::Loop,
            "0x6b8a4ea37c72f1992626eb9bd48d4aa6aa077c47".to_string(),
            false,
            None,
            None,
            "10000000000000000".to_string(),
            "100000000000000000".to_string(),
            "20000000000000000".to_string(),
            "30000000000000000".to_string(),
            None,
            None,
            None,
            None,
        ))
        .await
        .unwrap();
    let pool_contract_config =
        RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
            RawContractConfig::new(
                2,
                "CommitmentPool".to_string(),
                "0x6b8a4ea37c72f1992626eb9bd48d4aa6aa077c47".to_string(),
                ContractType::Pool,
                1000000,
                None,
                None,
            ),
            "A Pool".to_string(),
            BridgeType::Loop,
            Some(String::from("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")),
            "40000000000000000".to_string(),
            vec![],
        ))
        .await
        .unwrap();
    raw_config
        .deposit_contracts
        .push(loop_deposit_contract_config);
    raw_config.pool_contracts.push(pool_contract_config);
    config = ChainConfig::new(
        raw_config.clone(),
        Some(AuxData::new(
            default_circuit_configs.clone(),
            circuit_configs_by_name.clone(),
            Rc::new(None),
        )),
    )
    .unwrap();
    assert_eq!(config.get_bridges(3, "MTT").unwrap().len(), 0);
    let celer_deposit_contract_config =
        RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
            RawContractConfig::new(
                2,
                "MystikoWithPolyERC20".to_string(),
                "0x4c55C41Bd839B3552fb2AbecaCFdF4a5D2879Cb9".to_string(),
                ContractType::Deposit,
                1000000,
                None,
                None,
            ),
            BridgeType::Celer,
            "0x20Eb345870059E688c59e89523442ade33C7c813".to_string(),
            false,
            Some(97),
            Some(String::from("0x390de26d772d2e2005c6d1d24afc902bae37a4bb")),
            "10000000000000000".to_string(),
            "100000000000000000".to_string(),
            "20000000000000000".to_string(),
            "30000000000000000".to_string(),
            None,
            None,
            None,
            None,
        ))
        .await
        .unwrap();
    let pool_contract_config2 =
        RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
            RawContractConfig::new(
                2,
                "CommitmentPool".to_string(),
                "0x20Eb345870059E688c59e89523442ade33C7c813".to_string(),
                ContractType::Deposit,
                1000000,
                None,
                None,
            ),
            "A Pool".to_string(),
            BridgeType::Celer,
            Some(String::from("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")),
            "40000000000000000".to_string(),
            vec![],
        ))
        .await
        .unwrap();
    raw_config
        .deposit_contracts
        .push(celer_deposit_contract_config);
    raw_config.pool_contracts.push(pool_contract_config2);
    config = ChainConfig::new(
        raw_config.clone(),
        Some(AuxData::new(
            default_circuit_configs.clone(),
            circuit_configs_by_name.clone(),
            Rc::new(None),
        )),
    )
    .unwrap();
    let a = config.get_bridges(97, "MTT").unwrap();
    assert_eq!(a.contains(&BridgeType::Celer), true);
    assert_eq!(a.contains(&BridgeType::Tbridge), true);
}

#[tokio::test]
async fn test_get_deposit_contract() {
    let mut config = default_chain_config().await;
    assert_eq!(
        config
            .get_deposit_contract(97, "MTT", BridgeType::Tbridge)
            .unwrap()
            .is_none(),
        true
    );
    let tbridge_deposit_contract_config =
        RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
            RawContractConfig::new(
                2,
                "MystikoWithPolyERC20".to_string(),
                "0x4c55C41Bd839B3552fb2AbecaCFdF4a5D2879Cb9".to_string(),
                ContractType::Deposit,
                1000000,
                None,
                None,
            ),
            BridgeType::Tbridge,
            "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d".to_string(),
            false,
            Some(97),
            Some(String::from("0x390de26d772d2e2005c6d1d24afc902bae37a4bb")),
            "10000000000000000".to_string(),
            "100000000000000000".to_string(),
            "20000000000000000".to_string(),
            "30000000000000000".to_string(),
            None,
            None,
            None,
            None,
        ))
        .await
        .unwrap();
    let loop_deposit_contract_config =
        RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
            RawContractConfig::new(
                2,
                "MystikoWithPolyERC20".to_string(),
                "0x2f0Fe3154C281Cb25D6a615bf524230e57A462e1".to_string(),
                ContractType::Deposit,
                1000000,
                None,
                None,
            ),
            BridgeType::Loop,
            "0x20Eb345870059E688c59e89523442ade33C7c813".to_string(),
            false,
            None,
            None,
            "10000000000000000".to_string(),
            "100000000000000000".to_string(),
            "20000000000000000".to_string(),
            "30000000000000000".to_string(),
            None,
            None,
            None,
            None,
        ))
        .await
        .unwrap();
    let pool_contract_config =
        RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
            RawContractConfig::new(
                2,
                "CommitmentPool".to_string(),
                "0x20Eb345870059E688c59e89523442ade33C7c813".to_string(),
                ContractType::Pool,
                1000000,
                None,
                None,
            ),
            "A Pool".to_string(),
            BridgeType::Loop,
            Some(String::from("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")),
            "40000000000000000".to_string(),
            vec![],
        ))
        .await
        .unwrap();
    let mut raw_config = default_raw_config().await;
    raw_config
        .deposit_contracts
        .push(tbridge_deposit_contract_config);
    raw_config
        .deposit_contracts
        .push(loop_deposit_contract_config);
    raw_config.pool_contracts.push(pool_contract_config);
    let default_circuit_configs = default_circuit_configs().await;
    let circuit_configs_by_name = circuit_configs_by_name().await;
    config = ChainConfig::new(
        raw_config.clone(),
        Some(AuxData::new(
            default_circuit_configs,
            circuit_configs_by_name,
            Rc::new(None),
        )),
    )
    .unwrap();
    assert_eq!(
        config
            .get_deposit_contract(97, "MTT", BridgeType::Tbridge,)
            .unwrap()
            .unwrap()
            .address(),
        "0x4c55C41Bd839B3552fb2AbecaCFdF4a5D2879Cb9"
    );
    assert_eq!(
        config
            .get_deposit_contract(3, "MTT", BridgeType::Loop,)
            .unwrap()
            .unwrap()
            .address(),
        "0x2f0Fe3154C281Cb25D6a615bf524230e57A462e1"
    );
    assert_eq!(
        config
            .get_deposit_contract_by_address(
                "0x2f0Fe3154C281Cb25D6a615bf524230e57A462e1".to_string()
            )
            .unwrap()
            .pool_address(),
        "0x20Eb345870059E688c59e89523442ade33C7c813"
    );
    assert_eq!(
        config
            .get_deposit_contract_by_address(
                "0x5380442d3c4ec4f5777f551f5edd2fa0f691a27c".to_string()
            )
            .is_none(),
        true
    );
}

#[tokio::test]
async fn test_get_pool_contract() {
    let tbridge_deposit_contract_config =
        RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
            RawContractConfig::new(
                2,
                "MystikoWithPolyERC20".to_string(),
                "0x4c55C41Bd839B3552fb2AbecaCFdF4a5D2879Cb9".to_string(),
                ContractType::Deposit,
                1000000,
                None,
                None,
            ),
            BridgeType::Tbridge,
            "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d".to_string(),
            false,
            Some(100),
            Some(String::from("0x390de26d772d2e2005c6d1d24afc902bae37a4bb")),
            "10000000000000000".to_string(),
            "100000000000000000".to_string(),
            "20000000000000000".to_string(),
            "30000000000000000".to_string(),
            None,
            None,
            None,
            None,
        ))
        .await
        .unwrap();
    let loop_deposit_contract_config =
        RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
            RawContractConfig::new(
                2,
                "MystikoWithPolyERC20".to_string(),
                "0x2f0Fe3154C281Cb25D6a615bf524230e57A462e1".to_string(),
                ContractType::Deposit,
                1000000,
                None,
                None,
            ),
            BridgeType::Loop,
            "0x954c6c78A2F93E6E19Ff1DE538F720311414530c".to_string(),
            false,
            None,
            None,
            "10000000000000000".to_string(),
            "100000000000000000".to_string(),
            "20000000000000000".to_string(),
            "30000000000000000".to_string(),
            None,
            None,
            None,
            None,
        ))
        .await
        .unwrap();
    let pool_contract_config1 =
        RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
            RawContractConfig::new(
                1,
                "CommitmentPool".to_string(),
                "0x81b7e08f65bdf5648606c89998a9cc8164397647".to_string(),
                ContractType::Pool,
                1000000,
                None,
                None,
            ),
            "A Pool".to_string(),
            BridgeType::Loop,
            None,
            "40000000000000000".to_string(),
            vec![],
        ))
        .await
        .unwrap();
    let pool_contract_config2 =
        RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
            RawContractConfig::new(
                2,
                "CommitmentPool".to_string(),
                "0x954c6c78A2F93E6E19Ff1DE538F720311414530c".to_string(),
                ContractType::Pool,
                1000000,
                None,
                None,
            ),
            "A Pool".to_string(),
            BridgeType::Loop,
            None,
            "40000000000000000".to_string(),
            vec![],
        ))
        .await
        .unwrap();
    let mut raw_config = default_raw_config().await;
    raw_config.pool_contracts.push(pool_contract_config1);
    raw_config.pool_contracts.push(pool_contract_config2);
    raw_config
        .deposit_contracts
        .push(tbridge_deposit_contract_config);
    raw_config
        .deposit_contracts
        .push(loop_deposit_contract_config);
    raw_config.deposit_contracts[0].disabled = false;
    let default_circuit_configs = default_circuit_configs().await;
    let circuit_configs_by_name = circuit_configs_by_name().await;
    let config = ChainConfig::new(
        raw_config.clone(),
        Some(AuxData::new(
            default_circuit_configs.clone(),
            circuit_configs_by_name.clone(),
            Rc::new(None),
        )),
    )
    .unwrap();
    assert_eq!(
        config
            .get_pool_contract("MTT", BridgeType::Tbridge, 2,)
            .unwrap()
            .address(),
        "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
    );
    assert_eq!(
        config
            .get_pool_contract("MTT", BridgeType::Tbridge, 3,)
            .is_none(),
        true
    );
    assert_eq!(
        config
            .get_pool_contract("mUSD", BridgeType::Tbridge, 2,)
            .is_none(),
        true
    );
    assert_eq!(
        config
            .get_pool_contract("MTT", BridgeType::Loop, 2,)
            .is_none(),
        true
    );
    assert_eq!(
        config
            .get_pool_contract("ETH", BridgeType::Loop, 1,)
            .unwrap()
            .address(),
        "0x81b7e08f65bdf5648606c89998a9cc8164397647"
    );
    assert_eq!(
        config
            .get_pool_contract("ETH", BridgeType::Loop, 2,)
            .unwrap()
            .address(),
        "0x954c6c78A2F93E6E19Ff1DE538F720311414530c"
    );
    assert_eq!(
        config
            .get_pool_contracts("mUSD", BridgeType::Tbridge,)
            .len(),
        0
    );
    assert_eq!(config.get_pool_contracts("MTT", BridgeType::Loop,).len(), 0);
    assert_eq!(
        config
            .get_pool_contracts("ETH", BridgeType::Loop,)
            .iter()
            .map(|c| c.address())
            .collect::<Vec<&str>>()
            .contains(&"0x954c6c78A2F93E6E19Ff1DE538F720311414530c"),
        true
    );
    assert_eq!(
        config
            .get_pool_contracts("ETH", BridgeType::Loop,)
            .iter()
            .map(|c| c.address())
            .collect::<Vec<&str>>()
            .contains(&"0x81b7e08f65bdf5648606c89998a9cc8164397647"),
        true
    );
    assert_eq!(
        config
            .get_pool_contracts("ETH", BridgeType::Loop,)
            .iter()
            .map(|c| c.address())
            .collect::<Vec<&str>>()
            .len(),
        2
    );
    assert_eq!(
        config
            .get_pool_contract_by_address("0x954c6c78A2F93E6E19Ff1DE538F720311414530c")
            .unwrap()
            .asset_symbol(),
        "ETH".to_string()
    );
    assert_eq!(
        config
            .get_pool_contract_by_address("0x5380442d3c4ec4f5777f551f5edd2fa0f691a27c")
            .is_none(),
        true
    );
}

#[tokio::test]
async fn test_get_pool_contract_bridge_type() {
    let config = CONFIG_CREATER.get().await;
    assert_eq!(
        config
            .get_pool_contract_bridge_type("0x721d424047d3a8dd20f7a88f2eadad16fd2fab51")
            .is_none(),
        true
    );
    assert_eq!(
        config
            .get_pool_contract_bridge_type("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d")
            .unwrap(),
        BridgeType::Tbridge
    );
}

#[tokio::test]
async fn test_get_event_filter_size_by_address() {
    let mut raw_config = default_raw_config().await;
    raw_config.event_filter_size = 12345;
    let default_circuit_configs = default_circuit_configs().await;
    let circuit_configs_by_name = circuit_configs_by_name().await;
    let mut config = ChainConfig::new(
        raw_config.clone(),
        Some(AuxData::new(
            default_circuit_configs.clone(),
            circuit_configs_by_name.clone(),
            Rc::new(None),
        )),
    )
    .unwrap();
    assert_eq!(
        config.get_event_filter_size_by_address("0x5380442d3c4ec4f5777f551f5edd2fa0f691a27c"),
        12345
    );
    assert_eq!(
        config.get_event_filter_size_by_address("0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"),
        12345
    );
    assert_eq!(
        config.get_event_filter_size_by_address("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"),
        12345
    );
    raw_config.deposit_contracts[0].base.event_filter_size = Some(87654321);
    raw_config.pool_contracts[0].base.event_filter_size = Some(987654321);
    config = ChainConfig::new(
        raw_config.clone(),
        Some(AuxData::new(
            default_circuit_configs.clone(),
            circuit_configs_by_name.clone(),
            Rc::new(None),
        )),
    )
    .unwrap();
    assert_eq!(
        config.get_event_filter_size_by_address("0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"),
        87654321
    );
    assert_eq!(
        config.get_event_filter_size_by_address("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"),
        987654321
    );
}

#[tokio::test]
async fn test_get_indexer_filter_size_by_address() {
    let mut raw_config = default_raw_config().await;
    raw_config.indexer_filter_size = 123450;
    let default_circuit_configs = default_circuit_configs().await;
    let circuit_configs_by_name = circuit_configs_by_name().await;
    let config = ChainConfig::new(
        raw_config.clone(),
        Some(AuxData::new(
            default_circuit_configs.clone(),
            circuit_configs_by_name.clone(),
            Rc::new(None),
        )),
    )
    .unwrap();
    assert_eq!(
        config.get_indexer_filter_size_by_address("0x5380442d3c4ec4f5777f551f5edd2fa0f691a27c"),
        123450
    );
    assert_eq!(
        config.get_indexer_filter_size_by_address("0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"),
        123450
    );
    assert_eq!(
        config.get_indexer_filter_size_by_address("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"),
        123450
    );
    raw_config.deposit_contracts[0].base.indexer_filter_size = Some(876543210);
    raw_config.pool_contracts[0].base.indexer_filter_size = Some(9876543210);
    let config = ChainConfig::new(
        raw_config.clone(),
        Some(AuxData::new(
            default_circuit_configs.clone(),
            circuit_configs_by_name.clone(),
            Rc::new(None),
        )),
    )
    .unwrap();
    assert_eq!(
        config.get_indexer_filter_size_by_address("0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"),
        876543210
    );
    assert_eq!(
        config.get_indexer_filter_size_by_address("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"),
        9876543210
    );
}

#[tokio::test]
async fn test_invalid_pool_address() {
    let mut raw_config = default_raw_config().await;
    raw_config.deposit_contracts[0].pool_address =
        "0x5380442d3c4ec4f5777f551f5edd2fa0f691a27c".to_string();
    let default_circuit_configs = default_circuit_configs().await;
    let circuit_configs_by_name = circuit_configs_by_name().await;
    let result = ChainConfig::new(
        raw_config,
        Some(AuxData::new(
            default_circuit_configs,
            circuit_configs_by_name,
            Rc::new(None),
        )),
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().errors.contains(
        &"deposit contract=0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 \
       poolAddress definition does not exist"
            .to_string()
    ));
}

#[tokio::test]
async fn test_duplicate_bridge_and_asset() {
    let pool_contract_config =
        RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
            RawContractConfig::new(
                2,
                "CommitmentPool".to_string(),
                "0x954c6c78A2F93E6E19Ff1DE538F720311414530c".to_string(),
                ContractType::Pool,
                1000000,
                None,
                None,
            ),
            "A Pool".to_string(),
            BridgeType::Tbridge,
            Some(String::from("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")),
            "40000000000000000".to_string(),
            vec![],
        ))
        .await
        .unwrap();
    let tbridge_deposit_contract_config =
        RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
            RawContractConfig::new(
                2,
                "MystikoWithPolyERC20".to_string(),
                "0x4c55C41Bd839B3552fb2AbecaCFdF4a5D2879Cb9".to_string(),
                ContractType::Deposit,
                1000000,
                None,
                None,
            ),
            BridgeType::Tbridge,
            "0x954c6c78A2F93E6E19Ff1DE538F720311414530c".to_string(),
            false,
            Some(100),
            Some(String::from("0x390de26d772d2e2005c6d1d24afc902bae37a4bb")),
            "10000000000000000".to_string(),
            "100000000000000000".to_string(),
            "20000000000000000".to_string(),
            "30000000000000000".to_string(),
            None,
            None,
            None,
            None,
        ))
        .await
        .unwrap();
    let mut raw_config = default_raw_config().await;
    raw_config.pool_contracts.push(pool_contract_config);
    raw_config
        .deposit_contracts
        .push(tbridge_deposit_contract_config);
    raw_config.deposit_contracts[0].disabled = false;
    let default_circuit_configs = default_circuit_configs().await;
    let circuit_configs_by_name = circuit_configs_by_name().await;
    let result = ChainConfig::new(
        raw_config.clone(),
        Some(AuxData::new(
            default_circuit_configs.clone(),
            circuit_configs_by_name.clone(),
            Rc::new(None),
        )),
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().errors.contains(
        &"only one pool address allowed for asset MTT \
        and bridge type Tbridge and version 2"
            .to_string()
    ))
}

#[tokio::test]
async fn test_different_bridge_with_same_pool_address() {
    let mut config = default_chain_config().await;
    assert_eq!(config.peer_chain_ids().len(), 0);
    let mut raw_config = default_raw_config().await;
    raw_config.deposit_contracts[0].disabled = false;
    let default_circuit_configs = default_circuit_configs().await;
    let circuit_configs_by_name = circuit_configs_by_name().await;
    config = ChainConfig::new(
        raw_config.clone(),
        Some(AuxData::new(
            default_circuit_configs.clone(),
            circuit_configs_by_name.clone(),
            Rc::new(None),
        )),
    )
    .unwrap();
    assert_eq!(config.peer_chain_ids(), vec![97]);
    let loop_deposit_contract_config =
        RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
            RawContractConfig::new(
                2,
                "MystikoWithPolyERC20".to_string(),
                "0x2f0Fe3154C281Cb25D6a615bf524230e57A462e1".to_string(),
                ContractType::Deposit,
                1000000,
                None,
                None,
            ),
            BridgeType::Loop,
            "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d".to_string(),
            false,
            None,
            None,
            "10000000000000000".to_string(),
            "100000000000000000".to_string(),
            "20000000000000000".to_string(),
            "30000000000000000".to_string(),
            None,
            None,
            None,
            None,
        ))
        .await
        .unwrap();
    raw_config
        .deposit_contracts
        .push(loop_deposit_contract_config);
    let result = ChainConfig::new(
        raw_config.clone(),
        Some(AuxData::new(
            default_circuit_configs.clone(),
            circuit_configs_by_name.clone(),
            Rc::new(None),
        )),
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().errors.contains(
        &"deposit contract=0x2f0Fe3154C281Cb25D6a615bf524230e57A462e1 bridgeType=Loop does \
        not equal to pool contract bridgeType=Tbridge"
            .to_string()
    ));
}

#[tokio::test]
async fn test_get_asset_config_by_address() {
    let config = CONFIG_CREATER.get().await;
    assert_eq!(
        config
            .get_asset_config_by_address("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")
            .unwrap()
            .asset_decimals(),
        16
    );
    assert_eq!(
        config
            .get_asset_config_by_address("0xBc28029D248FC60bce0bAC01cF41A53aEEaE06F9")
            .is_none(),
        true
    );
}

#[tokio::test]
async fn test_get_transaction_url() {
    let config = CONFIG_CREATER.get().await;
    assert_eq!(
        config.get_transaction_url(
            "0xbce8d733536ee3b769456cf91bebae1e9e5be6cb89bb7490c6225384e1bc5e3e"
        ),
        String::from(
            "https://ropsten.etherscan.io/tx/0xbce8d733536ee3b769456cf91bebae1e9e5be6cb89bb7490c6225384e1bc5e3e"
        )
    );
}

#[tokio::test]
async fn test_copy() {
    let config = CONFIG_CREATER.get().await;
    let raw_config = default_raw_config().await;
    assert_eq!(config.copy_data(), raw_config);
}

#[tokio::test]
async fn test_mutate() {
    let config = CONFIG_CREATER.get().await;
    let mut raw_config = default_raw_config().await;
    assert_eq!(config.mutate(None, None).unwrap().copy_data(), raw_config);
    raw_config.name = "another name".to_string();
    let mut new_config = config.mutate(Some(raw_config.clone()), None).unwrap();
    assert_eq!(new_config.name(), "another name");
    let default_circuit_configs = default_circuit_configs().await;
    let circuit_configs_by_name = circuit_configs_by_name().await;
    new_config = config
        .mutate(
            Some(raw_config.clone()),
            Some(AuxData::new(
                default_circuit_configs.clone(),
                circuit_configs_by_name.clone(),
                Rc::new(None),
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
        RawConfig::create_from_json_string::<RawChainConfig>(json_string.as_str())
            .await
            .unwrap();
    assert_eq!(loaded_raw_config, default_raw_config().await);
}
