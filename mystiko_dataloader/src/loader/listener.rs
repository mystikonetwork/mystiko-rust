use crate::error::DataLoaderError;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct ScheduleEvent {}

#[derive(Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct StopScheduleEvent {}

#[derive(Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct LoadEvent {
    pub start_block: u64,
    pub target_block: u64,
}

#[derive(Debug, Clone, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct LoadSuccessEvent {
    pub start_block: u64,
    pub loaded_block: u64,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct LoadFailureEvent {
    pub start_block: u64,
    pub loaded_block: u64,
    pub load_error: DataLoaderError,
}

#[derive(Debug)]
pub enum LoaderEvent {
    ScheduleEvent(ScheduleEvent),
    StopScheduleEvent(StopScheduleEvent),
    LoadEvent(LoadEvent),
    LoadSuccessEvent(LoadSuccessEvent),
    LoadFailureEvent(LoadFailureEvent),
}

#[async_trait]
pub trait LoaderListener: Send + Sync {
    async fn callback(&self, chain_id: u64, event: &LoaderEvent) -> Result<()>;
}

#[async_trait]
impl LoaderListener for Box<dyn LoaderListener> {
    async fn callback(&self, chain_id: u64, event: &LoaderEvent) -> Result<()> {
        self.as_ref().callback(chain_id, event).await
    }
}
