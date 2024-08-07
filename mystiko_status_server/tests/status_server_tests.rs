use async_trait::async_trait;
use hyper::body::Buf;
use hyper::Body;
use mime::Mime;
use mystiko_status_server::{Status, StatusServer};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[tokio::test]
async fn test_status_server() {
    let expected_status = CounterStatus::builder().count(1234u32).build();
    let status_getter = CounterStatusGetter::builder().status(expected_status.clone()).build();
    let server = StatusServer::builder()
        .port(34231_u16)
        .status(Arc::new(Box::new(status_getter) as Box<dyn Status>))
        .bind_address("127.0.0.1")
        .build();
    server.start().await.unwrap();
    let client = hyper::Client::new();
    let status_uri: http::Uri = "http://127.0.0.1:34231/status".parse().unwrap();
    let status_response = client.get(status_uri.clone()).await.unwrap();
    let status_body = hyper::body::aggregate(status_response).await.unwrap();
    let status = serde_json::from_reader::<_, CounterStatus>(status_body.reader()).unwrap();
    assert_eq!(status, expected_status);
    server.stop().await.unwrap();
}

#[tokio::test]
async fn test_status_server_with_error() {
    let status_getter = CounterStatusGetter::builder().error_on_status(true).build();
    let server = StatusServer::builder()
        .port(34232_u16)
        .status(Arc::new(Box::new(status_getter) as Box<dyn Status>))
        .bind_address("127.0.0.1")
        .build();
    server.start().await.unwrap();
    let client = hyper::Client::new();
    let status_uri: http::Uri = "http://127.0.0.1:34232/status".parse().unwrap();
    let status_response = client.get(status_uri.clone()).await.unwrap();
    assert!(status_response.status().is_server_error());
}

#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
struct CounterStatusGetter {
    pub(crate) status: CounterStatus,
    pub(crate) error_on_status: bool,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
struct CounterStatus {
    count: u32,
}

#[async_trait]
impl Status for CounterStatusGetter {
    async fn status(&self) -> anyhow::Result<(Mime, Body)> {
        if self.error_on_status {
            return Err(anyhow::anyhow!("query status error"));
        }
        let counter_status = serde_json::to_string(&self.status)?;
        Ok((mime::APPLICATION_JSON, Body::from(counter_status)))
    }
}
