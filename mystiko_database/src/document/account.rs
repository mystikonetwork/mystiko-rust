#![forbid(unsafe_code)]

use mystiko_storage::column::{IndexColumns, UniqueColumns};
use mystiko_storage::document::DocumentData;
use mystiko_storage_macros::CollectionBuilder;
use mystiko_types::AccountStatus;
use serde::{Deserialize, Serialize};

#[derive(CollectionBuilder, Clone, PartialEq, Debug, Deserialize, Serialize)]
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
        vec![AccountColumn::WalletId, AccountColumn::ShieldedAddress].into(),
        vec![AccountColumn::WalletId, AccountColumn::PublicKey].into(),
    ]
}

fn indexes() -> Vec<IndexColumns> {
    vec![
        vec![AccountColumn::WalletId].into(),
        vec![AccountColumn::ShieldedAddress].into(),
        vec![AccountColumn::PublicKey].into(),
    ]
}
