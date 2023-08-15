use crate::handler::HandlerError;
use anyhow::Error as AnyhowError;
use ethers_contract::ContractError;
use mystiko_ethers::provider::factory::Provider;
use thiserror::Error;

pub type Result<T> = anyhow::Result<T, RuleValidatorError>;

#[derive(Error, Debug)]
pub enum RuleValidatorError {
    #[error("init error {0}")]
    BuildError(String),
    #[error("validate error {0}")]
    ValidateError(String),
    #[error(transparent)]
    ContractError(#[from] ContractError<Provider>),
    #[error(transparent)]
    HandlerError(#[from] HandlerError),
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
}
