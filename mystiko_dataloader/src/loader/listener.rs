use crate::error::DataloaderError;
use anyhow::Result;
use async_trait::async_trait;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct StartEvent {
    pub start_block: u64,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct StopEvent {
    pub end_block: u64,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct LoadEvent {
    pub start_block: u64,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct LoadSuccessEvent {
    pub end_block: u64,
}

#[derive(Debug, TypedBuilder)]
pub struct LoadFailureEvent {
    pub end_block: u64,
    pub load_error: DataloaderError,
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
