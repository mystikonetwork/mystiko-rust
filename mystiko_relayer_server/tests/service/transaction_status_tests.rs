use crate::common::TestServer;
use actix_web::test::{call_and_read_body_json, init_service, TestRequest};
use actix_web::web::Data;
use actix_web::App;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_relayer_server::error::ResponseError;
use mystiko_relayer_server::service::transaction_status;
use mystiko_relayer_types::response::{ApiResponse, ResponseCode};
use mystiko_relayer_types::{RelayTransactStatusResponse, TransactRequestData, TransactStatus};
use mystiko_types::{BridgeType, CircuitType, TransactionType};

lazy_static! {
    static ref SERVER: AsyncOnce<TestServer> = AsyncOnce::new(async { TestServer::new(None).await.unwrap() });
}

#[actix_rt::test]
async fn test_id_not_found() {
    // create test server
    let server = SERVER.get().await;
    // init service
    let app = init_service(
        App::new()
            .app_data(Data::new(server.transaction_handler.clone()))
            .service(transaction_status),
    )
    .await;

    let req = TestRequest::get().uri("/transaction/status/123456").to_request();
    let resp: ApiResponse<RelayTransactStatusResponse> = call_and_read_body_json(&app, req).await;
    assert_eq!(resp.code, ResponseCode::TransactionNotFound as i32);
    assert!(resp.error.is_some());
    assert_eq!(
        resp.error.unwrap(),
        ResponseError::TransactionNotFound {
            id: "123456".parse().unwrap()
        }
        .to_string()
    );
}

#[actix_rt::test]
async fn test_successful() {
    // create test server
    let server = SERVER.get().await;
    // init service
    let app = init_service(
        App::new()
            .app_data(Data::new(server.transaction_handler.clone()))
            .service(transaction_status),
    )
    .await;

    // insert raw transaction data
    let request = TransactRequestData {
        contract_param: Default::default(),
        transaction_type: TransactionType::Withdraw,
        bridge_type: BridgeType::Loop,
        chain_id: 5,
        asset_symbol: "MTT".to_string(),
        asset_decimals: 16,
        pool_address: "0x4F416Acfd1153F9Af782056e68607227Af29D931".to_string(),
        circuit_type: CircuitType::Transaction1x0,
        signature: "0x800157ae47e94a156c42584190c33362b13ff94a7e8f5ef6ffd602c8d19ae\
        0684a4da6afd3c10bae9bd252dd20a9388d86c617bacb807a236a0385603e4086d61c"
            .to_string(),
    };
    let insert = server.transaction_handler.create_by_request(request).await;
    assert!(insert.is_ok());
    let id = insert.unwrap().id;

    let req = TestRequest::get()
        .uri(&format!("/transaction/status/{}", id))
        .to_request();
    let resp: ApiResponse<RelayTransactStatusResponse> = call_and_read_body_json(&app, req).await;
    assert_eq!(resp.code, ResponseCode::Successful as i32);
    assert!(resp.error.is_none());
    assert!(resp.result.is_some());

    let info = resp.result.unwrap();
    assert_eq!(info.uuid, id);
    assert_eq!(info.status, TransactStatus::Queued);
}
