use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::Address;
use mystiko_protos::sequencer::v1::sequencer_service_client::SequencerServiceClient;
use mystiko_protos::sequencer::v1::ChainLoadedBlockRequest;
use mystiko_protos::sequencer::v1::ContractLoadedBlockRequest;
use mystiko_protos::sequencer::v1::HealthCheckRequest;
use mystiko_protos::sequencer::v1::{FetchChainRequest, FetchChainResponse};
use mystiko_protos::service::v1::ClientOptions;
use mystiko_utils::address::ethers_address_to_bytes;
use thiserror::Error;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SequencerClient {
    client: Mutex<SequencerServiceClient<Channel>>,
}

#[derive(Debug, Error)]
pub enum SequencerClientError {
    #[error("unknown error: {0}")]
    UnknownError(String),
    #[error("connect error: {0}")]
    ConnectError(String),
    #[error(transparent)]
    TonicStatusError(#[from] tonic::Status),
}

impl SequencerClient {
    pub fn new(client: SequencerServiceClient<Channel>) -> Self {
        Self {
            client: Mutex::new(client),
        }
    }

    pub async fn connect(options: &ClientOptions) -> Result<Self> {
        let channel = mystiko_grpc::connect(options)
            .await
            .map_err(|err| SequencerClientError::ConnectError(err.to_string()))?;
        Ok(Self::new(SequencerServiceClient::new(channel)))
    }
}

impl From<SequencerServiceClient<Channel>> for SequencerClient {
    fn from(client: SequencerServiceClient<Channel>) -> Self {
        SequencerClient::new(client)
    }
}

#[async_trait]
impl crate::SequencerClient<FetchChainRequest, FetchChainResponse> for SequencerClient {
    async fn chain_loaded_block(&self, chain_id: u64) -> Result<u64> {
        log::debug!("sequencer client load block from chain {}", chain_id);
        let request = ChainLoadedBlockRequest { chain_id };
        let mut client = self.client.lock().await;
        client
            .chain_loaded_block(request)
            .await
            .map(|resp| resp.get_ref().block_number)
            .map_err(|err| {
                match err {
                    err if matches!(err, tonic::Status { .. }) => SequencerClientError::TonicStatusError(err),
                    _ => SequencerClientError::UnknownError(err.to_string()),
                }
                .into()
            })
    }

    async fn contract_loaded_block(&self, chain_id: u64, address: &Address) -> Result<u64> {
        log::debug!(
            "sequencer client load block from chain {}, contract {}",
            chain_id,
            address
        );
        let contract_address = ethers_address_to_bytes(address);
        let request = ContractLoadedBlockRequest {
            chain_id,
            contract_address,
        };
        let mut client = self.client.lock().await;
        client
            .contract_loaded_block(request)
            .await
            .map(|resp| resp.get_ref().block_number)
            .map_err(|err| {
                match err {
                    err if matches!(err, tonic::Status { .. }) => SequencerClientError::TonicStatusError(err),
                    _ => SequencerClientError::UnknownError(err.to_string()),
                }
                .into()
            })
    }

    async fn fetch_chain(&self, request: FetchChainRequest) -> Result<FetchChainResponse> {
        log::debug!(
            "sequencer client fetch chain {}, expected block range [{}, {}]",
            request.chain_id,
            request.start_block,
            request.target_block,
        );
        let mut client = self.client.lock().await;
        client
            .fetch_chain(request)
            .await
            .map(|resp| resp.into_inner())
            .map_err(|err| {
                match err {
                    err if matches!(err, tonic::Status { .. }) => SequencerClientError::TonicStatusError(err),
                    _ => SequencerClientError::UnknownError(err.to_string()),
                }
                .into()
            })
    }
    async fn health_check(&self) -> Result<()> {
        log::debug!("sequencer client health_check");
        let mut client = self.client.lock().await;
        client
            .health_check(HealthCheckRequest {})
            .await
            .map(|_| ())
            .map_err(|err| {
                match err {
                    err if matches!(err, tonic::Status { .. }) => SequencerClientError::TonicStatusError(err),
                    _ => SequencerClientError::UnknownError(err.to_string()),
                }
                .into()
            })
    }
}
