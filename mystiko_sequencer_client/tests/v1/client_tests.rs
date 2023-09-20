use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::Address;
use mockall::mock;
use mystiko_grpc::GrpcServer;
use mystiko_protos::data::v1::{Commitment, CommitmentStatus};
use mystiko_protos::sequencer::v1::sequencer_service_client::SequencerServiceClient;
use mystiko_protos::sequencer::v1::sequencer_service_server::SequencerServiceServer;
use mystiko_protos::sequencer::v1::{
    ChainLoadedBlockRequest, ChainLoadedBlockResponse, ContractLoadedBlockRequest, ContractLoadedBlockResponse,
    FetchChainRequest, FetchChainResponse, FetchContractRequest, FetchContractResponse, HealthCheckRequest,
    HealthCheckResponse,
};
use mystiko_protos::service::v1::ClientOptions;
use mystiko_protos::service::v1::ServerOptions;
use mystiko_sequencer_client::v1::SequencerClient as SequencerClientV1;
use mystiko_sequencer_client::SequencerClient;
use mystiko_utils::address::ethers_address_to_bytes;
use tonic::{Code, Request, Response, Status};

mock! {
    #[derive(Debug)]
    SequencerService {}

    #[async_trait]
    impl mystiko_protos::sequencer::v1::sequencer_service_server::SequencerService for SequencerService {
        async fn fetch_chain(&self,request: Request<FetchChainRequest>,) -> Result<Response<FetchChainResponse>,Status>;
        async fn chain_loaded_block(&self, chain_id: Request<ChainLoadedBlockRequest>) -> Result<Response<ChainLoadedBlockResponse>,Status>;
        async fn contract_loaded_block(&self,request: Request<ContractLoadedBlockRequest>,) -> Result<Response<ContractLoadedBlockResponse>,Status>;
        async fn health_check(&self,request: Request<HealthCheckRequest>,) -> Result<Response<HealthCheckResponse>,Status>;

    }
}

async fn setup(service: MockSequencerService, options: ServerOptions) -> GrpcServer {
    let mut server = GrpcServer::default();
    server
        .start(SequencerServiceServer::new(service), options)
        .await
        .unwrap();
    server
}

#[tokio::test]
async fn test_chain_loaded_block() {
    let chain_id: u64 = 1;
    let block_number: u64 = 123;
    let mut service = MockSequencerService::new();
    service.expect_chain_loaded_block().returning(move |_| {
        Ok(Response::new(
            ChainLoadedBlockResponse::builder()
                .chain_id(chain_id)
                .block_number(block_number)
                .build(),
        ))
    });
    let options = ServerOptions::builder()
        .port(50151u32)
        .accept_http1(true)
        .enable_web(true)
        .build();
    let mut server = setup(service, options.clone()).await;

    let client_options = ClientOptions::builder().port(50151u32).build();
    let client = SequencerClientV1::connect(&client_options).await.unwrap();
    let result = client.chain_loaded_block(chain_id).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(block_number, result);
    server.stop().await.unwrap();

    let mut service = MockSequencerService::new();
    let esg = "unknown err";
    service
        .expect_chain_loaded_block()
        .returning(move |_| Err(Status::new(Code::Unknown, esg)));
    let mut server = setup(service, options.clone()).await;
    let result = client.chain_loaded_block(chain_id).await;
    assert!(result.is_err());
    let result = result.unwrap_err();
    assert!(result.to_string().contains(esg));
    server.stop().await.unwrap();
}

#[tokio::test]
async fn test_contract_loaded_block() {
    let chain_id: u64 = 1;
    let address = "0xCB255075f38C75EAf2DE8A72897649dba9B90299".parse::<Address>().unwrap();
    let block_number: u64 = 123;
    let mut service = MockSequencerService::new();
    service.expect_contract_loaded_block().returning(move |_| {
        Ok(Response::new(
            ContractLoadedBlockResponse::builder()
                .chain_id(chain_id)
                .contract_address(ethers_address_to_bytes(&address))
                .block_number(block_number)
                .build(),
        ))
    });
    let options = ServerOptions::builder()
        .port(50152u32)
        .accept_http1(true)
        .enable_web(true)
        .build();
    let mut server = setup(service, options.clone()).await;

    let client_options = ClientOptions::builder().port(50152u32).build();
    let client = SequencerClientV1::connect(&client_options).await.unwrap();
    let result = client.contract_loaded_block(chain_id, &address).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(block_number, result);
    server.stop().await.unwrap();

    let mut service = MockSequencerService::new();
    let esg = "address not found";
    service
        .expect_contract_loaded_block()
        .returning(move |_| Err(Status::new(Code::NotFound, esg)));
    let mut server = setup(service, options.clone()).await;
    let result = client.contract_loaded_block(chain_id, &address).await;
    assert!(result.is_err());
    let result = result.unwrap_err();
    assert!(result.to_string().contains(esg));
    server.stop().await.unwrap();
}

#[tokio::test]
async fn test_fetch_chain() {
    let chain_id: u64 = 1;
    let address = "0xCB255075f38C75EAf2DE8A72897649dba9B90299".parse::<Address>().unwrap();
    let test_start_block: u64 = 46013154;
    let test_end_block: u64 = 46213153;

    let mut service = MockSequencerService::new();
    service.expect_fetch_chain().returning(move |_| {
        Ok(Response::new(
            FetchChainResponse::builder()
                .chain_id(chain_id)
                .contracts(vec![FetchContractResponse::builder()
                    .contract_address(ethers_address_to_bytes(&address))
                    .start_block(test_start_block)
                    .end_block(test_end_block)
                    .commitments(vec![Commitment::builder()
                        .commitment_hash(vec![1u8, 2u8, 3u8])
                        .status(CommitmentStatus::Queued as i32)
                        .block_number(46015154u64)
                        .leaf_index(Some(10u64))
                        .encrypted_note(Some(vec![1u8, 2u8, 3u8]))
                        .queued_transaction_hash(Some(vec![4u8, 5u8, 6u8]))
                        .build()])
                    .nullifiers(vec![])
                    .build()])
                .build(),
        ))
    });
    let options = ServerOptions::builder()
        .port(50153u32)
        .accept_http1(true)
        .enable_web(true)
        .build();
    let mut server = setup(service, options.clone()).await;
    let client_options = ClientOptions::builder().port(50153u32).build();
    let client = SequencerClientV1::connect(&client_options).await.unwrap();

    let request = FetchChainRequest::builder()
        .chain_id(chain_id)
        .start_block(test_start_block)
        .target_block(test_end_block)
        .is_full(false)
        .contracts(vec![FetchContractRequest::builder()
            .contract_address(ethers_address_to_bytes(&address))
            .start_block(test_start_block)
            .target_block(test_end_block)
            .build()])
        .build();
    let result = client.fetch_chain(request.clone()).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(chain_id, result.chain_id);
    assert_eq!(1, result.contracts.len());
    let result = &result.contracts[0];
    assert_eq!(ethers_address_to_bytes(&address), result.contract_address);
    assert_eq!(test_start_block, result.start_block);
    assert_eq!(test_end_block, result.end_block);
    assert_eq!(0, result.nullifiers.len());
    assert_eq!(1, result.commitments.len());
    let result = &result.commitments[0];
    assert_eq!(2, result.status);
    server.stop().await.unwrap();

    let mut service = MockSequencerService::new();
    let esg = "cancelled err";
    service
        .expect_fetch_chain()
        .returning(move |_| Err(Status::new(Code::Cancelled, esg)));
    let mut server = setup(service, options.clone()).await;
    let result = client.fetch_chain(request).await;
    assert!(result.is_err());
    let result = result.unwrap_err();
    assert!(result.to_string().contains(esg));
    server.stop().await.unwrap();
}

#[tokio::test]
async fn test_health_check() {
    let mut service = MockSequencerService::new();
    service
        .expect_health_check()
        .returning(move |_| Ok(Response::new(HealthCheckResponse::builder().build())));
    let options = ServerOptions::builder()
        .port(50154u32)
        .accept_http1(true)
        .enable_web(true)
        .build();
    let mut server = setup(service, options.clone()).await;

    let client_options = ClientOptions::builder().port(50154u32).build();
    let channel = mystiko_grpc::connect(&client_options).await.unwrap();
    let client: SequencerClientV1 = SequencerServiceClient::new(channel).into();
    let result = client.health_check().await;
    assert!(result.is_ok());
    server.stop().await.unwrap();

    let mut service = MockSequencerService::new();
    let esg = "aborted err";
    service
        .expect_health_check()
        .returning(move |_| Err(Status::new(Code::Aborted, esg)));
    let mut server = setup(service, options.clone()).await;
    let result = client.health_check().await;
    assert!(result.is_err());
    let result = result.unwrap_err();
    assert!(result.to_string().contains(esg));
    server.stop().await.unwrap();
}
