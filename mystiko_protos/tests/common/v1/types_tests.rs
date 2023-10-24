use mystiko_protos::common::v1::{AssetType, BridgeType, CircuitType, ContractType, ProviderType, TransactionType};

#[test]
fn test_bridge_type_to_proto() {
    let type1: BridgeType = Into::into(&mystiko_types::BridgeType::Loop);
    let type2: BridgeType = Into::into(&mystiko_types::BridgeType::Poly);
    let type3: BridgeType = Into::into(&mystiko_types::BridgeType::Tbridge);
    let type4: BridgeType = Into::into(&mystiko_types::BridgeType::Celer);
    let type5: BridgeType = Into::into(&mystiko_types::BridgeType::LayerZero);
    let type6: BridgeType = Into::into(&mystiko_types::BridgeType::Axelar);
    assert_eq!(type1, BridgeType::Loop);
    assert_eq!(type2, BridgeType::Poly);
    assert_eq!(type3, BridgeType::Tbridge);
    assert_eq!(type4, BridgeType::Celer);
    assert_eq!(type5, BridgeType::LayerZero);
    assert_eq!(type6, BridgeType::Axelar);
}

#[test]
fn test_asset_type_to_proto() {
    let type1: AssetType = Into::into(&mystiko_types::AssetType::Main);
    let type2: AssetType = Into::into(&mystiko_types::AssetType::Erc20);
    assert_eq!(type1, AssetType::Main);
    assert_eq!(type2, AssetType::Erc20);
}

#[test]
fn test_circuit_type_to_proto() {
    let type1: CircuitType = Into::into(&mystiko_types::CircuitType::Rollup1);
    let type2: CircuitType = Into::into(&mystiko_types::CircuitType::Rollup2);
    let type3: CircuitType = Into::into(&mystiko_types::CircuitType::Rollup4);
    let type4: CircuitType = Into::into(&mystiko_types::CircuitType::Rollup8);
    let type5: CircuitType = Into::into(&mystiko_types::CircuitType::Rollup16);
    let type6: CircuitType = Into::into(&mystiko_types::CircuitType::Transaction1x0);
    let type7: CircuitType = Into::into(&mystiko_types::CircuitType::Transaction1x1);
    let type8: CircuitType = Into::into(&mystiko_types::CircuitType::Transaction1x2);
    let type9: CircuitType = Into::into(&mystiko_types::CircuitType::Transaction2x0);
    let type10: CircuitType = Into::into(&mystiko_types::CircuitType::Transaction2x1);
    let type11: CircuitType = Into::into(&mystiko_types::CircuitType::Transaction2x2);
    assert_eq!(type1, CircuitType::Rollup1);
    assert_eq!(type2, CircuitType::Rollup2);
    assert_eq!(type3, CircuitType::Rollup4);
    assert_eq!(type4, CircuitType::Rollup8);
    assert_eq!(type5, CircuitType::Rollup16);
    assert_eq!(type6, CircuitType::Transaction1x0);
    assert_eq!(type7, CircuitType::Transaction1x1);
    assert_eq!(type8, CircuitType::Transaction1x2);
    assert_eq!(type9, CircuitType::Transaction2x0);
    assert_eq!(type10, CircuitType::Transaction2x1);
    assert_eq!(type11, CircuitType::Transaction2x2);
}

#[test]
fn test_contract_type_to_proto() {
    let type1: ContractType = Into::into(&mystiko_types::ContractType::Deposit);
    let type2: ContractType = Into::into(&mystiko_types::ContractType::Pool);
    assert_eq!(type1, ContractType::Deposit);
    assert_eq!(type2, ContractType::Pool);
}

#[test]
fn test_provider_type_to_proto() {
    let type1: ProviderType = Into::into(&mystiko_types::ProviderType::Failover);
    let type2: ProviderType = Into::into(&mystiko_types::ProviderType::Quorum);
    assert_eq!(type1, ProviderType::Failover);
    assert_eq!(type2, ProviderType::Quorum);
}

#[test]
fn test_proto_to_contract_type() {
    let type1: mystiko_types::ContractType = Into::into(ContractType::Deposit);
    let type2: mystiko_types::ContractType = Into::into(ContractType::Pool);
    assert_eq!(type1, mystiko_types::ContractType::Deposit);
    assert_eq!(type2, mystiko_types::ContractType::Pool);
}

#[test]
fn test_proto_to_bridge_type() {
    let type1: mystiko_types::BridgeType = Into::into(BridgeType::Loop);
    let type2: mystiko_types::BridgeType = Into::into(BridgeType::Poly);
    let type3: mystiko_types::BridgeType = Into::into(BridgeType::Tbridge);
    let type4: mystiko_types::BridgeType = Into::into(BridgeType::Celer);
    let type5: mystiko_types::BridgeType = Into::into(BridgeType::LayerZero);
    let type6: mystiko_types::BridgeType = Into::into(BridgeType::Axelar);
    assert_eq!(type1, mystiko_types::BridgeType::Loop);
    assert_eq!(type2, mystiko_types::BridgeType::Poly);
    assert_eq!(type3, mystiko_types::BridgeType::Tbridge);
    assert_eq!(type4, mystiko_types::BridgeType::Celer);
    assert_eq!(type5, mystiko_types::BridgeType::LayerZero);
    assert_eq!(type6, mystiko_types::BridgeType::Axelar);
}

#[test]
fn test_proto_to_circuit_type() {
    let type1: mystiko_types::CircuitType = Into::into(CircuitType::Rollup1);
    let type2: mystiko_types::CircuitType = Into::into(CircuitType::Rollup2);
    let type3: mystiko_types::CircuitType = Into::into(CircuitType::Rollup4);
    let type4: mystiko_types::CircuitType = Into::into(CircuitType::Rollup8);
    let type5: mystiko_types::CircuitType = Into::into(CircuitType::Rollup16);
    let type6: mystiko_types::CircuitType = Into::into(CircuitType::Transaction1x0);
    let type7: mystiko_types::CircuitType = Into::into(CircuitType::Transaction1x1);
    let type8: mystiko_types::CircuitType = Into::into(CircuitType::Transaction1x2);
    let type9: mystiko_types::CircuitType = Into::into(CircuitType::Transaction2x0);
    let type10: mystiko_types::CircuitType = Into::into(CircuitType::Transaction2x1);
    let type11: mystiko_types::CircuitType = Into::into(CircuitType::Transaction2x2);
    assert_eq!(type1, mystiko_types::CircuitType::Rollup1);
    assert_eq!(type2, mystiko_types::CircuitType::Rollup2);
    assert_eq!(type3, mystiko_types::CircuitType::Rollup4);
    assert_eq!(type4, mystiko_types::CircuitType::Rollup8);
    assert_eq!(type5, mystiko_types::CircuitType::Rollup16);
    assert_eq!(type6, mystiko_types::CircuitType::Transaction1x0);
    assert_eq!(type7, mystiko_types::CircuitType::Transaction1x1);
    assert_eq!(type8, mystiko_types::CircuitType::Transaction1x2);
    assert_eq!(type9, mystiko_types::CircuitType::Transaction2x0);
    assert_eq!(type10, mystiko_types::CircuitType::Transaction2x1);
    assert_eq!(type11, mystiko_types::CircuitType::Transaction2x2);
}

#[test]
fn test_transaction_type_to_proto() {
    let type1: TransactionType = Into::into(&mystiko_types::TransactionType::Legacy);
    let type2: TransactionType = Into::into(&mystiko_types::TransactionType::Eip1559);
    let type3: TransactionType = Into::into(&mystiko_types::TransactionType::Eip2930);
    assert_eq!(type1, TransactionType::Legacy);
    assert_eq!(type2, TransactionType::Eip1559);
    assert_eq!(type3, TransactionType::Eip2930);
}
