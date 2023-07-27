use crate::errors::EtherScanError;
use ethers_providers::JsonRpcError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct EtherScanResponse<T> {
    pub status: String,
    pub message: String,
    pub result: Option<T>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: String,
    pub id: u64,
    pub error: Option<JsonRpcError>,
    pub result: Option<T>,
}

pub type Result<T> = anyhow::Result<T, EtherScanError>;