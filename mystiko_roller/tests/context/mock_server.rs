use mockito::{Matcher, Mock, Server, ServerGuard};
use mystiko_fs::read_file_bytes;
use mystiko_indexer_client::{ApiResponse, CommitmentQueuedResponse, ContractSyncResponse};
use mystiko_server_utils::token_price::query::CurrencyQuoteResponse;

pub async fn create_mock_indexer_server() -> (ServerGuard, u16) {
    let server = Server::new_async().await;
    let binding = server.host_with_port();
    let indexer_port = binding.strip_prefix("127.0.0.1:").unwrap().parse::<u16>().unwrap();
    (server, indexer_port)
}

pub async fn config_mock_indexer_server(
    server: &mut ServerGuard,
    chain_id: u64,
    contract_address: &str,
    block_number: Option<u64>,
    included_count: Option<u32>,
    cms: Option<&[CommitmentQueuedResponse]>,
) -> Vec<Mock> {
    let mut mocks = vec![];

    if let Some(block) = block_number {
        let block_number_path = format!("/chains/{}/contracts/{}/block-number", chain_id, contract_address);
        let block_number_rsp = ApiResponse {
            code: 0,
            result: ContractSyncResponse {
                chain_id: Some(chain_id),
                contract_address: contract_address.to_string(),
                current_sync_block_num: block,
                current_sync_time: None,
            },
        };
        let block_number_json = serde_json::to_string(&block_number_rsp).unwrap();
        let mock = server
            .mock("GET", block_number_path.as_str())
            .with_status(200)
            .with_body(block_number_json)
            .with_header("content-type", "application/json")
            .create_async()
            .await;
        mocks.push(mock);
    }

    if let Some(count) = included_count {
        let included_count_path = format!(
            "/chains/{}/address/{}/count/commitment-included",
            chain_id, contract_address
        );

        let included_count_rsp = ApiResponse { code: 0, result: count };
        let included_count_json = serde_json::to_string(&included_count_rsp).unwrap();
        let path = server
            .mock("GET", included_count_path.as_str())
            .with_status(200)
            .with_body(included_count_json)
            .with_header("content-type", "application/json")
            .create_async()
            .await;
        mocks.push(path);
    }

    if let Some(queued) = cms {
        let queued_commitment_path = format!(
            "/chains/{}/contracts/{}/events/commitment-queued",
            chain_id, contract_address
        );

        let cms_rsp = ApiResponse {
            code: 0,
            result: queued,
        };
        let cms_rsp_json = serde_json::to_string(&cms_rsp).unwrap();
        let path = server
            .mock("POST", queued_commitment_path.as_str())
            .match_query(Matcher::Regex("startBlock".into()))
            .with_status(200)
            .with_body(cms_rsp_json)
            .with_header("content-type", "application/json")
            .create_async()
            .await;
        mocks.push(path);
    }

    mocks
}

pub async fn create_mock_token_price_server() -> (ServerGuard, Mock) {
    let mut server = Server::new_async().await;
    let price_bytes = read_file_bytes("./../mystiko_server_utils/tests/token_price/files/token_price.json")
        .await
        .unwrap();
    let currency_quote: CurrencyQuoteResponse = serde_json::from_slice(&price_bytes).unwrap();
    let resp_json = serde_json::to_string(&currency_quote).unwrap();
    let mock = server
        .mock("GET", "/v2/cryptocurrency/quotes/latest")
        .match_query(Matcher::Regex("id".into()))
        .with_status(200)
        .with_body(resp_json)
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    (server, mock)
}
