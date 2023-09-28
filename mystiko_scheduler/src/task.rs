use async_trait::async_trait;

#[async_trait]
pub trait SchedulerTask<I>: Send + Sync {
    type Error;

    async fn run(&self, args: &I) -> Result<(), Self::Error>;
}

pub trait RetryPolicy<E>: Send + Sync {
    fn should_retry(&self, error: &E) -> bool;
}

pub trait AbortPolicy<E>: Send + Sync {
    fn should_abort(&self, error: &E) -> bool;
}

impl<E> RetryPolicy<E> for Box<dyn RetryPolicy<E>> {
    fn should_retry(&self, error: &E) -> bool {
        self.as_ref().should_retry(error)
    }
}

impl<E> AbortPolicy<E> for Box<dyn AbortPolicy<E>> {
    fn should_abort(&self, error: &E) -> bool {
        self.as_ref().should_abort(error)
    }
}
