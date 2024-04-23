use crate::{setup, FULL_CONFIG_FILE};
use mystiko_lib::config::{
    find_asset_symbols, find_bridge, find_bridges, find_chain, find_circuit, find_contract_by_address,
    find_default_circuit, find_deposit_contract, find_deposit_contract_by_address, find_peer_chains,
    find_pool_contract, find_pool_contract_by_address, find_pool_contracts, get, get_transaction_url,
    supported_asset_symbols,
};
use mystiko_protos::api::config::v1::{
    FindAssetSymbolsRequest, FindAssetSymbolsResponse, FindBridgeRequest, FindBridgeResponse, FindBridgesRequest,
    FindBridgesResponse, FindChainRequest, FindChainResponse, FindCircuitRequest, FindCircuitResponse,
    FindContractByAddressRequest, FindContractByAddressResponse, FindDefaultCircuitRequest, FindDefaultCircuitResponse,
    FindDepositContractByAddressRequest, FindDepositContractByAddressResponse, FindDepositContractRequest,
    FindDepositContractResponse, FindPeerChainsRequest, FindPeerChainsResponse, FindPoolContractByAddressRequest,
    FindPoolContractByAddressResponse, FindPoolContractRequest, FindPoolContractResponse, FindPoolContractsRequest,
    FindPoolContractsResponse, GetConfigResponse, GetTransactionUrlRequest, GetTransactionUrlResponse,
    SupportedAssetSymbolsRequest, SupportedAssetSymbolsResponse,
};
use mystiko_protos::api::v1::api_response;
use mystiko_protos::common::v1::{BridgeType, CircuitType};
use serial_test::serial;

#[test]
#[serial]
fn test_with_valid_config() {
    setup(None);
    let response = get();
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = GetConfigResponse::try_from(data).unwrap();
            assert!(response.config.is_some());
            let config = response.config.unwrap();
            assert_eq!(config.version, "0.1.0");
            assert_eq!(config.git_revision(), "b6b5b5b");
            assert_eq!(config.country_blacklist, vec![String::from("US"), String::from("CN")]);
            let mut chain_ids: Vec<u64> = config.chain_configs.into_keys().collect();
            chain_ids.sort();
            assert_eq!(chain_ids, vec![5, 97]);
            let mut bridge_names: Vec<String> = config.bridge_configs.into_values().map(|v| v.name.clone()).collect();
            bridge_names.sort();
            assert_eq!(
                bridge_names,
                vec![
                    "Axelar Network",
                    "Celer Network",
                    "LayerZero Bridge",
                    "Mystiko Testnet Bridge",
                    "Poly Bridge"
                ]
            );
            let mut circuit_names: Vec<String> = config.circuit_configs.into_iter().map(|c| c.name).collect();
            circuit_names.sort();
            assert_eq!(
                circuit_names,
                vec![
                    "zokrates-1.0-rollup1",
                    "zokrates-1.0-rollup16",
                    "zokrates-1.0-rollup2",
                    "zokrates-1.0-rollup32",
                    "zokrates-1.0-rollup4",
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
            assert_eq!(
                &config.packer_config.unwrap().url,
                "https://static.mystiko.network/packer/v1"
            );
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_default_circuit(
        FindDefaultCircuitRequest::builder()
            .circuit_type(CircuitType::Rollup1)
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindDefaultCircuitResponse::try_from(data).unwrap();
            assert!(response.config.is_some());
            let config = response.config.unwrap();
            assert_eq!(config.name, "zokrates-1.0-rollup1");
            assert_eq!(config.circuit_type(), CircuitType::Rollup1);
            assert!(config.is_default);
            assert_eq!(config.program_file, vec![String::from("./Rollup1.program.gz")]);
            assert_eq!(config.abi_file, vec![String::from("./Rollup1.abi.json")]);
            assert_eq!(config.proving_key_file, vec![String::from("./Rollup1.pkey.gz")]);
            assert_eq!(config.verifying_key_file, vec![String::from("./Rollup1.vkey.gz")]);
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_circuit(
        FindCircuitRequest::builder()
            .circuit_name("zokrates-1.0-transaction1x0")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindCircuitResponse::try_from(data).unwrap();
            assert!(response.config.is_some());
            let config = response.config.unwrap();
            assert_eq!(config.name, "zokrates-1.0-transaction1x0");
            assert_eq!(config.circuit_type(), CircuitType::Transaction1x0);
            assert!(config.is_default);
            assert_eq!(config.program_file, vec![String::from("./Transaction1x0.program.gz")]);
            assert_eq!(config.abi_file, vec![String::from("./Transaction1x0.abi.json")]);
            assert_eq!(config.proving_key_file, vec![String::from("./Transaction1x0.pkey.gz")]);
            assert_eq!(
                config.verifying_key_file,
                vec![String::from("./Transaction1x0.vkey.gz")]
            );
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_chain(FindChainRequest::builder().chain_id(5u64).build());
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindChainResponse::try_from(data).unwrap();
            assert!(response.config.is_some());
            let config = response.config.unwrap();
            assert_eq!(config.chain_id, 5);
            assert_eq!(config.name, "Ethereum Goerli");
            assert_eq!(config.asset_symbol, "ETH");
            assert_eq!(config.asset_decimals, 18);
            assert_eq!(config.explorer_url, "https://goerli.etherscan.io");
            assert_eq!(config.explorer_api_url, "https://api-goerli.etherscan.io");
            assert_eq!(config.explorer_prefix, "/tx/%tx%");
            assert_eq!(config.provider_configs.len(), 2);
            assert_eq!(
                config.signer_endpoint,
                "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161"
            );
            assert_eq!(config.deposit_contract_configs.len(), 1);
            assert_eq!(config.pool_contract_configs.len(), 1);
            assert_eq!(config.asset_configs.len(), 1);
            assert_eq!(config.granularities.len(), 4);
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_bridge(FindBridgeRequest::builder().bridge_type(BridgeType::Axelar).build());
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindBridgeResponse::try_from(data).unwrap();
            assert!(response.config.is_some());
            let config = response.config.unwrap();
            assert_eq!(config.name, "Axelar Network");
            assert_eq!(config.bridge_type(), BridgeType::Axelar);
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = get_transaction_url(
        GetTransactionUrlRequest::builder()
            .chain_id(5u64)
            .tx_hash("0xbce8d733536ee3b769456cf91bebae1e9e5be6cb89bb7490c6225384e1bc5e3e")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = GetTransactionUrlResponse::try_from(data).unwrap();
            assert_eq!(
                response.url(),
                "https://goerli.etherscan.io/tx/\
        0xbce8d733536ee3b769456cf91bebae1e9e5be6cb89bb7490c6225384e1bc5e3e"
            );
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = get_transaction_url(
        GetTransactionUrlRequest::builder()
            .chain_id(234243u64)
            .tx_hash("xxx")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = GetTransactionUrlResponse::try_from(data).unwrap();
            assert!(response.url.is_none());
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }
}

#[test]
#[serial]
fn test_with_full_config() {
    setup(Some(FULL_CONFIG_FILE.to_string()));

    let response = find_peer_chains(FindPeerChainsRequest::builder().chain_id(5u64).build());
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindPeerChainsResponse::try_from(data).unwrap();
            let configs = response.configs;
            let mut peer_chain_ids = configs.into_iter().map(|c| c.chain_id).collect::<Vec<u64>>();
            peer_chain_ids.sort();
            assert_eq!(peer_chain_ids, vec![5, 97]);
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_asset_symbols(
        FindAssetSymbolsRequest::builder()
            .chain_id(5u64)
            .peer_chain_id(5u64)
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindAssetSymbolsResponse::try_from(data).unwrap();
            assert_eq!(response.asset_symbol, vec!["ETH"]);
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_asset_symbols(
        FindAssetSymbolsRequest::builder()
            .chain_id(5u64)
            .peer_chain_id(97u64)
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindAssetSymbolsResponse::try_from(data).unwrap();
            assert_eq!(response.asset_symbol, vec!["MTT"]);
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_asset_symbols(
        FindAssetSymbolsRequest::builder()
            .chain_id(97u64)
            .peer_chain_id(97u64)
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindAssetSymbolsResponse::try_from(data).unwrap();
            assert!(response.asset_symbol.is_empty());
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_bridges(
        FindBridgesRequest::builder()
            .chain_id(5u64)
            .peer_chain_id(97u64)
            .asset_symbol("MTT")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindBridgesResponse::try_from(data).unwrap();
            let mut bridges: Vec<BridgeType> = response.bridge_type().collect();
            bridges.sort_by_key(|b| format!("{:?}", b));
            assert_eq!(
                bridges,
                [
                    BridgeType::Axelar,
                    BridgeType::Celer,
                    BridgeType::LayerZero,
                    BridgeType::Tbridge
                ]
            );
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_deposit_contract(
        FindDepositContractRequest::builder()
            .chain_id(5u64)
            .peer_chain_id(97u64)
            .bridge_type(BridgeType::Celer)
            .asset_symbol("MTT")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindDepositContractResponse::try_from(data).unwrap();
            assert!(response.config.is_some());
            let config = response.config.unwrap();
            assert_eq!(config.address, "0xe6394a06905d83B19Dbd51804Ca84677a2054FA6");
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error)
        }
    }

    let response = find_deposit_contract(
        FindDepositContractRequest::builder()
            .chain_id(5u64)
            .peer_chain_id(97u64)
            .bridge_type(BridgeType::Tbridge)
            .asset_symbol("MTT")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindDepositContractResponse::try_from(data).unwrap();
            assert!(response.config.is_some());
            let config = response.config.unwrap();
            assert_eq!(config.address, "0xbF5605f5Ed6d18ed957cBA80dbA8838dFcb9A69f");
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_deposit_contract(
        FindDepositContractRequest::builder()
            .chain_id(97u64)
            .peer_chain_id(5u64)
            .bridge_type(BridgeType::Celer)
            .asset_symbol("MTT")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindDepositContractResponse::try_from(data).unwrap();
            let config = response.config.unwrap();
            assert_eq!(config.address, "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D");
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_deposit_contract(
        FindDepositContractRequest::builder()
            .chain_id(97u64)
            .peer_chain_id(5u64)
            .bridge_type(BridgeType::Tbridge)
            .asset_symbol("MTT")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindDepositContractResponse::try_from(data).unwrap();
            let config = response.config.unwrap();
            assert_eq!(config.address, "0x9C33eaCc2F50E39940D3AfaF2c7B8246B681A374");
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_deposit_contract(
        FindDepositContractRequest::builder()
            .chain_id(5u64)
            .peer_chain_id(5u64)
            .bridge_type(BridgeType::Loop)
            .asset_symbol("ETH")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindDepositContractResponse::try_from(data).unwrap();
            let config = response.config.unwrap();
            assert_eq!(config.address, "0x390d485f4d43212d4ae8cdd967a711514ed5a54f");
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_deposit_contract(
        FindDepositContractRequest::builder()
            .chain_id(5u64)
            .peer_chain_id(1024234u64)
            .bridge_type(BridgeType::Loop)
            .asset_symbol("MTT")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindDepositContractResponse::try_from(data).unwrap();
            assert!(response.config.is_none());
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_pool_contracts(
        FindPoolContractsRequest::builder()
            .chain_id(5u64)
            .asset_symbol("MTT")
            .bridge_type(BridgeType::Celer)
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindPoolContractsResponse::try_from(data).unwrap();
            let config = response.config;
            let address: Vec<String> = config.into_iter().map(|c| c.address).collect();
            assert_eq!(address, vec!["0x20Eb345870059E688c59e89523442ade33C7c813"]);
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_pool_contract_by_address(
        FindPoolContractByAddressRequest::builder()
            .chain_id(5u64)
            .address("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindPoolContractByAddressResponse::try_from(data).unwrap();
            assert!(response.config.is_some());
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_pool_contract_by_address(
        FindPoolContractByAddressRequest::builder()
            .chain_id(2342342u64)
            .address("0xBe2C9c8a00951662dF3a978b25F448968F0595AE")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindPoolContractByAddressResponse::try_from(data).unwrap();
            assert!(response.config.is_none());
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_deposit_contract_by_address(
        FindDepositContractByAddressRequest::builder()
            .chain_id(5u64)
            .address("0x961f315a836542e603a3df2e0dd9d4ecd06ebc67")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindDepositContractByAddressResponse::try_from(data).unwrap();
            assert!(response.config.is_some());
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_deposit_contract_by_address(
        FindDepositContractByAddressRequest::builder()
            .chain_id(97u64)
            .address("0xd791049D0a154bC7860804e1A18ACD148Eb0afD9")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindDepositContractByAddressResponse::try_from(data).unwrap();
            assert!(response.config.is_some());
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_deposit_contract_by_address(
        FindDepositContractByAddressRequest::builder()
            .chain_id(2342343u64)
            .address("0x9C33eaCc2F50E39940D3AfaF2c7B8246B681A374")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindDepositContractByAddressResponse::try_from(data).unwrap();
            assert!(response.config.is_none());
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_pool_contract(
        FindPoolContractRequest::builder()
            .chain_id(5u64)
            .asset_symbol("MTT")
            .bridge_type(BridgeType::Celer)
            .version(2u32)
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindPoolContractResponse::try_from(data).unwrap();
            assert_eq!(
                response.config.unwrap().address,
                "0x20Eb345870059E688c59e89523442ade33C7c813"
            );
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_pool_contract(
        FindPoolContractRequest::builder()
            .chain_id(5u64)
            .asset_symbol("MTT")
            .bridge_type(BridgeType::Tbridge)
            .version(2u32)
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindPoolContractResponse::try_from(data).unwrap();
            assert_eq!(
                response.config.unwrap().address,
                "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
            );
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_contract_by_address(
        FindContractByAddressRequest::builder()
            .chain_id(2342343u64)
            .address("0x9C33eaCc2F50E39940D3AfaF2c7B8246B681A374")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindPoolContractResponse::try_from(data).unwrap();
            assert!(response.config.is_none());
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_contract_by_address(
        FindContractByAddressRequest::builder()
            .chain_id(5u64)
            .address("0x961f315a836542e603a3df2e0dd9d4ecd06ebc67")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindContractByAddressResponse::try_from(data).unwrap();
            assert!(response.config.is_some());
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = find_contract_by_address(
        FindContractByAddressRequest::builder()
            .chain_id(97u64)
            .address("0xd791049D0a154bC7860804e1A18ACD148Eb0afD9")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = FindContractByAddressResponse::try_from(data).unwrap();
            assert!(response.config.is_some());
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }

    let response = supported_asset_symbols(SupportedAssetSymbolsRequest::builder().chain_id(97u64).build());
    assert!(response.code.unwrap().success);
    let result = response.result.unwrap();
    match result {
        api_response::Result::Data(data) => {
            let response = SupportedAssetSymbolsResponse::try_from(data).unwrap();
            assert_eq!(response.symbols, vec!["MTT"]);
        }
        api_response::Result::ErrorMessage(error) => {
            panic!("{}", error);
        }
    }
}
