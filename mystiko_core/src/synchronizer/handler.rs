use crate::SynchronizerHandler;
use async_trait::async_trait;
use mystiko_dataloader::loader::DataLoader;
use mystiko_protos::core::synchronizer::v1::{SyncOptions, SynchronizerStatus};
use std::collections::HashMap;
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Synchronizer<L: DataLoader> {
    chains: HashMap<u64, L>,
}

#[derive(Debug, Error)]
pub enum SynchronizerError {}

#[async_trait]
impl<L> SynchronizerHandler<SyncOptions, SynchronizerStatus> for Synchronizer<L>
where
    L: DataLoader,
{
    type Error = SynchronizerError;

    async fn chain_synced_block(&self, chain_id: u64) -> Result<Option<u64>, Self::Error> {
        todo!()
    }

    async fn contract_synced_block(&self, chain_id: u64, contract_address: &str) -> Result<Option<u64>, Self::Error> {
        todo!()
    }

    async fn status(&self, with_contracts: bool) -> Result<SynchronizerStatus, Self::Error> {
        todo!()
    }

    async fn sync(&self, sync_option: SyncOptions) -> Result<(), Self::Error> {
        todo!()
    }
}
