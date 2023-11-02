use crate::TransactionSigner;
use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TxHash};
use mystiko_protos::core::v1::transaction_service_client::TransactionServiceClient;
use mystiko_protos::core::v1::{GetAddressRequest, SendTransactionRequest, Transaction};
use mystiko_protos::service::v1::ClientOptions;
use mystiko_utils::address::ethers_address_from_string;
use std::str::FromStr;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GrpcSigner {
    client: Mutex<TransactionServiceClient<Channel>>,
}

impl GrpcSigner {
    pub fn new(client: TransactionServiceClient<Channel>) -> Self {
        Self::builder().client(client).build()
    }

    pub async fn connect(options: &ClientOptions) -> Result<Self> {
        Ok(Self::new(TransactionServiceClient::new(
            mystiko_grpc::connect(options).await?,
        )))
    }
}

#[async_trait]
impl TransactionSigner for GrpcSigner {
    async fn address(&self) -> Result<Address> {
        let response = self
            .client
            .lock()
            .await
            .get_address(GetAddressRequest::default())
            .await?
            .into_inner();
        Ok(ethers_address_from_string(response.address)?)
    }

    async fn send_transaction(&self, chain_id: u64, tx: TypedTransaction) -> Result<TxHash> {
        let tx: Transaction = tx.into();
        let request = SendTransactionRequest::builder()
            .chain_id(chain_id)
            .transaction(tx)
            .build();
        let response = self.client.lock().await.send_transaction(request).await?.into_inner();
        Ok(TxHash::from_str(&response.tx_hash)?)
    }
}
