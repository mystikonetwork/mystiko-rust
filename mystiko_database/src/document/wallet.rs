#![forbid(unsafe_code)]

use mystiko_storage::document::DocumentData;
use mystiko_storage_macros::CollectionBuilder;

#[derive(CollectionBuilder, Clone, PartialEq, Debug)]
pub struct Wallet {
    pub encrypted_entropy: String,
    pub hashed_password: String,
    pub account_nonce: u32,
}
