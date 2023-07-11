use crate::common::ENV_MUTEX;
use crate::context::mock_context::explorer_server_port;
use crate::test_files::load::load_commitment_logs;
use httptest::{matchers::*, responders::*, Expectation, Server, ServerBuilder};
use mystiko_roller::chain::explorer::{ExplorerApiResponse, ExplorerRpcResponse, ExplorerStub};
use mystiko_roller::chain::ChainDataGiver;
use mystiko_roller::common::error::RollerError;
use mystiko_roller::config::roller::ChainDataSource;
use serde_json::json;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::test]
pub async fn test_explorer_stub() {
    let stub = create_explorer_stub("http://error", "test_explorer").await;
    assert_eq!(stub.data_source(), ChainDataSource::Explorer);
    let block_number = stub.get_latest_block_number(1, "").await;
    assert!(matches!(block_number.err().unwrap(), RollerError::ReqwestError(_)));

    let server = create_explorer_server(1, "0x7f88f2a3cf18e96844e14cae59ec97b908734c01", "test_explorer").await;
    let stub = create_explorer_stub(&server.url("").to_string(), "test_explorer").await;
    let block_number = stub.get_latest_block_number(1, "").await;
    assert_eq!(block_number.unwrap(), 256);
    let block_number = stub
        .get_included_count(1, "0x7f88f2a3cf18e96844e14cae59ec97b908734c01")
        .await;
    assert_eq!(block_number.unwrap(), 512);
    let cms = stub
        .get_queued_commitments(1, "0x7f88f2a3cf18e96844e14cae59ec97b908734c01", 1, 100)
        .await;
    assert_eq!(cms.unwrap().len(), 35);
}

#[tokio::test]
pub async fn test_explorer_rpc_error_response() {
    let server = create_explorer_rpc_error_server(2).await;
    let stub = create_explorer_stub(&server.url("").to_string(), "test_explorer").await;
    let result = stub.get_included_count(2, "error_address1").await;
    assert!(matches!(result.err().unwrap(), RollerError::ReqwestError(_)));
    let result = stub.get_included_count(2, "error_address2").await;
    assert!(matches!(result.err().unwrap(), RollerError::StubExplorerError(_, _)));
    let result = stub.get_included_count(2, "error_address3").await;
    assert!(matches!(result.err().unwrap(), RollerError::StubExplorerError(_, _)));
    let result = stub.get_included_count(2, "error_address4").await;
    assert!(matches!(result.err().unwrap(), RollerError::StubExplorerError(_, _)));
}

#[tokio::test]
pub async fn test_explorer_api_error_response() {
    let server = create_explorer_api_error_server(3).await;
    let stub = create_explorer_stub(&server.url("").to_string(), "test_explorer").await;
    let result = stub.get_queued_commitments(3, "", 100, 1000).await;
    assert!(matches!(result.err().unwrap(), RollerError::StubExplorerError(_, _)));
    let result = stub.get_queued_commitments(3, "", 200, 1000).await;
    assert!(matches!(result.err().unwrap(), RollerError::StubExplorerError(_, _)));
    let result = stub.get_queued_commitments(3, "", 300, 1000).await;
    assert!(matches!(result.err().unwrap(), RollerError::SerdeJsonError(_)));
}

async fn create_explorer_server(chain_id: u64, contract_address: &str, key: &str) -> Server {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), explorer_server_port(chain_id));
    let server = ServerBuilder::new().bind_addr(addr).run().unwrap();

    let block_number_query = format!("module=proxy&action=eth_blockNumber&apikey={}", key);
    let block_number_rsp = ExplorerRpcResponse {
        jsonrpc: "1.0".to_string(),
        id: 1,
        result: serde_json::Value::String("0x100".to_string()),
    };
    let block_number_json = json_encoded(json!(block_number_rsp));
    server.expect(Expectation::matching(request::query(matches(block_number_query))).respond_with(block_number_json));

    let included_count_query = format!(
        "module=proxy&action=eth_call&to={}&data=0xe500f504&tag=latest",
        contract_address
    );
    let included_count_rsp = ExplorerRpcResponse {
        jsonrpc: "1.0".to_string(),
        id: 1,
        result: serde_json::Value::String("0x200".to_string()),
    };
    let included_count_json = json_encoded(json!(included_count_rsp));
    server
        .expect(Expectation::matching(request::query(matches(included_count_query))).respond_with(included_count_json));

    let commitment_query = "module=logs&action=getLogs&";
    let logs = load_commitment_logs("tests/test_files/data/commitment_logs.json").await;
    let commitment_rsp = ExplorerApiResponse {
        status: "1".to_string(),
        message: "OK".to_string(),
        result: json!(logs),
    };
    let commitment_json = json_encoded(json!(commitment_rsp));
    server.expect(Expectation::matching(request::query(matches(commitment_query))).respond_with(commitment_json));
    server
}

async fn create_explorer_rpc_error_server(chain_id: u64) -> Server {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), explorer_server_port(chain_id));
    let server = ServerBuilder::new().bind_addr(addr).run().unwrap();

    let error_address1_query = format!(
        "module=proxy&action=eth_call&to={}&data=0xe500f504&tag=latest",
        "error_address1"
    );
    server.expect(Expectation::matching(request::query(matches(error_address1_query))).respond_with(status_code(500)));

    let error_address1_query = format!(
        "module=proxy&action=eth_call&to={}&data=0xe500f504&tag=latest",
        "error_address2"
    );
    server.expect(
        Expectation::matching(request::query(matches(error_address1_query)))
            .respond_with(status_code(200).append_header("X-Custom-Foo", "Bar").body("")),
    );

    let error_address1_query = format!(
        "module=proxy&action=eth_call&to={}&data=0xe500f504&tag=latest",
        "error_address3"
    );
    server.expect(
        Expectation::matching(request::query(matches(error_address1_query))).respond_with(
            status_code(200)
                .append_header("content-type", "application/error")
                .body(""),
        ),
    );

    let error_address2_query = format!(
        "module=proxy&action=eth_call&to={}&data=0xe500f504&tag=latest",
        "error_address4"
    );
    let included_count_rsp = ExplorerRpcResponse {
        jsonrpc: "1.0".to_string(),
        id: 1,
        result: serde_json::Value::String("error_address4".to_string()),
    };
    let included_count_json = json_encoded(json!(included_count_rsp));
    server
        .expect(Expectation::matching(request::query(matches(error_address2_query))).respond_with(included_count_json));

    server
}

async fn create_explorer_api_error_server(chain_id: u64) -> Server {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), explorer_server_port(chain_id));
    let server = ServerBuilder::new().bind_addr(addr).run().unwrap();

    let commitment_query = "module=logs&action=getLogs&fromBlock=100";
    let commitment_rsp = ExplorerApiResponse {
        status: "0".to_string(),
        message: "OK".to_string(),
        result: json!(vec!["error1".to_string()]),
    };
    let commitment_json = json_encoded(json!(commitment_rsp));
    server.expect(Expectation::matching(request::query(matches(commitment_query))).respond_with(commitment_json));

    let commitment_query = "module=logs&action=getLogs&fromBlock=200";
    let commitment_rsp = ExplorerApiResponse {
        status: "1".to_string(),
        message: "Error".to_string(),
        result: json!(vec!["error2".to_string()]),
    };
    let commitment_json = json_encoded(json!(commitment_rsp));
    server.expect(Expectation::matching(request::query(matches(commitment_query))).respond_with(commitment_json));

    let commitment_query = "module=logs&action=getLogs&fromBlock=300";
    let commitment_rsp = ExplorerApiResponse {
        status: "1".to_string(),
        message: "OK".to_string(),
        result: json!(vec!["error3".to_string()]),
    };
    let commitment_json = json_encoded(json!(commitment_rsp));
    server.expect(Expectation::matching(request::query(matches(commitment_query))).respond_with(commitment_json));

    server
}

async fn create_explorer_stub(url: &str, key: &str) -> ExplorerStub {
    let _guard = ENV_MUTEX.write().await;
    env::set_var("MYSTIKO_ROLLER_CHAIN_EXPLORER_API_KEY", key);
    let stub = ExplorerStub::new(url, 5).unwrap();
    env::remove_var("MYSTIKO_ROLLER_CHAIN_EXPLORER_API_KEY");
    stub
}
