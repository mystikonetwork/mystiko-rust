use mystiko_types::{AccountStatus, ContractType};

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
