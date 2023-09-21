use async_trait::async_trait;
use mystiko_scheduler::{Scheduler, SchedulerTask, StartOptions};
use std::sync::Arc;
use tokio::signal::unix::SignalKind;

#[tokio::test]
async fn test_shutdown_signal() {
    let _ = env_logger::builder()
        .filter_module("mystiko_scheduler", log::LevelFilter::Debug)
        .try_init();
    let task = Arc::<SimpleTask>::default();
    let scheduler = Scheduler::new(task);
    let options = StartOptions::<anyhow::Error>::builder().interval_ms(5000u64).build();
    scheduler.start((), options).await.unwrap();
    unsafe {
        libc::raise(SignalKind::interrupt().as_raw_value());
    }
    scheduler.wait_shutdown().await.unwrap();
}

#[derive(Debug, Clone, Default)]
struct SimpleTask;

#[async_trait]
impl SchedulerTask<()> for SimpleTask {
    type Error = anyhow::Error;

    async fn run(&self, _args: &()) -> Result<(), Self::Error> {
        Ok(())
    }
}
