use anyhow::Error as AnyhowError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidatorError {
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
}
