#![forbid(unsafe_code)]

use mystiko_storage::column::{IndexColumns, UniqueColumns};
use mystiko_storage_macros::CollectionBuilder;
use mystiko_types::AccountStatus;

#[derive(CollectionBuilder, Clone, PartialEq, Debug)]
#[collection(uniques = uniques(), indexes = indexes())]
pub struct Account {
    pub name: String,
    #[column(length_limit = 255)]
    pub shielded_address: String,
    #[column(length_limit = 255)]
    pub public_key: String,
    pub encrypted_secret_key: String,
    #[column(length_limit = 32)]
    pub status: AccountStatus,
    pub scan_size: u32,
    #[column(length_limit = 64)]
    pub wallet_id: String,
}

fn uniques() -> Vec<UniqueColumns> {
    vec![
        UniqueColumns::builder()
            .column_names(vec![
                AccountColumn::WalletId.to_string(),
                AccountColumn::ShieldedAddress.to_string(),
            ])
            .build(),
        UniqueColumns::builder()
            .column_names(vec![
                AccountColumn::WalletId.to_string(),
                AccountColumn::PublicKey.to_string(),
            ])
            .build(),
    ]
}

fn indexes() -> Vec<IndexColumns> {
    vec![
        IndexColumns::builder()
            .column_names(vec![AccountColumn::WalletId.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![AccountColumn::ShieldedAddress.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![AccountColumn::PublicKey.to_string()])
            .build(),
    ]
}
