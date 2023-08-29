#![forbid(unsafe_code)]

use mystiko_protos::core::document::v1::Chain as ProtoChain;
use mystiko_protos::core::document::v1::Provider as ProtoProvider;
use mystiko_storage::column::{IndexColumns, UniqueColumns};
use mystiko_storage::document::{Document, DocumentData};
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

impl From<ProtoProvider> for Provider {
    fn from(value: ProtoProvider) -> Self {
        Provider {
            url: value.url,
            timeout_ms: value.timeout_ms,
            max_try_count: value.max_try_count,
            quorum_weight: value.quorum_weight,
        }
    }
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

impl Chain {
    pub fn from_proto(proto: ProtoChain) -> Document<Self> {
        Document::new(
            proto.id,
            proto.created_at,
            proto.updated_at,
            Chain {
                chain_id: proto.chain_id,
                name: proto.name,
                name_override: proto.name_override,
                providers: proto
                    .providers
                    .into_iter()
                    .map(|provider| provider.into())
                    .collect::<Vec<Provider>>(),
                provider_override: proto.provider_override,
                synced_block_number: proto.synced_block_number,
            },
        )
    }

    pub fn into_proto(chain: Document<Self>) -> ProtoChain {
        ProtoChain::builder()
            .id(chain.id)
            .created_at(chain.created_at)
            .updated_at(chain.updated_at)
            .chain_id(chain.data.chain_id)
            .name(chain.data.name)
            .name_override(chain.data.name_override)
            .providers(
                chain
                    .data
                    .providers
                    .into_iter()
                    .map(|provider| provider.into())
                    .collect::<Vec<ProtoProvider>>(),
            )
            .provider_override(chain.data.provider_override)
            .synced_block_number(chain.data.synced_block_number)
            .build()
    }
}
