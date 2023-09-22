use async_trait::async_trait;
use mystiko_scheduler::{Scheduler, SchedulerOptions, SchedulerTask, StartOptions};
use std::sync::Arc;
use tokio::signal::unix::SignalKind;

#[tokio::test]
async fn test_shutdown_custom_signal() {
    let _ = env_logger::builder()
        .filter_module("mystiko_scheduler", log::LevelFilter::Debug)
        .try_init();
    let task = Arc::<SimpleTask>::default();
    let scheduler: Scheduler<_, _> = SchedulerOptions::<(), SimpleTask>::builder()
        .task(task)
        .shutdown_signals(vec![SignalKind::user_defined1()])
        .status_server_port(23220u16)
        .build()
        .into();
    let options = StartOptions::<anyhow::Error>::builder().interval_ms(5000u64).build();
    scheduler.start((), options).await.unwrap();
    unsafe {
        libc::raise(SignalKind::user_defined1().as_raw_value());
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
