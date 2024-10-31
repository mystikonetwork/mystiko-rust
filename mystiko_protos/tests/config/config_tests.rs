use mystiko_config::MAIN_ASSET_ADDRESS;
use mystiko_protos::common::v1::CircuitType;
use mystiko_protos::config::v1::{
    AssetConfig, ChainConfig, CircuitConfig, MystikoConfig, PackerConfig, ProviderConfig,
};
use mystiko_protos::core::v1::{PackerChecksum, PackerCompression};

#[tokio::test]
async fn test_asset_config_to_proto() {
    let mystiko_config = mystiko_config::MystikoConfig::from_json_file("tests/files/config/mystiko.json")
        .await
        .unwrap();
    let asset_config: AssetConfig = mystiko_config
        .find_chain(5)
        .unwrap()
        .find_asset("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(asset_config.asset_type(), mystiko_protos::common::v1::AssetType::Erc20);
    assert_eq!(&asset_config.asset_symbol, "MTT");
    assert_eq!(asset_config.asset_decimals, 16);
    assert!(asset_config.asset_address.is_some());
    assert_eq!(
        &asset_config.asset_address.unwrap(),
        "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"
    );
    assert_eq!(asset_config.recommended_amounts.len(), 2);
    assert_eq!(
        asset_config.recommended_amounts,
        vec![
            String::from("1000000000000000000"),
            String::from("10000000000000000000")
        ]
    );
}

#[tokio::test]
async fn test_circuit_config_to_proto() {
    let mystiko_config = mystiko_config::MystikoConfig::from_json_file("tests/files/config/mystiko.json")
        .await
        .unwrap();
    let circuit_config: CircuitConfig = mystiko_config
        .find_circuit("zokrates-1.0-rollup1")
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(&circuit_config.name, "zokrates-1.0-rollup1");
    assert_eq!(circuit_config.circuit_type(), CircuitType::Rollup1);
    assert!(circuit_config.is_default);
    assert_eq!(circuit_config.program_file, vec![String::from("./Rollup1.program.gz")]);
    assert_eq!(circuit_config.abi_file, vec![String::from("./Rollup1.abi.json")]);
    assert_eq!(circuit_config.proving_key_file, vec![String::from("./Rollup1.pkey.gz")]);
    assert_eq!(
        circuit_config.verifying_key_file,
        vec![String::from("./Rollup1.vkey.gz")]
    );
}

#[tokio::test]
async fn test_packer_config_to_proto() {
    let mystiko_config = mystiko_config::MystikoConfig::from_json_file("tests/files/config/mystiko.json")
        .await
        .unwrap();
    let packer_config: PackerConfig = mystiko_config.packer().unwrap().try_into().unwrap();
    assert_eq!(&packer_config.url, "https://static.mystiko.network/packer/v1");
    assert_eq!(packer_config.client_timeout_ms, 10000);
    assert_eq!(packer_config.version, 2);
    assert_eq!(packer_config.checksum(), PackerChecksum::Sha512);
    assert_eq!(packer_config.compression(), PackerCompression::Zstd);
}

#[tokio::test]
async fn test_provider_config_to_proto() {
    let mystiko_config = mystiko_config::MystikoConfig::from_json_file("tests/files/config/mystiko.json")
        .await
        .unwrap();
    let provider_config: ProviderConfig = mystiko_config
        .find_chain(5)
        .unwrap()
        .providers()
        .remove(0)
        .try_into()
        .unwrap();
    assert_eq!(
        &provider_config.url,
        "wss://goerli.infura.io/ws/v3/9aa3d95b3bc440fa88ea12eaa4456161"
    );
    assert_eq!(provider_config.timeout_ms, 100000);
    assert_eq!(provider_config.max_try_count, 5);
    assert_eq!(provider_config.quorum_weight, 2);
}

#[tokio::test]
async fn test_chain_config_to_proto() {
    let mystiko_config = mystiko_config::MystikoConfig::from_json_file("tests/files/config/mystiko.json")
        .await
        .unwrap();
    let chain_config: ChainConfig = mystiko_config.find_chain(5).unwrap().try_into().unwrap();
    assert_eq!(chain_config.chain_id, 5);
    assert_eq!(&chain_config.name, "Ethereum Goerli");
    assert_eq!(&chain_config.asset_symbol, "ETH");
    assert_eq!(chain_config.asset_decimals, 18);
    assert_eq!(&chain_config.explorer_url, "https://goerli.etherscan.io");
    assert_eq!(&chain_config.explorer_api_url, "https://api-goerli.etherscan.io");
    assert_eq!(&chain_config.explorer_prefix, "/tx/%tx%");
    assert_eq!(chain_config.provider_quorum_percentage, 80);
    assert_eq!(
        &chain_config.signer_endpoint,
        "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161"
    );
    assert_eq!(chain_config.event_delay_blocks, 200);
    assert_eq!(chain_config.event_filter_size, 1000);
    assert_eq!(chain_config.sequencer_fetch_size, 20000);
    assert!(chain_config.main_asset_config.is_some());
    assert_eq!(
        chain_config.provider_type(),
        mystiko_protos::common::v1::ProviderType::Quorum
    );
    assert_eq!(
        chain_config.transaction_type(),
        mystiko_protos::common::v1::TransactionType::Legacy
    );
    assert_eq!(chain_config.provider_configs.len(), 2);
    assert_eq!(chain_config.deposit_contract_configs.len(), 6);
    assert_eq!(chain_config.asset_configs.len(), 1);
    assert_eq!(chain_config.pool_contract_configs.len(), 6);
    assert_eq!(chain_config.granularities, vec![2000, 4000, 8000, 16000]);
    let main_asset = chain_config.main_asset_config.unwrap();
    assert_eq!(main_asset.asset_address(), MAIN_ASSET_ADDRESS);
    assert_eq!(main_asset.asset_type(), mystiko_protos::common::v1::AssetType::Main);
    assert_eq!(&main_asset.asset_symbol, "ETH");
    assert_eq!(main_asset.asset_decimals, 18);
    assert_eq!(
        main_asset.recommended_amounts,
        vec![
            String::from("1000000000000000000"),
            String::from("10000000000000000000"),
        ]
    );
    assert_eq!(main_asset.recommended_amounts, chain_config.recommended_amounts);
    assert!(chain_config
        .asset_configs
        .contains_key("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"));
    assert!(chain_config
        .pool_contract_configs
        .contains_key("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"));
    assert!(chain_config
        .deposit_contract_configs
        .contains_key("0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"));
    let provider = chain_config.provider_configs.first().unwrap();
    assert_eq!(
        &provider.url,
        "wss://goerli.infura.io/ws/v3/9aa3d95b3bc440fa88ea12eaa4456161"
    );
}

#[tokio::test]
async fn test_mystiko_config_to_proto() {
    let mystiko_config = mystiko_config::MystikoConfig::from_json_file("tests/files/config/mystiko.json")
        .await
        .unwrap();
    let mystiko_config: MystikoConfig = (&mystiko_config).try_into().unwrap();
    assert_eq!(&mystiko_config.version, "0.1.0");
    assert_eq!(mystiko_config.git_revision(), "b6b5b5b");
    assert_eq!(
        mystiko_config.country_blacklist,
        vec![String::from("US"), String::from("CN")]
    );
    let mut chain_ids: Vec<u64> = mystiko_config.chain_configs.into_keys().collect();
    chain_ids.sort();
    assert_eq!(chain_ids, vec![5, 97]);
    let mut bridge_names: Vec<String> = mystiko_config
        .bridge_configs
        .into_values()
        .map(|v| v.name.clone())
        .collect();
    bridge_names.sort();
    assert_eq!(
        bridge_names,
        vec![
            "Axelar Bridge",
            "Celer Bridge",
            "LayerZero Bridge",
            "Mystiko Testnet Bridge",
            "Poly Bridge",
            "Wormhole Bridge"
        ]
    );
    let mut circuit_names: Vec<String> = mystiko_config.circuit_configs.into_iter().map(|c| c.name).collect();
    circuit_names.sort();
    assert_eq!(
        circuit_names,
        vec![
            "zokrates-1.0-rollup1",
            "zokrates-1.0-rollup1024",
            "zokrates-1.0-rollup128",
            "zokrates-1.0-rollup16",
            "zokrates-1.0-rollup2",
            "zokrates-1.0-rollup256",
            "zokrates-1.0-rollup32",
            "zokrates-1.0-rollup4",
            "zokrates-1.0-rollup512",
            "zokrates-1.0-rollup64",
            "zokrates-1.0-rollup8",
            "zokrates-1.0-transaction1x0",
            "zokrates-1.0-transaction1x1",
            "zokrates-1.0-transaction1x2",
            "zokrates-1.0-transaction2x0",
            "zokrates-1.0-transaction2x1",
            "zokrates-1.0-transaction2x2",
            "zokrates-2.0-rollup1"
        ]
    );
    let sequencer_config = mystiko_config.sequencer_config.clone().unwrap();
    assert_eq!(sequencer_config.host(), "example.com");
    assert_eq!(sequencer_config.port(), 23433);
    assert!(sequencer_config.is_ssl());
    assert_eq!(
        &mystiko_config.packer_config.unwrap().url,
        "https://static.mystiko.network/packer/v1"
    );
}
