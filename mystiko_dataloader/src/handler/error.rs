use anyhow::Error as AnyhowError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
}
