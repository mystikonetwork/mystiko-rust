use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::{test, Responder};
use mystiko_relayer_types::response::{failed, success};

#[tokio::test]
async fn test_success_responder() {
    let response = success("Success");
    let request = test::TestRequest::default().to_http_request();

    let resp = response.respond_to(&request);
    let status = resp.status();

    let body = test::read_body(ServiceResponse::new(request, resp)).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "{\"code\":0,\"result\":\"Success\",\"error\":null}");
}

#[tokio::test]
async fn test_failed_responder() {
    let response = failed::<()>(None, Some("Error".to_string()));
    let request = test::TestRequest::default().to_http_request();

    let resp = response.respond_to(&request);
    let status = resp.status();

    let body = test::read_body(ServiceResponse::new(request, resp)).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "{\"code\":-1,\"result\":null,\"error\":\"Error\"}");
}
