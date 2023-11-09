extern crate anyhow;
extern crate ethers_contract;
extern crate ethers_core;
extern crate futures;
extern crate log;
extern crate mystiko_ethers;
extern crate mystiko_relayer_abi;
extern crate mystiko_relayer_config;
extern crate mystiko_types;
extern crate reqwest;
extern crate rust_decimal;
extern crate rustc_hex;
extern crate serde;
extern crate serde_json;
extern crate thiserror;
extern crate tokio;
extern crate typed_builder;
extern crate validator;
pub mod error;
pub mod request;
pub mod types;
pub mod v2;

use crate::types::register::RegisterInfo;
use crate::types::result::Result;
use async_trait::async_trait;
use mystiko_relayer_types::{
    RegisterInfoRequest, RelayTransactRequest, RelayTransactResponse, RelayTransactStatusRequest,
    RelayTransactStatusResponse, WaitingTransactionRequest,
};

#[async_trait]
pub trait RelayerClient: Send + Sync {
    async fn all_register_info(&self, request: RegisterInfoRequest) -> Result<Vec<RegisterInfo>>;

    async fn relay_transact(&self, request: RelayTransactRequest) -> Result<RelayTransactResponse>;

    async fn relay_transaction_status(
        &self,
        request: RelayTransactStatusRequest,
    ) -> Result<RelayTransactStatusResponse>;

    async fn wait_transaction(&self, request: WaitingTransactionRequest) -> Result<RelayTransactStatusResponse>;
    async fn handshake(&self, url: &str) -> Result<bool>;
}
