use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LoaderEvent {
    StartEvent,
    StopEvent,
    LoadEvent,
    LoadSuccessEvent,
    LoadFailureEvent,
}

#[async_trait]
pub trait Loaderlistener: Send + Sync {
    async fn callback(&self, event: &LoaderEvent) -> Result<()>;
}
