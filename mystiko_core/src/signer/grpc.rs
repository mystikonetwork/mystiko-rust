use crate::TransactionSigner;
use async_trait::async_trait;
use ethers_core::types::{Address, TransactionRequest, TxHash};
use mystiko_protos::core::v1::transaction_service_client::TransactionServiceClient;
use mystiko_protos::core::v1::{GetAddressRequest, SendTransactionRequest, Transaction};
use mystiko_protos::service::v1::ClientOptions;
use mystiko_utils::address::ethers_address_from_string;
use std::str::FromStr;
use thiserror::Error;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GrpcSigner {
    client: Mutex<TransactionServiceClient<Channel>>,
}

#[derive(Debug, Error)]
pub enum GrpcSignerError {
    #[error("failed to connect to grpc server: {0}")]
    ConnectError(anyhow::Error),
    #[error(transparent)]
    GrpcError(#[from] tonic::Status),
    #[error(transparent)]
    FromHexError(#[from] rustc_hex::FromHexError),
}

impl GrpcSigner {
    pub fn new(client: TransactionServiceClient<Channel>) -> Self {
        Self::builder().client(client).build()
    }

    pub async fn connect(options: &ClientOptions) -> Result<Self, GrpcSignerError> {
        Ok(Self::new(TransactionServiceClient::new(
            mystiko_grpc::connect(options)
                .await
                .map_err(GrpcSignerError::ConnectError)?,
        )))
    }
}

#[async_trait]
impl TransactionSigner for GrpcSigner {
    type Error = GrpcSignerError;

    async fn address(&self) -> Result<Address, Self::Error> {
        let response = self
            .client
            .lock()
            .await
            .get_address(GetAddressRequest::default())
            .await?
            .into_inner();
        Ok(ethers_address_from_string(response.address)?)
    }

    async fn send_transaction<T>(&self, chain_id: u64, tx: T) -> Result<TxHash, Self::Error>
    where
        T: Into<TransactionRequest> + Send + Sync,
    {
        let ethers_tx = tx.into();
        let tx: Transaction = ethers_tx.into();
        let request = SendTransactionRequest::builder()
            .chain_id(chain_id)
            .transaction(tx)
            .build();
        let response = self.client.lock().await.send_transaction(request).await?.into_inner();
        Ok(TxHash::from_str(&response.tx_hash)?)
    }
}
