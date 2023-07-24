use crate::common::ENV_MUTEX;
use crate::test_files::load::load_commitment_logs;
use mockito::{Matcher, Mock, Server, ServerGuard};
use mystiko_roller::chain::explorer::{ExplorerApiResponse, ExplorerRpcResponse, ExplorerStub};
use mystiko_roller::chain::ChainDataGiver;
use mystiko_roller::common::error::RollerError;
use mystiko_roller::config::roller::ChainDataSource;
use serde_json::json;
use std::env;

#[tokio::test]
pub async fn test_explorer_blcok_number() {
    let stub = create_explorer_stub("", "test_explorer").await;
    assert_eq!(stub.data_source(), ChainDataSource::Explorer);
    let block_number = stub.get_latest_block_number(401, "").await;
    assert!(matches!(block_number.err().unwrap(), RollerError::ReqwestError(_)));

    let (server, mocks) = create_explorer_server(
        "0x7f88f2a3cf18e96844e14cae59ec97b908734c01",
        "test_explorer",
        Some(true),
        None,
        None,
    )
    .await;
    let stub = create_explorer_stub(&server.url(), "test_explorer").await;
    let block_number = stub.get_latest_block_number(401, "").await;
    assert_eq!(block_number.unwrap(), 256);
    for mock in mocks {
        mock.assert_async().await;
    }
}

#[tokio::test]
pub async fn test_explorer_stub_included() {
    let stub = create_explorer_stub("", "test_explorer").await;
    let included_count = stub
        .get_included_count(1, "0x7f88f2a3cf18e96844e14cae59ec97b908734c01")
        .await;
    assert!(matches!(included_count.err().unwrap(), RollerError::ReqwestError(_)));

    let (server, mocks) = create_explorer_server(
        "0x7f88f2a3cf18e96844e14cae59ec97b908734c01",
        "test_explorer",
        None,
        Some(true),
        None,
    )
    .await;
    let stub = create_explorer_stub(&server.url(), "test_explorer").await;
    let included_count = stub.get_included_count(1, "error_address").await;
    assert!(matches!(included_count.err().unwrap(), RollerError::ReqwestError(_)));

    let included_count = stub
        .get_included_count(1, "0x7f88f2a3cf18e96844e14cae59ec97b908734c01")
        .await;
    assert_eq!(included_count.unwrap(), 512);
    for mock in mocks {
        mock.assert_async().await;
    }
}

#[tokio::test]
pub async fn test_explorer_stub_cms() {
    let stub = create_explorer_stub("", "test_explorer").await;
    let included_count = stub
        .get_queued_commitments(1, "0x7f88f2a3cf18e96844e14cae59ec97b908734c01", 100, 200)
        .await;
    assert!(matches!(included_count.err().unwrap(), RollerError::ReqwestError(_)));

    let (server, mocks) = create_explorer_server(
        "0x7f88f2a3cf18e96844e14cae59ec97b908734c01",
        "test_explorer",
        None,
        None,
        Some(true),
    )
    .await;
    let stub = create_explorer_stub(&server.url(), "test_explorer").await;
    let cms = stub
        .get_queued_commitments(1, "0x7f88f2a3cf18e96844e14cae59ec97b908734c01", 1, 100)
        .await;
    assert_eq!(cms.unwrap().len(), 35);
    for mock in mocks {
        mock.assert_async().await;
    }
}

#[tokio::test]
pub async fn test_explorer_api_error_response() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/api")
        .match_query(Matcher::AnyOf(vec![Matcher::UrlEncoded(
            "action".into(),
            "getLogs".into(),
        )]))
        .with_status(500)
        .with_body("")
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let stub = create_explorer_stub(&server.url(), "test_explorer").await;
    let result = stub.get_queued_commitments(403, "", 100, 1000).await;
    assert!(
        matches!(result.as_ref().err().unwrap(), RollerError::ReqwestError(_)),
        "{:?}",
        result.as_ref().err().unwrap()
    );
    mock.assert_async().await;
}

#[tokio::test]
pub async fn test_explorer_api_error_response2() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/api")
        .match_query(Matcher::AnyOf(vec![Matcher::UrlEncoded(
            "action".into(),
            "getLogs".into(),
        )]))
        .with_status(200)
        .with_body("")
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let stub = create_explorer_stub(&server.url(), "test_explorer").await;
    let result = stub.get_queued_commitments(403, "", 100, 1000).await;
    assert!(
        matches!(result.as_ref().err().unwrap(), RollerError::ExplorerError(_)),
        "{:?}",
        result.as_ref().err().unwrap()
    );
    mock.assert_async().await;
}

#[tokio::test]
pub async fn test_explorer_api_error_response3() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/api")
        .match_query(Matcher::AnyOf(vec![Matcher::UrlEncoded(
            "action".into(),
            "getLogs".into(),
        )]))
        .with_status(200)
        .with_body("")
        .with_header("X-Custom-Foo", "Bar")
        .create_async()
        .await;

    let stub = create_explorer_stub(&server.url(), "test_explorer").await;
    let result = stub.get_queued_commitments(403, "", 100, 1000).await;
    assert!(
        matches!(result.as_ref().err().unwrap(), RollerError::ExplorerError(_)),
        "{:?}",
        result.as_ref().err().unwrap()
    );
    mock.assert_async().await;
}

#[tokio::test]
pub async fn test_explorer_api_error_response4() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/api")
        .match_query(Matcher::AnyOf(vec![Matcher::UrlEncoded(
            "action".into(),
            "getLogs".into(),
        )]))
        .with_status(200)
        .with_body("")
        .with_header("content-type", "application/error")
        .create_async()
        .await;

    let stub = create_explorer_stub(&server.url(), "test_explorer").await;
    let result = stub.get_queued_commitments(403, "", 100, 1000).await;
    assert!(
        matches!(result.as_ref().err().unwrap(), RollerError::ExplorerError(_)),
        "{:?}",
        result.as_ref().err().unwrap()
    );
    mock.assert_async().await;
}

#[tokio::test]
pub async fn test_explorer_api_error_response5() {
    let mut server = Server::new_async().await;
    let commitment_rsp = ExplorerApiResponse {
        status: "0".to_string(),
        message: "No records found".to_string(),
        result: json!(vec!["error1".to_string()]),
    };
    let commitment_rsp = serde_json::to_string(&commitment_rsp).unwrap();
    let mock = server
        .mock("GET", "/api")
        .match_query(Matcher::AnyOf(vec![Matcher::UrlEncoded(
            "action".into(),
            "getLogs".into(),
        )]))
        .with_status(200)
        .with_body(commitment_rsp)
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let stub = create_explorer_stub(&server.url(), "test_explorer").await;
    let result = stub.get_queued_commitments(403, "", 100, 1000).await;
    assert!(
        matches!(result.as_ref().err().unwrap(), RollerError::ExplorerError(_)),
        "{:?}",
        result.as_ref().err().unwrap()
    );
    mock.assert_async().await;
}

#[tokio::test]
pub async fn test_explorer_api_error_response6() {
    let mut server = Server::new_async().await;
    let commitment_rsp = ExplorerApiResponse {
        status: "2".to_string(),
        message: "Error".to_string(),
        result: json!(vec!["error2".to_string()]),
    };
    let commitment_json = serde_json::to_string(&commitment_rsp).unwrap();
    let mock = server
        .mock("GET", "/api")
        .match_query(Matcher::AnyOf(vec![Matcher::UrlEncoded(
            "action".into(),
            "getLogs".into(),
        )]))
        .with_status(200)
        .with_body(commitment_json)
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let stub = create_explorer_stub(&server.url(), "test_explorer").await;
    let result = stub.get_queued_commitments(403, "", 100, 1000).await;
    assert!(
        matches!(result.as_ref().err().unwrap(), RollerError::ExplorerError(_)),
        "{:?}",
        result.as_ref().err().unwrap()
    );
    mock.assert_async().await;
}

#[tokio::test]
pub async fn test_explorer_api_error_response7() {
    let mut server = Server::new_async().await;
    let commitment_rsp = ExplorerApiResponse {
        status: "1".to_string(),
        message: "OK".to_string(),
        result: json!(vec!["error3".to_string()]),
    };
    let commitment_json = serde_json::to_string(&commitment_rsp).unwrap();
    let mock = server
        .mock("GET", "/api")
        .match_query(Matcher::AnyOf(vec![Matcher::UrlEncoded(
            "action".into(),
            "getLogs".into(),
        )]))
        .with_status(200)
        .with_body(commitment_json)
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let stub = create_explorer_stub(&server.url(), "test_explorer").await;
    let result = stub.get_queued_commitments(403, "", 100, 1000).await;
    assert!(
        matches!(result.as_ref().err().unwrap(), RollerError::ExplorerError(_)),
        "{:?}",
        result.as_ref().err().unwrap()
    );
    mock.assert_async().await;
}

async fn create_explorer_server(
    contract_address: &str,
    key: &str,
    block: Option<bool>,
    include_count: Option<bool>,
    cms: Option<bool>,
) -> (ServerGuard, Vec<Mock>) {
    let mut server = Server::new_async().await;
    let mut mocks = vec![];
    if let Some(_) = block {
        let block_number_rsp = ExplorerRpcResponse {
            jsonrpc: "1.0".to_string(),
            id: 1,
            result: serde_json::Value::String("0x100".to_string()),
        };
        let block_number_json = serde_json::to_string(&block_number_rsp).unwrap();
        let mock = server
            .mock("GET", "/api")
            .with_status(200)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("module".into(), "proxy".into()),
                Matcher::UrlEncoded("action".into(), "eth_blockNumber".into()),
                Matcher::UrlEncoded("apikey".into(), key.into()),
            ]))
            .with_body(block_number_json)
            .with_header("content-type", "application/json")
            .create_async()
            .await;
        mocks.push(mock);
    }

    if let Some(_) = include_count {
        let included_count_rsp = ExplorerRpcResponse {
            jsonrpc: "1.0".to_string(),
            id: 1,
            result: serde_json::Value::String("0x200".to_string()),
        };
        let included_count_json = serde_json::to_string(&included_count_rsp).unwrap();
        let mock = server
            .mock("GET", "/api")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("module".into(), "proxy".into()),
                Matcher::UrlEncoded("action".into(), "eth_call".into()),
                Matcher::UrlEncoded("to".into(), contract_address.into()),
                Matcher::UrlEncoded("data".into(), "0xe500f504".into()),
                Matcher::UrlEncoded("tag".into(), "latest".into()),
            ]))
            .with_status(200)
            .with_body(included_count_json)
            .with_header("content-type", "application/json")
            .create_async()
            .await;
        mocks.push(mock);
    }

    if let Some(_) = cms {
        let logs = load_commitment_logs("tests/test_files/data/commitment_logs.json").await;
        let commitment_rsp = ExplorerApiResponse {
            status: "1".to_string(),
            message: "OK".to_string(),
            result: json!(logs),
        };
        let commitment_json = serde_json::to_string(&commitment_rsp).unwrap();
        let mock = server
            .mock("GET", "/api")
            .match_query(Matcher::AnyOf(vec![Matcher::UrlEncoded(
                "action".into(),
                "getLogs".into(),
            )]))
            .with_status(200)
            .with_body(commitment_json)
            .with_header("content-type", "application/json")
            .create_async()
            .await;
        mocks.push(mock);
    }

    (server, mocks)
}

async fn create_explorer_stub(url: &str, key: &str) -> ExplorerStub {
    let _guard = ENV_MUTEX.write().await;
    env::set_var("MYSTIKO_ROLLER_CHAIN_EXPLORER_API_KEY", key);
    let stub = ExplorerStub::new(url, 5).unwrap();
    env::remove_var("MYSTIKO_ROLLER_CHAIN_EXPLORER_API_KEY");
    stub
}
