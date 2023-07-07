use crate::common::TestServer;
use actix_web::test::{call_and_read_body_json, init_service, TestRequest};
use actix_web::web::Data;
use actix_web::App;
use mystiko_relayer_server::service::ping;
use mystiko_relayer_types::response::{ApiResponse, ResponseCode};
use mystiko_relayer_types::PingResponse;

#[actix_rt::test]
async fn test_ping() {
    let server = TestServer::new(None).await.unwrap();
    let app = init_service(App::new().app_data(Data::new(server.app_state.clone())).service(ping)).await;

    let req = TestRequest::get().uri("/ping").to_request();
    let resp: ApiResponse<PingResponse> = call_and_read_body_json(&app, req).await;
    let result = resp.result.unwrap();

    assert_eq!(resp.code, ResponseCode::Successful as i32);
    assert_eq!(result.api_version.len(), 1);
    assert_eq!(result.api_version.get(0).unwrap(), "v2");
}
