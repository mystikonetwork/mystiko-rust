use mystiko_protos::core::v1::{AccountStatus, PackerChecksum, PackerCompression};

#[test]
fn test_packer_checksum_to_proto() {
    let type1: PackerChecksum = Into::into(&mystiko_types::PackerChecksum::Sha512);
    assert_eq!(type1, PackerChecksum::Sha512);
}

#[test]
fn test_packer_compression_to_proto() {
    let type1: PackerCompression = Into::into(&mystiko_types::PackerCompression::Zstd);
    assert_eq!(type1, PackerCompression::Zstd);
}

#[test]
fn test_account_status_to_proto() {
    let type1: AccountStatus = Into::into(&mystiko_types::AccountStatus::Created);
    let type2: AccountStatus = Into::into(&mystiko_types::AccountStatus::Scanning);
    let type3: AccountStatus = Into::into(&mystiko_types::AccountStatus::Scanned);
    assert_eq!(type1, AccountStatus::Created);
    assert_eq!(type2, AccountStatus::Scanning);
    assert_eq!(type3, AccountStatus::Scanned);
}

#[test]
fn test_proto_to_account_status() {
    let type1: mystiko_types::AccountStatus = Into::into(AccountStatus::Created);
    let type2: mystiko_types::AccountStatus = Into::into(AccountStatus::Scanning);
    let type3: mystiko_types::AccountStatus = Into::into(AccountStatus::Scanned);
    assert_eq!(type1, mystiko_types::AccountStatus::Created);
    assert_eq!(type2, mystiko_types::AccountStatus::Scanning);
    assert_eq!(type3, mystiko_types::AccountStatus::Scanned);
}
