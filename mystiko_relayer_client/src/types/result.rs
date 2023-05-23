use crate::error::RelayerClientError;

pub type Result<T> = anyhow::Result<T, RelayerClientError>;
