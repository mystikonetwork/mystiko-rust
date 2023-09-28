use crate::{AbortPolicy, RetryPolicy, SchedulerError, SchedulerTask};
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::oneshot::{channel, Receiver, Sender};
use tokio::sync::{Mutex, RwLock};
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct StartOptions<E, R: RetryPolicy<E> = Box<dyn RetryPolicy<E>>, A: AbortPolicy<E> = Box<dyn AbortPolicy<E>>> {
    pub interval_ms: u64,
    #[builder(default)]
    pub rounds_count: Option<u64>,
    #[builder(default)]
    pub start_delay_ms: Option<u64>,
    #[builder(default)]
    pub task_timeout_ms: Option<u64>,
    #[builder(default)]
    pub no_retry_on_timeout: bool,
    #[builder(default)]
    pub abort_on_timeout: bool,
    #[builder(default)]
    pub retry_policy: Option<Arc<R>>,
    #[builder(default)]
    pub abort_policy: Option<Arc<A>>,
    #[builder(default)]
    pub max_retry_times: u32,
    #[builder(default, setter(skip))]
    _phantom: std::marker::PhantomData<E>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub(crate) struct ScheduleExecutor<I, T: SchedulerTask<I>> {
    task: Arc<T>,
    #[builder(default)]
    started: RwLock<bool>,
    #[builder(default)]
    sleep_interrupt: Mutex<Option<Sender<()>>>,
    #[builder(default)]
    stop_receiver: Mutex<Option<Receiver<()>>>,
    #[builder(default, setter(skip))]
    _phantom: std::marker::PhantomData<I>,
}

impl<I, T> ScheduleExecutor<I, T>
where
    T: SchedulerTask<I>,
    I: Debug,
    T::Error: Debug,
{
    pub(crate) async fn started(&self) -> bool {
        *self.started.read().await
    }

    pub(crate) async fn start<R, A>(
        &self,
        run_args: I,
        options: StartOptions<T::Error, R, A>,
        start_sender: Sender<()>,
    ) -> Result<(), SchedulerError>
    where
        R: RetryPolicy<T::Error>,
        A: AbortPolicy<T::Error>,
    {
        if !self.set_started(true).await {
            start_sender
                .send(())
                .map_err(|_| SchedulerError::SendStartSignalError)?;
            let (stop_sender, stop_receiver) = channel::<()>();
            let mut sleep_ms = options.start_delay_ms;
            *self.stop_receiver.lock().await = Some(stop_receiver);
            let mut round = 1u64;
            while self.started().await {
                if let Some(sleep_ms) = sleep_ms {
                    let (sleep_sender, sleep_receiver) = channel::<()>();
                    *self.sleep_interrupt.lock().await = Some(sleep_sender);
                    let should_interrupt = interruptible_sleep(sleep_ms, sleep_receiver).await;
                    *self.sleep_interrupt.lock().await = None;
                    if should_interrupt {
                        break;
                    }
                }
                if self.run_task_with_retry::<R, A>(round, &run_args, &options).await {
                    log::error!("scheduler is aborted due to some error(s)");
                    break;
                }
                if let Some(rounds_count) = options.rounds_count {
                    if round >= rounds_count {
                        break;
                    }
                }
                round += 1;
                sleep_ms = Some(options.interval_ms);
            }
            self.set_started(false).await;
            stop_sender.send(()).map_err(|_| SchedulerError::SendStopSignalError)?;
        }
        Ok(())
    }

    pub(crate) async fn stop(&self) -> Result<(), SchedulerError> {
        if self.set_started(false).await {
            if let Some(sleep_interrupt) = self.sleep_interrupt.lock().await.take() {
                sleep_interrupt
                    .send(())
                    .map_err(|_| SchedulerError::SendSleepInterruptError)?;
            }
            if let Some(stop_receiver) = self.stop_receiver.lock().await.take() {
                let _ = stop_receiver.await;
            }
        }
        Ok(())
    }

    async fn set_started(&self, started: bool) -> bool {
        let mut last_state = *self.started.read().await;
        if last_state != started {
            let mut writer = self.started.write().await;
            last_state = *writer;
            if last_state != started {
                *writer = started;
            }
        }
        last_state
    }

    async fn run_task<R, A>(
        &self,
        round: u64,
        try_count: u32,
        run_args: &I,
        options: &StartOptions<T::Error, R, A>,
    ) -> Option<Result<(), T::Error>>
    where
        R: RetryPolicy<T::Error>,
        A: AbortPolicy<T::Error>,
    {
        if let Some(timeout_ms) = options.task_timeout_ms {
            match tokio::time::timeout(Duration::from_millis(timeout_ms), self.task.run(run_args)).await {
                Err(_) => {
                    log::error!(
                        "run_task(round={}, try_count={}) timed out after {}ms with args: {:?}",
                        round,
                        try_count,
                        timeout_ms,
                        run_args
                    );
                    None
                }
                Ok(result) => Some(result),
            }
        } else {
            Some(self.task.run(run_args).await)
        }
    }

    async fn run_task_with_retry<R, A>(&self, round: u64, run_args: &I, options: &StartOptions<T::Error, R, A>) -> bool
    where
        R: RetryPolicy<T::Error>,
        A: AbortPolicy<T::Error>,
    {
        let mut try_count = 0u32;
        let mut should_abort = false;
        while try_count <= options.max_retry_times {
            match self.run_task::<R, A>(round, try_count + 1, run_args, options).await {
                None => {
                    if options.abort_on_timeout {
                        should_abort = true;
                        break;
                    }
                    if options.no_retry_on_timeout {
                        break;
                    }
                }
                Some(Ok(_)) => break,
                Some(Err(err)) => {
                    log::error!(
                        "run_task(round={}, try_count={}) with args {:?} raised error: {:?}",
                        round,
                        try_count + 1,
                        run_args,
                        err
                    );
                    if let Some(abort_policy) = &options.abort_policy {
                        if abort_policy.should_abort(&err) {
                            should_abort = true;
                            break;
                        }
                    }
                    if let Some(retry_policy) = &options.retry_policy {
                        if !retry_policy.should_retry(&err) {
                            break;
                        }
                    }
                }
            }
            try_count += 1;
        }
        should_abort
    }
}

async fn interruptible_sleep(sleep_ms: u64, sleep_receiver: Receiver<()>) -> bool {
    tokio::select! {
        _ = tokio::time::sleep(Duration::from_millis(sleep_ms)) => { false },
        _ = sleep_receiver => {
            log::info!("scheduler sleep is interrupted!");
            true
        },
    }
}
