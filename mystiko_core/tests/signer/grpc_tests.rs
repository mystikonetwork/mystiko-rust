use ethers_core::types::TransactionRequest;
use mockall::mock;
use mystiko_core::{GrpcSigner, TransactionSigner};
use mystiko_grpc::GrpcServer;
use mystiko_protos::core::v1::transaction_service_server::TransactionServiceServer;
use mystiko_protos::core::v1::{GetAddressResponse, SendTransactionResponse};
use mystiko_protos::service::v1::{ClientOptions, ServerOptions};
use mystiko_utils::address::ethers_address_to_string;

#[tokio::test]
async fn test_get_address() {
    let mut service = MockTransactionService::new();
    service.expect_get_address().returning(|_| {
        Ok(tonic::Response::new(
            GetAddressResponse::builder()
                .address("0xE5ee3802314D587ABC9E0Ee3f1F42a9E82c9E1f1")
                .build(),
        ))
    });
    let server_options = ServerOptions::builder().port(42131_u32).build();
    let mut server = setup(service, server_options).await;
    let signer = GrpcSigner::connect(&ClientOptions::builder().port(42131_u32).build())
        .await
        .unwrap();
    assert_eq!(
        ethers_address_to_string(&signer.address().await.unwrap()),
        "0xE5ee3802314D587ABC9E0Ee3f1F42a9E82c9E1f1"
    );
    server.stop().await.unwrap();
}

#[tokio::test]
async fn test_send_transaction() {
    let mut service = MockTransactionService::new();
    service
        .expect_send_transaction()
        .withf(|req| req.get_ref().chain_id == 56_u64)
        .returning(|_| {
            Ok(tonic::Response::new(
                SendTransactionResponse::builder()
                    .chain_id(56_u64)
                    .tx_hash("0x8d874bd339d08093701771307ea134f2534610d1425bee4b3ed8e30854ac68b6")
                    .build(),
            ))
        });
    let server_options = ServerOptions::builder().port(42132_u32).build();
    let mut server = setup(service, server_options).await;
    let signer = GrpcSigner::connect(&ClientOptions::builder().port(42132_u32).build())
        .await
        .unwrap();
    let tx_hash = signer
        .send_transaction(
            56_u64,
            TransactionRequest::pay("0x70f657164e5b75689b64b7fd1fa275f334f28e18", 100).into(),
        )
        .await
        .unwrap();
    assert_eq!(
        format!("0x{:x}", tx_hash),
        "0x8d874bd339d08093701771307ea134f2534610d1425bee4b3ed8e30854ac68b6"
    );
    server.stop().await.unwrap();
}

#[tokio::test]
async fn test_personal_sign() {
    let signature = "0x8d874bd339d08093701771307ea134f2534610d1425bee4b3ed8e30854ac68b6";
    let mut service = MockTransactionService::new();
    service
        .expect_personal_sign()
        .withf(|req| req.get_ref().message == "message" && req.get_ref().address == "account")
        .returning(|_| {
            Ok(tonic::Response::new(
                mystiko_protos::core::v1::PersonalSignResponse::builder()
                    .signature(signature.to_string())
                    .build(),
            ))
        });

    let server_options = ServerOptions::builder().port(42133_u32).build();
    let mut server = setup(service, server_options).await;
    let signer = GrpcSigner::connect(&ClientOptions::builder().port(42133_u32).build())
        .await
        .unwrap();
    let response = signer
        .sign_message("account".to_string(), "message".to_string())
        .await
        .unwrap();
    assert_eq!(response, signature);
    server.stop().await.unwrap();
}

async fn setup(service: MockTransactionService, options: ServerOptions) -> GrpcServer {
    let mut server = GrpcServer::default();
    server
        .start(TransactionServiceServer::new(service), options)
        .await
        .unwrap();
    server
}

mock! {
    TransactionService {}

    #[tonic::async_trait]
    impl mystiko_protos::core::v1::transaction_service_server::TransactionService for TransactionService {
        async fn get_address(
            &self,
            request: tonic::Request<mystiko_protos::core::v1::GetAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<mystiko_protos::core::v1::GetAddressResponse>,
            tonic::Status,
        >;
        async fn send_transaction(
            &self,
            request: tonic::Request<mystiko_protos::core::v1::SendTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<mystiko_protos::core::v1::SendTransactionResponse>,
            tonic::Status,
        >;
        async fn personal_sign(
            &self,
            request: tonic::Request<mystiko_protos::core::v1::PersonalSignRequest>,
        ) -> std::result::Result<
            tonic::Response<mystiko_protos::core::v1::PersonalSignResponse>,
            tonic::Status,
        >;
    }
}
