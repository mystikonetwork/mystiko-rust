use mystiko_config::MystikoConfig;
use mystiko_dataloader::handler::document::{Contract, DatabaseContract};
use std::sync::Arc;

#[test]
fn test_contract() {
    assert_eq!(Contract::column_chain_id(), "chain_id");
    assert_eq!(Contract::column_contract_address(), "contract_address");
    assert_eq!(Contract::column_loaded_block(), "loaded_block");
    let mut contract = Contract::builder()
        .chain_id(1u64)
        .contract_address("address1".to_string())
        .loaded_block(1000u64)
        .build();
    assert_eq!(contract.get_chain_id(), 1_u64);
    assert_eq!(contract.get_contract_address(), "address1");
    assert_eq!(contract.get_loaded_block(), 1000u64);
    contract.update_loaded_block(2000u64);
    assert_eq!(contract.get_chain_id(), 1u64);
    assert_eq!(contract.get_contract_address(), "address1");
    assert_eq!(contract.get_loaded_block(), 2000u64);
}

#[tokio::test]
async fn test_from_config() {
    let config = Arc::new(
        MystikoConfig::from_json_file("./tests/files/handler/config.json")
            .await
            .unwrap(),
    );
    let contract_result =
        Contract::from_config(Arc::clone(&config), 1u64, "0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01");
    assert!(contract_result.is_ok());
    let contract = contract_result.unwrap();
    assert_eq!(
        contract.contract_address,
        "0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01".to_string()
    );
    assert_eq!(contract.chain_id, 1u64);
    assert_eq!(contract.loaded_block, 16690440);
    let contract_result = Contract::from_config(
        Arc::clone(&config),
        123456u64,
        "0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01",
    );
    assert!(contract_result.is_err());
}
