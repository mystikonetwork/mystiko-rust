#![forbid(unsafe_code)]

use mystiko_protos::core::document::v1::Provider as ProtoProvider;
use mystiko_storage::column::{IndexColumns, UniqueColumns};
use mystiko_storage::document::DocumentData;
use mystiko_storage_macros::CollectionBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Provider {
    pub url: String,
    pub timeout_ms: u32,
    pub max_try_count: u32,
    pub quorum_weight: u32,
}

#[derive(CollectionBuilder, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[collection(uniques = uniques(), indexes = indexes())]
pub struct Chain {
    pub chain_id: u64,
    #[column(length_limit = 64)]
    pub name: String,
    pub name_override: bool,
    pub providers: Vec<Provider>,
    pub provider_override: bool,
    pub synced_block_number: u64,
}

fn uniques() -> Vec<UniqueColumns> {
    vec![vec![ChainColumn::ChainId].into()]
}

fn indexes() -> Vec<IndexColumns> {
    vec![vec![ChainColumn::ChainId].into()]
}

impl From<Provider> for ProtoProvider {
    fn from(value: Provider) -> Self {
        ProtoProvider::builder()
            .url(value.url)
            .timeout_ms(value.timeout_ms)
            .max_try_count(value.max_try_count)
            .quorum_weight(value.quorum_weight)
            .build()
    }
}
