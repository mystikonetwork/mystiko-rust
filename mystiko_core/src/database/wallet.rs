use mystiko_protos::core::document::v1::Wallet as ProtoWallet;
use mystiko_storage::{Document, DocumentData};
use mystiko_storage_macros::CollectionBuilder;
use serde::{Deserialize, Serialize};

#[derive(CollectionBuilder, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub mnemonic_type: i32,
    pub encrypted_entropy: String,
    pub hashed_password: String,
    pub account_nonce: u32,
}

impl Wallet {
    pub fn document_from_proto(proto: ProtoWallet) -> Document<Self> {
        Document::new(
            proto.id,
            proto.created_at,
            proto.updated_at,
            Wallet {
                mnemonic_type: proto.mnemonic_type,
                encrypted_entropy: proto.encrypted_entropy,
                hashed_password: proto.hashed_password,
                account_nonce: proto.account_nonce,
            },
        )
    }

    pub fn document_into_proto(wallet: Document<Self>) -> ProtoWallet {
        ProtoWallet::builder()
            .id(wallet.id)
            .created_at(wallet.created_at)
            .updated_at(wallet.updated_at)
            .mnemonic_type(wallet.data.mnemonic_type)
            .encrypted_entropy(wallet.data.encrypted_entropy)
            .hashed_password(wallet.data.hashed_password)
            .account_nonce(wallet.data.account_nonce)
            .build()
    }
}
