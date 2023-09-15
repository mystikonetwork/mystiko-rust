#![forbid(unsafe_code)]

use mystiko_protos::core::document::v1::Account as ProtoAccount;
use mystiko_storage::{Document, DocumentData, IndexColumns, UniqueColumns};
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

impl Account {
    pub fn from_proto(proto: ProtoAccount) -> Document<Account> {
        let status = proto.status().into();
        Document::new(
            proto.id,
            proto.created_at,
            proto.updated_at,
            Account {
                name: proto.name,
                shielded_address: proto.shielded_address,
                public_key: proto.public_key,
                encrypted_secret_key: proto.encrypted_secret_key,
                status,
                scan_size: proto.scan_size,
                wallet_id: proto.wallet_id,
            },
        )
    }

    pub fn into_proto(account: Document<Self>) -> ProtoAccount {
        ProtoAccount::builder()
            .id(account.id)
            .created_at(account.created_at)
            .updated_at(account.updated_at)
            .name(account.data.name)
            .shielded_address(account.data.shielded_address)
            .public_key(account.data.public_key)
            .encrypted_secret_key(account.data.encrypted_secret_key)
            .scan_size(account.data.scan_size)
            .wallet_id(account.data.wallet_id)
            .status(Into::<mystiko_protos::core::v1::AccountStatus>::into(
                &account.data.status,
            ))
            .build()
    }
}
