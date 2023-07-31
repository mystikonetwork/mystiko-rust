use crate::error::DataLoaderError;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct StartEvent {
    pub start_block: u64,
}

#[derive(Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct StopEvent {
    pub end_block: u64,
}

#[derive(Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct LoadEvent {
    pub start_block: u64,
}

#[derive(Debug, Clone, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct LoadSuccessEvent {
    pub end_block: u64,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct LoadFailureEvent {
    pub end_block: u64,
    pub load_error: DataLoaderError,
}

#[derive(Debug)]
pub enum LoaderEvent {
    StartEvent(StartEvent),
    StopEvent(StopEvent),
    LoadEvent(LoadEvent),
    LoadSuccessEvent(LoadSuccessEvent),
    LoadFailureEvent(LoadFailureEvent),
}

#[async_trait]
pub trait LoaderListener: Send + Sync {
    async fn callback(&self, event: &LoaderEvent) -> Result<()>;
}

#[async_trait]
impl LoaderListener for Box<dyn LoaderListener> {
    async fn callback(&self, event: &LoaderEvent) -> Result<()> {
        self.as_ref().callback(event).await
    }
}
