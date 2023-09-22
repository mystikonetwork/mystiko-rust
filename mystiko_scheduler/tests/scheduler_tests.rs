use async_trait::async_trait;
use mystiko_scheduler::{AbortPolicy, RetryPolicy, Scheduler, SchedulerOptions, SchedulerTask, StartOptions};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use typed_builder::TypedBuilder;

#[tokio::test]
async fn test_start_and_stop() {
    setup();
    let task = Arc::<CounterTask>::default();
    let scheduler_options = SchedulerOptions::<Option<_>, CounterTask>::builder()
        .task(task)
        .status_server_port(23211u16)
        .build();
    let scheduler = Scheduler::new(scheduler_options);
    assert!(!scheduler.started().await);
    let options = StartOptions::<anyhow::Error>::builder()
        .interval_ms(5000u64)
        .start_delay_ms(10u64)
        .build();
    scheduler.start(None, options).await.unwrap();
    assert!(scheduler.started().await);
    scheduler.stop().await.unwrap();
    assert!(!scheduler.started().await);
}

#[tokio::test]
async fn test_start_multiple_times() {
    setup();
    let task = Arc::<CounterTask>::default();
    let scheduler_options = SchedulerOptions::<Option<_>, CounterTask>::builder()
        .task(task)
        .status_server_port(23212u16)
        .build();
    let scheduler = Scheduler::new(scheduler_options);
    scheduler
        .start(
            None,
            StartOptions::<anyhow::Error>::builder().interval_ms(5000u64).build(),
        )
        .await
        .unwrap();
    assert!(scheduler.started().await);
    scheduler
        .start(
            None,
            StartOptions::<anyhow::Error>::builder().interval_ms(5000u64).build(),
        )
        .await
        .unwrap();
    assert!(scheduler.started().await);
    scheduler.stop().await.unwrap();
    assert!(!scheduler.started().await);
}

#[tokio::test]
async fn test_stop_before_start() {
    setup();
    let task = Arc::<CounterTask>::default();
    let scheduler_options = SchedulerOptions::<Option<_>, CounterTask>::builder()
        .task(task)
        .status_server_port(23213u16)
        .build();
    let scheduler = Scheduler::new(scheduler_options);
    scheduler.join().await.unwrap();
    scheduler.stop().await.unwrap();
}

#[tokio::test]
async fn test_task_timeout() {
    setup();
    let task = Arc::new(CounterTask::builder().sleep_ms(10000u64).build());
    let scheduler_options = SchedulerOptions::<Option<_>, CounterTask>::builder()
        .task(task.clone())
        .status_server_port(23214u16)
        .build();
    let scheduler = Scheduler::new(scheduler_options);
    let options = StartOptions::<anyhow::Error>::builder()
        .interval_ms(2u64)
        .task_timeout_ms(1u64)
        .rounds_count(2u64)
        .no_retry_on_timeout(true)
        .build();
    scheduler.start(None, options).await.unwrap();
    scheduler.join().await.unwrap();
    assert_eq!(*task.counter.lock().await, 2u32);
}

#[tokio::test]
async fn test_task_timeout_with_abort() {
    setup();
    let task = Arc::new(CounterTask::builder().sleep_ms(10000u64).build());
    let scheduler_options = SchedulerOptions::<Option<_>, CounterTask>::builder()
        .task(task.clone())
        .status_server_port(23215u16)
        .build();
    let scheduler = Scheduler::new(scheduler_options);
    let options = StartOptions::<anyhow::Error>::builder()
        .interval_ms(2u64)
        .task_timeout_ms(1u64)
        .abort_on_timeout(true)
        .build();
    scheduler.start(None, options).await.unwrap();
    scheduler.join().await.unwrap();
    assert_eq!(*task.counter.lock().await, 1u32);
}

#[tokio::test]
async fn test_task_timeout_with_retry() {
    setup();
    let task = Arc::new(CounterTask::builder().sleep_ms(10000u64).build());
    let scheduler_options = SchedulerOptions::<Option<_>, CounterTask>::builder()
        .task(task.clone())
        .status_server_port(23216u16)
        .build();
    let scheduler = Scheduler::new(scheduler_options);
    let options = StartOptions::<anyhow::Error>::builder()
        .interval_ms(2u64)
        .task_timeout_ms(1u64)
        .max_retry_times(2u32)
        .rounds_count(2u64)
        .build();
    scheduler.start(None, options).await.unwrap();
    scheduler.join().await.unwrap();
    assert_eq!(*task.counter.lock().await, 6u32);
}

#[tokio::test]
async fn test_retry_policy() {
    setup();
    let task = Arc::<CounterTask>::default();
    let scheduler_options = SchedulerOptions::<Option<_>, CounterTask>::builder()
        .task(task.clone())
        .status_server_port(23217u16)
        .build();
    let scheduler = Scheduler::new(scheduler_options);
    let options = StartOptions::<anyhow::Error>::builder()
        .interval_ms(2u64)
        .max_retry_times(2u32)
        .rounds_count(2u64)
        .retry_policy(Arc::new(
            Box::new(CounterRetryPolicy) as Box<dyn RetryPolicy<anyhow::Error>>
        ))
        .build();
    scheduler
        .start(Some(anyhow::anyhow!("retryable error")), options)
        .await
        .unwrap();
    scheduler.join().await.unwrap();
    assert_eq!(*task.counter.lock().await, 6u32);

    let options = StartOptions::<anyhow::Error>::builder()
        .interval_ms(2u64)
        .max_retry_times(2u32)
        .rounds_count(2u64)
        .retry_policy(Arc::new(
            Box::new(CounterRetryPolicy) as Box<dyn RetryPolicy<anyhow::Error>>
        ))
        .build();
    scheduler
        .start(Some(anyhow::anyhow!("non-retryable error")), options)
        .await
        .unwrap();
    scheduler.join().await.unwrap();
    assert_eq!(*task.counter.lock().await, 8u32);
}

#[tokio::test]
async fn test_abort_policy() {
    setup();
    let task = Arc::new(CounterTask::builder().abort_on_count(4u32).build());
    let scheduler_options = SchedulerOptions::<Option<_>, CounterTask>::builder()
        .task(task.clone())
        .status_server_port(23218u16)
        .build();
    let scheduler = Scheduler::new(scheduler_options);
    let options = StartOptions::<anyhow::Error>::builder()
        .interval_ms(2u64)
        .abort_policy(Arc::new(
            Box::new(CounterAbortPolicy) as Box<dyn AbortPolicy<anyhow::Error>>
        ))
        .build();
    scheduler.start(None, options).await.unwrap();
    scheduler.join().await.unwrap();
    assert!(!scheduler.started().await);
    assert_eq!(*task.counter.lock().await, 4u32);
}

#[tokio::test]
async fn test_http_status() {
    setup();
    let task = Arc::new(CounterTask::builder().abort_on_count(4u32).build());
    let scheduler_options = SchedulerOptions::<Option<_>, CounterTask>::builder()
        .task(task)
        .status_server_port(23219u16)
        .build();
    let scheduler = Scheduler::new(scheduler_options);
    let client = hyper::Client::new();
    let status_uri: http::Uri = "http://localhost:23219/status".parse().unwrap();
    assert!(client.get(status_uri.clone()).await.is_err());

    let options = StartOptions::<anyhow::Error>::builder().interval_ms(1000u64).build();
    scheduler.start(None, options).await.unwrap();

    let status_response = client.get(status_uri.clone()).await.unwrap();
    assert!(status_response.status().is_success());

    scheduler.stop().await.unwrap();
    assert!(client.get(status_uri).await.is_err());
}

fn setup() {
    let _ = env_logger::builder()
        .filter_module("mystiko_scheduler", log::LevelFilter::Debug)
        .try_init();
}

#[derive(Default, Debug, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
struct CounterTask {
    pub(crate) counter: Mutex<u32>,
    pub(crate) abort_on_count: Option<u32>,
    pub(crate) sleep_ms: Option<u64>,
}

#[derive(Debug, Clone, Default)]
struct CounterAbortPolicy;

#[derive(Debug, Clone, Default)]
struct CounterRetryPolicy;

#[async_trait]
impl SchedulerTask<Option<anyhow::Error>> for CounterTask {
    type Error = anyhow::Error;

    async fn run(&self, error: &Option<anyhow::Error>) -> Result<(), Self::Error> {
        *self.counter.lock().await += 1;
        if let Some(abort_on_count) = self.abort_on_count {
            if *self.counter.lock().await == abort_on_count {
                return Err(anyhow::anyhow!("abort on count"));
            }
        }
        if let Some(error) = error {
            return Err(anyhow::anyhow!(error.to_string()));
        }
        if let Some(sleep_ms) = self.sleep_ms {
            tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
        }
        Ok(())
    }
}

impl AbortPolicy<anyhow::Error> for CounterAbortPolicy {
    fn should_abort(&self, error: &anyhow::Error) -> bool {
        error.to_string() == "abort on count"
    }
}

impl RetryPolicy<anyhow::Error> for CounterRetryPolicy {
    fn should_retry(&self, error: &anyhow::Error) -> bool {
        error.to_string() == "retryable error"
    }
}
