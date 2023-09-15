use crate::core::v1::{AccountStatus, PackerChecksum, PackerCompression};

impl From<&mystiko_types::PackerChecksum> for PackerChecksum {
    fn from(value: &mystiko_types::PackerChecksum) -> Self {
        match value {
            mystiko_types::PackerChecksum::Sha512 => PackerChecksum::Sha512,
        }
    }
}

impl From<&mystiko_types::PackerCompression> for PackerCompression {
    fn from(value: &mystiko_types::PackerCompression) -> Self {
        match value {
            mystiko_types::PackerCompression::Zstd => PackerCompression::Zstd,
        }
    }
}

impl From<&mystiko_types::AccountStatus> for AccountStatus {
    fn from(value: &mystiko_types::AccountStatus) -> Self {
        match value {
            mystiko_types::AccountStatus::Created => AccountStatus::Created,
            mystiko_types::AccountStatus::Scanning => AccountStatus::Scanning,
            mystiko_types::AccountStatus::Scanned => AccountStatus::Scanned,
        }
    }
}

impl From<AccountStatus> for mystiko_types::AccountStatus {
    fn from(value: AccountStatus) -> Self {
        match value {
            AccountStatus::Unspecified => mystiko_types::AccountStatus::Created,
            AccountStatus::Created => mystiko_types::AccountStatus::Created,
            AccountStatus::Scanning => mystiko_types::AccountStatus::Scanning,
            AccountStatus::Scanned => mystiko_types::AccountStatus::Scanned,
        }
    }
}
