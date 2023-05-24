use ethers_providers::ProviderError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

pub type Result<T> = anyhow::Result<T, TxManagerError>;

#[derive(Error, Debug)]
pub enum TxManagerError {
    #[error("read file error {0}")]
    FileError(String),
    #[error(transparent)]
    SerdeJsonError(#[from] SerdeJsonError),
    #[error(transparent)]
    FromHexError(#[from] ff::hex::FromHexError),
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
    #[error("nonce manager error {0}")]
    NonceError(String),
    #[error("gas price error {0}")]
    GasPriceError(String),
    #[error("estimate gas error {0}")]
    EstimateGasError(String),
    #[error("send transaction error {0}")]
    SendTxError(String),
    #[error("transaction dropped")]
    TxDropped,
    #[error("confirm transaction error {0}")]
    ConfirmTxError(String),
}

impl PartialEq for TxManagerError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::FileError(_), Self::FileError(_))
                | (Self::SerdeJsonError(_), Self::SerdeJsonError(_))
                | (Self::FromHexError(_), Self::FromHexError(_))
                | (Self::ProviderError(_), Self::ProviderError(_))
                | (Self::NonceError(_), Self::NonceError(_))
                | (Self::GasPriceError(_), Self::GasPriceError(_))
                | (Self::EstimateGasError(_), Self::EstimateGasError(_))
                | (Self::SendTxError(_), Self::SendTxError(_))
                | (Self::TxDropped, Self::TxDropped)
                | (Self::ConfirmTxError(_), Self::ConfirmTxError(_)),
        )
    }
}
