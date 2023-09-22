use crate::ScheduleExecutor;
use crate::{AbortPolicy, RetryPolicy, SchedulerError, SchedulerTask, StartOptions};
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::oneshot::{channel, Sender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SchedulerOptions<I, T: SchedulerTask<I> + Debug> {
    pub task: Arc<T>,
    #[cfg(feature = "signal")]
    #[builder(default)]
    pub shutdown_signals: Option<Vec<tokio::signal::unix::SignalKind>>,
    #[cfg(feature = "status")]
    #[builder(default = String::from("0.0.0.0"))]
    pub status_server_bind_address: String,
    #[cfg(feature = "status")]
    #[builder(default = 21818)]
    pub status_server_port: u16,
    #[builder(default, setter(skip))]
    _phantom: std::marker::PhantomData<I>,
}

#[derive(Debug)]
pub struct Scheduler<I, T: SchedulerTask<I> + Debug> {
    executor: Arc<ScheduleExecutor<I, T>>,
    join_handle: Mutex<Option<JoinHandle<Result<(), SchedulerError>>>>,
    #[cfg(feature = "signal")]
    shutdown_join_handle: Mutex<Option<JoinHandle<()>>>,
    #[cfg(feature = "signal")]
    shutdown_signals: Vec<tokio::signal::unix::SignalKind>,
    #[cfg(feature = "status")]
    status_server: Arc<crate::StatusServer>,
}

impl<I, T> Scheduler<I, T>
where
    T: SchedulerTask<I> + Send + Sync + Debug + 'static,
    I: Send + Sync + Debug + 'static,
    T::Error: Send + Sync + Debug + 'static,
{
    pub fn new<O>(options: O) -> Self
    where
        O: Into<SchedulerOptions<I, T>>,
    {
        let options = options.into();
        let executor = Arc::new(ScheduleExecutor::<I, T>::builder().task(options.task).build());
        Self {
            executor,
            join_handle: Mutex::default(),
            #[cfg(feature = "signal")]
            shutdown_join_handle: Mutex::default(),
            #[cfg(feature = "signal")]
            shutdown_signals: options.shutdown_signals.unwrap_or(vec![
                tokio::signal::unix::SignalKind::terminate(),
                tokio::signal::unix::SignalKind::quit(),
                tokio::signal::unix::SignalKind::interrupt(),
            ]),
            #[cfg(feature = "status")]
            status_server: Arc::new(
                crate::StatusServer::builder()
                    .bind_address(options.status_server_bind_address)
                    .port(options.status_server_port)
                    .build(),
            ),
        }
    }

    pub async fn started(&self) -> bool {
        self.executor.started().await
    }

    pub async fn start<R, A>(&self, run_args: I, options: StartOptions<T::Error, R, A>) -> Result<(), SchedulerError>
    where
        R: RetryPolicy<T::Error> + 'static,
        A: AbortPolicy<T::Error> + 'static,
    {
        if !self.started().await {
            let executor = self.executor.clone();
            let (start_sender, start_receiver) = channel::<()>();
            #[cfg(feature = "status")]
            let status_server = self.status_server.clone();
            *self.join_handle.lock().await = Some(tokio::spawn(async move {
                let result = start_executor(
                    run_args,
                    options,
                    start_sender,
                    executor,
                    #[cfg(feature = "status")]
                    status_server,
                )
                .await;
                match result {
                    Ok(_) => {
                        log::info!("scheduler has been stopped successfully");
                        Ok(())
                    }
                    Err(err) => {
                        log::error!("scheduler has been stopped with error: {}", err);
                        Err(err)
                    }
                }
            }));
            let _ = start_receiver.await;
            #[cfg(feature = "signal")]
            self.bind_shutdown_signals().await?;
            #[cfg(feature = "status")]
            self.status_server.start().await?;
        } else {
            log::warn!("scheduler has already been started, skipping this start");
        }
        Ok(())
    }

    pub async fn join(&self) -> Result<(), SchedulerError> {
        if let Some(join_handle) = self.join_handle.lock().await.take() {
            join_handle.await?
        } else {
            log::warn!("scheduler does not have join_handle, are you sure it was started?");
            Ok(())
        }
    }

    pub async fn stop(&self) -> Result<(), SchedulerError> {
        #[cfg(feature = "status")]
        self.status_server.stop().await?;
        self.executor.stop().await?;
        self.join().await
    }

    #[cfg(feature = "signal")]
    pub async fn wait_shutdown(&self) -> Result<(), SchedulerError> {
        if let Some(shutdown_join_handle) = self.shutdown_join_handle.lock().await.take() {
            shutdown_join_handle.await?;
        }
        self.stop().await
    }

    #[cfg(feature = "signal")]
    async fn bind_shutdown_signals(&self) -> Result<(), SchedulerError> {
        let mut shutdown_join_handle = self.shutdown_join_handle.lock().await;
        if shutdown_join_handle.is_none() {
            let signal_kinds = self.shutdown_signals.clone();
            let mut signals = signal_kinds
                .iter()
                .map(|signal| tokio::signal::unix::signal(*signal))
                .collect::<std::io::Result<Vec<_>>>()?;
            let join_handle = tokio::spawn(async move {
                let signal_futures = signals
                    .iter_mut()
                    .map(|signal| Box::pin(signal.recv()))
                    .collect::<Vec<_>>();
                futures::future::select_all(signal_futures).await;
                log::info!("received one or more signal(s) from {:?}", signal_kinds);
            });
            *shutdown_join_handle = Some(join_handle);
        }
        Ok(())
    }
}

impl<I, T> From<SchedulerOptions<I, T>> for Scheduler<I, T>
where
    T: SchedulerTask<I> + Send + Sync + Debug + 'static,
    I: Send + Sync + Debug + 'static,
    T::Error: Send + Sync + Debug + 'static,
{
    fn from(options: SchedulerOptions<I, T>) -> Self {
        Self::new(options)
    }
}

impl<I, T> From<Arc<T>> for SchedulerOptions<I, T>
where
    T: SchedulerTask<I> + Send + Sync + Debug,
{
    fn from(task: Arc<T>) -> Self {
        Self::builder().task(task).build()
    }
}

async fn start_executor<I, T, R, A>(
    run_args: I,
    options: StartOptions<T::Error, R, A>,
    start_sender: Sender<()>,
    executor: Arc<ScheduleExecutor<I, T>>,
    #[cfg(feature = "status")] status_server: Arc<crate::StatusServer>,
) -> Result<(), SchedulerError>
where
    T: SchedulerTask<I> + Send + Sync + Debug + 'static,
    I: Send + Sync + Debug + 'static,
    T::Error: Send + Sync + Debug + 'static,
    R: RetryPolicy<T::Error> + 'static,
    A: AbortPolicy<T::Error> + 'static,
{
    executor.start(run_args, options, start_sender).await?;
    #[cfg(feature = "status")]
    status_server.stop().await?;
    Ok(())
}
