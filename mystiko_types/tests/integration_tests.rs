use mystiko_types::{
    AccountStatus, AssetType, BridgeType, CircuitType, ContractType, PackerChecksum, PackerCompression, ProviderType,
};

#[test]
fn test_i32_to_account_status() {
    let i1 = 1;
    let i2 = 2;
    let i3 = 3;
    let status1: AccountStatus = i1.into();
    let status2: AccountStatus = i2.into();
    let status3: AccountStatus = i3.into();
    assert_eq!(status1, AccountStatus::Created);
    assert_eq!(status2, AccountStatus::Scanning);
    assert_eq!(status3, AccountStatus::Scanned);
}

#[test]
fn test_account_status_to_i32() {
    let status1 = AccountStatus::Created;
    let status2 = AccountStatus::Scanning;
    let status3 = AccountStatus::Scanned;
    let i1: i32 = status1.into();
    let i2: i32 = status2.into();
    let i3: i32 = status3.into();
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
    assert_eq!(i3, 3);
}

#[test]
fn test_i32_to_contract_type() {
    let i1 = 1;
    let i2 = 2;
    let type1: ContractType = i1.into();
    let type2: ContractType = i2.into();
    assert_eq!(type1, ContractType::Deposit);
    assert_eq!(type2, ContractType::Pool);
}

#[test]
fn test_contract_type_to_i32() {
    let type1 = &ContractType::Deposit;
    let type2 = &ContractType::Pool;
    let i1: i32 = type1.into();
    let i2: i32 = type2.into();
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
}

#[test]
fn test_bridge_type_to_i32() {
    let type1 = &BridgeType::Loop;
    let type2 = &BridgeType::Poly;
    let type3 = &BridgeType::Tbridge;
    let type4 = &BridgeType::Celer;
    let type5 = &BridgeType::LayerZero;
    let type6 = &BridgeType::Axelar;
    let i1: i32 = type1.into();
    let i2: i32 = type2.into();
    let i3: i32 = type3.into();
    let i4: i32 = type4.into();
    let i5: i32 = type5.into();
    let i6: i32 = type6.into();
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
    assert_eq!(i3, 3);
    assert_eq!(i4, 4);
    assert_eq!(i5, 5);
    assert_eq!(i6, 6);
}

#[test]
fn test_asset_type_to_i32() {
    let type1 = &AssetType::Erc20;
    let type2 = &AssetType::Main;
    let i1: i32 = type1.into();
    let i2: i32 = type2.into();
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
}

#[test]
fn test_circuit_type_to_i32() {
    let type1 = &CircuitType::Rollup1;
    let type2 = &CircuitType::Rollup2;
    let type3 = &CircuitType::Rollup4;
    let type4 = &CircuitType::Rollup8;
    let type5 = &CircuitType::Rollup16;
    let type6 = &CircuitType::Transaction1x0;
    let type7 = &CircuitType::Transaction1x1;
    let type8 = &CircuitType::Transaction1x2;
    let type9 = &CircuitType::Transaction2x0;
    let type10 = &CircuitType::Transaction2x1;
    let type11 = &CircuitType::Transaction2x2;
    let i1: i32 = type1.into();
    let i2: i32 = type2.into();
    let i3: i32 = type3.into();
    let i4: i32 = type4.into();
    let i5: i32 = type5.into();
    let i6: i32 = type6.into();
    let i7: i32 = type7.into();
    let i8: i32 = type8.into();
    let i9: i32 = type9.into();
    let i10: i32 = type10.into();
    let i11: i32 = type11.into();
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
    assert_eq!(i3, 3);
    assert_eq!(i4, 4);
    assert_eq!(i5, 5);
    assert_eq!(i6, 6);
    assert_eq!(i7, 7);
    assert_eq!(i8, 8);
    assert_eq!(i9, 9);
    assert_eq!(i10, 10);
    assert_eq!(i11, 11);
}

#[test]
fn test_provider_type_to_i32() {
    let type1 = &ProviderType::Failover;
    let type2 = &ProviderType::Quorum;
    let i1: i32 = type1.into();
    let i2: i32 = type2.into();
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
}

#[test]
fn test_packer_compression_to_i32() {
    let type1 = &PackerCompression::Zstd;
    let i1: i32 = type1.into();
    assert_eq!(i1, 1);
}

#[test]
fn test_packer_sum_to_i32() {
    let type1 = &PackerChecksum::Sha512;
    let i1: i32 = type1.into();
    assert_eq!(i1, 1);
}
