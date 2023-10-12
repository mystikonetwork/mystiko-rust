use crate::{ChainLoadedBlock, ContractLoadedBlock};
use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::Address;
use mystiko_protos::data::v1::{Commitment, Nullifier};
use mystiko_protos::sequencer::v1::sequencer_service_client::SequencerServiceClient;
use mystiko_protos::sequencer::v1::HealthCheckRequest;
use mystiko_protos::sequencer::v1::{ChainLoadedBlockRequest, GetCommitmentsRequest};
use mystiko_protos::sequencer::v1::{ContractLoadedBlockRequest, GetNullifiersRequest};
use mystiko_protos::sequencer::v1::{FetchChainRequest, FetchChainResponse};
use mystiko_protos::service::v1::ClientOptions;
use mystiko_utils::address::{ethers_address_to_bytes, ethers_address_to_string, string_address_from_bytes};
use mystiko_utils::convert::biguint_to_bytes;
use num_bigint::BigUint;
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
    #[error("failed to connect server: {0}")]
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
impl crate::SequencerClient<FetchChainRequest, FetchChainResponse, Commitment, Nullifier> for SequencerClient {
    type Error = SequencerClientError;

    async fn chain_loaded_block(&self, chain_id: u64, with_contracts: bool) -> Result<ChainLoadedBlock, Self::Error> {
        log::debug!(
            "sequencer_client received request of chain_loaded_block with chain_id={}, with_contracts={}",
            chain_id,
            with_contracts,
        );
        let request = ChainLoadedBlockRequest::builder()
            .chain_id(chain_id)
            .with_contracts(with_contracts)
            .build();
        let response = self.client.lock().await.chain_loaded_block(request).await?.into_inner();
        let contracts_response = response
            .contracts
            .into_iter()
            .map(|contract| {
                ContractLoadedBlock::builder()
                    .address(string_address_from_bytes(contract.contract_address))
                    .loaded_block(contract.block_number)
                    .build()
            })
            .collect::<Vec<_>>();
        Ok(ChainLoadedBlock::builder()
            .loaded_block(response.block_number)
            .contracts(contracts_response)
            .build())
    }

    async fn contract_loaded_block(&self, chain_id: u64, address: &Address) -> Result<u64, Self::Error> {
        log::debug!(
            "sequencer_client received request of contract_loaded_block with chain_id={}, address={}",
            chain_id,
            address
        );
        let contract_address = ethers_address_to_bytes(address);
        let request = ContractLoadedBlockRequest {
            chain_id,
            contract_address,
        };
        let response = self.client.lock().await.contract_loaded_block(request).await?;
        Ok(response.get_ref().block_number)
    }

    async fn fetch_chain(&self, request: FetchChainRequest) -> Result<FetchChainResponse, Self::Error> {
        log::debug!("sequencer_client received request of fetch_chain with {:?}", request);
        let response = self.client.lock().await.fetch_chain(request).await?;
        Ok(response.into_inner())
    }

    async fn get_commitments(
        &self,
        chain_id: u64,
        contract_address: &Address,
        commitment_hashes: Vec<BigUint>,
    ) -> Result<Vec<Commitment>, Self::Error> {
        log::debug!(
            "sequencer_client received request of get_commitments \
            with chain_id={}, contract_address={}, commitment_hashes={:?}",
            chain_id,
            ethers_address_to_string(contract_address),
            commitment_hashes.iter().map(BigUint::to_string).collect::<Vec<_>>(),
        );
        let request = GetCommitmentsRequest::builder()
            .chain_id(chain_id)
            .contract_address(ethers_address_to_bytes(contract_address))
            .commitment_hashes(commitment_hashes.iter().map(biguint_to_bytes).collect::<Vec<_>>())
            .build();
        let response = self.client.lock().await.get_commitments(request).await?;
        Ok(response.into_inner().commitments)
    }

    async fn get_nullifiers(
        &self,
        chain_id: u64,
        contract_address: &Address,
        nullifier_hashes: Vec<BigUint>,
    ) -> Result<Vec<Nullifier>, Self::Error> {
        log::debug!(
            "sequencer_client received request of get_nullifiers \
            with chain_id={}, contract_address={}, nullifier_hashes={:?}",
            chain_id,
            ethers_address_to_string(contract_address),
            nullifier_hashes.iter().map(BigUint::to_string).collect::<Vec<_>>(),
        );
        let request = GetNullifiersRequest::builder()
            .chain_id(chain_id)
            .contract_address(ethers_address_to_bytes(contract_address))
            .nullifier_hashes(nullifier_hashes.iter().map(biguint_to_bytes).collect::<Vec<_>>())
            .build();
        let response = self.client.lock().await.get_nullifiers(request).await?;
        Ok(response.into_inner().nullifiers)
    }

    async fn health_check(&self) -> Result<(), Self::Error> {
        log::debug!("sequencer_client received request of health_check");
        self.client.lock().await.health_check(HealthCheckRequest {}).await?;
        Ok(())
    }
}
