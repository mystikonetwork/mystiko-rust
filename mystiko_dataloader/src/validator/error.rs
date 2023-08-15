use crate::handler::error::HandlerError;
use anyhow::Error as AnyhowError;
use ethers_contract::ContractError;
use mystiko_ethers::provider::factory::Provider;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidatorError {
    #[error("validator init error {0}")]
    ValidatorBuildError(String),
    #[error("validator data to be validated is empty")]
    ValidatorEmptyValidateDataError,
    #[error("validator validate error {0}")]
    ValidatorValidateError(String),
    #[error(transparent)]
    ContractError(#[from] ContractError<Provider>),
    #[error(transparent)]
    HandlerError(#[from] HandlerError),
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
}
