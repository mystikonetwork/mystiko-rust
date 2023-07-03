#![forbid(unsafe_code)]

use mystiko_storage::document::DocumentData;
use mystiko_storage_macros::CollectionBuilder;
use serde::{Deserialize, Serialize};

#[derive(CollectionBuilder, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub encrypted_entropy: String,
    pub hashed_password: String,
    pub account_nonce: u32,
}
