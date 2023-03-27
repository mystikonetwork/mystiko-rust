use base64::{engine::general_purpose, Engine as _};

use mockito::*;
use mystiko_indexer_client::errors::ClientError;
use mystiko_indexer_client::response::ApiResponse;
use mystiko_indexer_client::{
    client::IndexerClient,
    types::{
        commitment_included::{
            CommitmentIncludedFilter, CommitmentIncludedForChainRequest, CommitmentIncludedResponse,
        },
        commitment_queued::{
            CommitmentQueuedFilter, CommitmentQueuedForChainRequest, CommitmentQueuedResponse,
        },
        commitment_spent::{
            CommitmentSpentFilter, CommitmentSpentForChainRequest, CommitmentSpentResponse,
        },
        sync_response::{ChainSyncRepsonse, ContractSyncResponse},
    },
};

static AUTH_USERNAME: &str = "test_username";
static AUTH_PASSWORD: &str = "110110";

struct TestClientSetupData {
    mocked_server: mockito::ServerGuard,
    indexer_client: IndexerClient,
}

fn create_indexer_client(base_url: &str) -> IndexerClient {
    IndexerClient::builder(base_url).build().unwrap()
}

fn create_indexer_client_with_auth(base_url: &str) -> IndexerClient {
    IndexerClient::builder(base_url)
        .auth_username(Some(String::from(AUTH_USERNAME)))
        .auth_password(Some(String::from(AUTH_PASSWORD)))
        .build()
        .unwrap()
}

async fn setup() -> Result<TestClientSetupData, Error> {
    let mocked_server = Server::new_async().await;
    let mocked_server_url = mocked_server.url();
    let indexer_client = create_indexer_client(&mocked_server_url);
    Ok(TestClientSetupData {
        mocked_server,
        indexer_client,
    })
}

async fn setup_with_auth() -> Result<TestClientSetupData, Error> {
    let mocked_server = Server::new_async().await;
    let mocked_server_url = mocked_server.url();
    let indexer_client = create_indexer_client_with_auth(&mocked_server_url);
    Ok(TestClientSetupData {
        mocked_server,
        indexer_client,
    })
}

fn generate_auth_header() -> String {
    let str = AUTH_USERNAME.to_owned() + ":" + AUTH_PASSWORD;
    let encoded = general_purpose::STANDARD.encode(str);
    "Basic ".to_owned() + &encoded
}

#[test]
fn test_create_indexer_client() {
    let base_url = "http://test_url:3098";
    let client = create_indexer_client(base_url);
    assert_eq!(client.base_url, base_url);
}

#[tokio::test]
async fn test_ping() {
    let TestClientSetupData {
        mut mocked_server,
        indexer_client,
    } = setup().await.unwrap();
    let message = String::from("hello");
    let mocked_api_resp = ApiResponse {
        code: 0,
        result: message.clone(),
    };
    let m = mocked_server
        .mock("get", "/ping?message=hello")
        .with_status(200)
        .with_body(serde_json::to_string(&mocked_api_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let resp = indexer_client.ping(&message).await.unwrap();
    assert_eq!(resp, message);
    m.assert_async().await;
}

#[tokio::test]
async fn test_auth_ping() {
    let TestClientSetupData {
        mut mocked_server,
        indexer_client,
    } = setup_with_auth().await.unwrap();
    let message = String::from("helloauth");
    let mocked_api_resp = ApiResponse {
        code: 0,
        result: message.clone(),
    };
    let m = mocked_server
        .mock("get", "/auth-ping?message=helloauth")
        .match_header("Authorization", Matcher::Exact(generate_auth_header()))
        .with_status(200)
        .with_body(serde_json::to_string(&mocked_api_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let resp = indexer_client.auth_ping(&message).await.unwrap();
    assert_eq!(resp, message);
    m.assert_async().await;
    let mocked_error_api_resp = ApiResponse {
        code: -1,
        result: message.clone(),
    };
    let m = mocked_server
        .mock("get", "/auth-ping?message=helloauth")
        .match_header("Authorization", Matcher::Exact(generate_auth_header()))
        .with_status(200)
        .with_body(serde_json::to_string(&mocked_error_api_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let resp2 = indexer_client.auth_ping(&message).await;
    assert!(resp2.is_err());
    let error = resp2.unwrap_err();
    assert!(error.downcast_ref::<ClientError>().is_some());
    assert_eq!(
        error.to_string(),
        ClientError::ApiResponseError {
            code: -1,
            message: String::from("\"helloauth\""),
        }
        .to_string()
    );
    m.assert_async().await;
    let m = mocked_server
        .mock("get", "/auth-ping?message=helloauth")
        .match_header("Authorization", Matcher::Exact(generate_auth_header()))
        .with_status(200)
        .with_body(serde_json::to_string(&mocked_error_api_resp).unwrap())
        .create_async()
        .await;
    let resp3 = indexer_client.auth_ping(&message).await;
    assert!(resp3.is_err());
    assert_eq!(
        resp3.unwrap_err().to_string(),
        ClientError::UnsupportedContentTypeError(String::from("")).to_string()
    );
    m.assert_async().await;
}

#[tokio::test]
async fn test_find_commitment_queued_for_chain() {
    let TestClientSetupData {
        mut mocked_server,
        indexer_client,
    } = setup().await.unwrap();
    let chain_id = 5;
    let start_block = 100;
    let end_block = 10000;
    let where_filter = CommitmentQueuedFilter::builder()
        .commit_hash("commit_hash 1".to_string())
        .build();
    let json_str = serde_json::to_string(&where_filter).unwrap();
    let where_filter2 = serde_json::from_str::<Option<CommitmentQueuedFilter>>(&json_str).unwrap();
    assert!(where_filter2.is_some());
    let resp_list: Vec<CommitmentQueuedResponse> = vec![
        CommitmentQueuedResponse::builder()
            .id(1)
            .chain_id(5)
            .block_num(100)
            .commit_hash("commit hash 1".to_string())
            .contract_address("contract_address 1".to_string())
            .contract_id(1)
            .create_at(Some(100))
            .encrypted_note("encrypted_note 1".to_string())
            .leaf_index(1)
            .rollup_fee("rollup_fee 1".to_string())
            .status(Some(1))
            .tx_hash("tx_hash 1".to_string())
            .build(),
        CommitmentQueuedResponse::builder()
            .id(2)
            .chain_id(25)
            .block_num(200)
            .commit_hash("commit hash 2".to_string())
            .contract_address("contract_address 2".to_string())
            .contract_id(2)
            .create_at(Some(200))
            .encrypted_note("encrypted_note 2".to_string())
            .leaf_index(2)
            .rollup_fee("rollup_fee 2".to_string())
            .status(Some(2))
            .tx_hash("tx_hash 2".to_string())
            .build(),
        CommitmentQueuedResponse::builder()
            .id(3)
            .chain_id(25)
            .block_num(300)
            .commit_hash("commit hash 3".to_string())
            .contract_address("contract_address 3".to_string())
            .contract_id(2)
            .create_at(Some(300))
            .encrypted_note("encrypted_note 3".to_string())
            .leaf_index(3)
            .rollup_fee("rollup_fee 3".to_string())
            .status(None)
            .tx_hash("tx_hash 3".to_string())
            .build(),
    ];
    let mocked_api_resp = ApiResponse {
        code: 0,
        result: &resp_list,
    };
    let m = mocked_server
        .mock("post", "/chains/5/events/commitment-queued")
        .with_status(200)
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("startBlock".into(), "100".into()),
            Matcher::UrlEncoded("endBlock".into(), "10000".into()),
        ]))
        .with_body(serde_json::to_string(&mocked_api_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let where_filter = CommitmentQueuedFilter::builder().build();
    let resp = indexer_client
        .find_commitment_queued_for_chain(
            &CommitmentQueuedForChainRequest::builder()
                .chain_id(chain_id)
                .start_block(start_block)
                .end_block(end_block)
                .where_filter(where_filter)
                .build(),
        )
        .await;
    assert!(resp.is_ok());
    assert_eq!(resp.unwrap(), resp_list);
    m.assert_async().await;
    let mocked_api_resp = ApiResponse {
        code: -1,
        result: "unknow error",
    };
    let m = mocked_server
        .mock("post", "/chains/5/events/commitment-queued")
        .with_status(200)
        .with_body(serde_json::to_string(&mocked_api_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let resp = indexer_client
        .find_commitment_queued_for_chain(
            &CommitmentQueuedForChainRequest::builder()
                .chain_id(chain_id)
                .build(),
        )
        .await;
    assert!(resp.is_err());
    m.assert_async().await;
}

#[tokio::test]
async fn test_find_commitment_included_for_chain() {
    let TestClientSetupData {
        mut mocked_server,
        indexer_client,
    } = setup().await.unwrap();

    let resp_list: Vec<CommitmentIncludedResponse> = vec![
        CommitmentIncludedResponse::builder()
            .id(1)
            .chain_id(97)
            .contract_address(String::from("0x159f94bda2E983aF1Eb0f49e8EdbA8D15b426974"))
            .block_num(21715843)
            .commit_hash(String::from(
                "21660236736937057929207091181615668803485477896288065933382978873519224549708",
            ))
            .tx_hash(String::from(
                "0x4aba2b761b412e94527c23d7f83e3f580c706f26c691ed9bf27d34fcac79180a",
            ))
            .create_at(1662958509)
            .status(1)
            .contract_id(1)
            .build(),
        CommitmentIncludedResponse::builder()
            .id(2)
            .chain_id(5)
            .contract_address(String::from("0x4c77cc65b9011D7355Cae4e68978a613197Ae13c"))
            .commit_hash(String::from(
                "20153626271459114557660212467000180564260730543508573984596461179335136649654",
            ))
            .tx_hash(String::from(
                "0x2485ab03fbc089453c71d093aa7bc5db194f78d465e2288478b16864ef07b7cd",
            ))
            .block_num(7580062)
            .create_at(1662958509)
            .status(1)
            .contract_id(2)
            .build(),
        CommitmentIncludedResponse::builder()
            .id(3)
            .chain_id(5)
            .contract_address(String::from("0xDDcF4fB58633a903b890f2C5ec11605063D8bFE1"))
            .commit_hash(String::from(
                "6526379515466338789429850953742086390406303560083519882995145633930466515760",
            ))
            .tx_hash(String::from(
                "0xde4de84f34929d496df7b9d36c012ce18517767e663c29fc0647b07c0c656d6e",
            ))
            .block_num(7580299)
            .create_at(1662958509)
            .status(1)
            .contract_id(2)
            .build(),
    ];

    let chain_id = 97;
    let start_block = 0;
    let end_block = 21716247;
    // test find without filter
    let mocked_api_resp = ApiResponse {
        code: 0,
        result: &resp_list,
    };
    let m = mocked_server
        .mock("post", "/chains/97/events/commitment-included")
        .with_status(200)
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("startBlock".into(), "0".into()),
            Matcher::UrlEncoded("endBlock".into(), "21716247".into()),
        ]))
        .with_body(serde_json::to_string(&mocked_api_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let where_filter = CommitmentIncludedFilter::builder().build();
    let resp = indexer_client
        .find_commitment_included_for_chain(
            &CommitmentIncludedForChainRequest::builder()
                .chain_id(chain_id)
                .start_block(start_block)
                .end_block(end_block)
                .where_filter(where_filter)
                .build(),
        )
        .await;
    assert!(resp.is_ok());
    assert_eq!(resp.unwrap(), resp_list);
    m.assert_async().await;

    //test find with filter
    let where_filter = CommitmentIncludedFilter::builder()
        .contract_address(String::from("0x4c77cc65b9011D7355Cae4e68978a613197Ae13c"))
        .build();
    let mocked_api_resp = ApiResponse {
        code: 0,
        result: vec![&resp_list[1]],
    };
    let m = mocked_server
        .mock("post", "/chains/5/events/commitment-included")
        .with_status(200)
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("startBlock".into(), "0".into()),
            Matcher::UrlEncoded("endBlock".into(), "21716247".into()),
        ]))
        .match_body(Matcher::JsonString(
            serde_json::to_string(&where_filter).unwrap(),
        ))
        .with_body(serde_json::to_string(&mocked_api_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let resp = indexer_client
        .find_commitment_included_for_chain(
            &CommitmentIncludedForChainRequest::builder()
                .chain_id(5)
                .start_block(start_block)
                .end_block(end_block)
                .where_filter(where_filter)
                .build(),
        )
        .await;
    assert!(resp.is_ok());
    assert_eq!(resp.as_ref().unwrap().len(), 1);
    assert_eq!(resp.unwrap()[0], resp_list[1]);
    m.assert_async().await;

    //test response error
    let mocked_api_resp = ApiResponse {
        code: -1,
        result: "unknow error",
    };
    let m = mocked_server
        .mock("post", "/chains/97/events/commitment-included")
        .with_status(200)
        .with_body(serde_json::to_string(&mocked_api_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let resp = indexer_client
        .find_commitment_included_for_chain(
            &CommitmentIncludedForChainRequest::builder()
                .chain_id(chain_id)
                .build(),
        )
        .await;
    assert!(resp.is_err());
    m.assert_async().await;
}

#[tokio::test]
async fn test_find_commitment_spent_for_chain() {
    let TestClientSetupData {
        mut mocked_server,
        indexer_client,
    } = setup().await.unwrap();

    let resp_list: Vec<CommitmentSpentResponse> = vec![
        CommitmentSpentResponse::builder()
            .id(1)
            .chain_id(97)
            .contract_address(String::from("0x159f94bda2E983aF1Eb0f49e8EdbA8D15b426974"))
            .root_hash(String::from(
                "573726375045931455101433550207148071210294757245834303948442818863142132452",
            ))
            .tx_hash(String::from(
                "0x4aba2b761b412e94527c23d7f83e3f580c706f26c691ed9bf27d34fcac79180a",
            ))
            .serial_num(String::from(
                "18289140417703032180442871124681619645313062156272988637933974003938311195138",
            ))
            .block_num(21715843)
            .create_at(1663058707)
            .update_at(1663058707)
            .status(1)
            .contract_id(1)
            .build(),
        CommitmentSpentResponse::builder()
            .id(2)
            .chain_id(55)
            .contract_address(String::from("0x4c77cc65b9011D7355Cae4e68978a613197Ae13c"))
            .root_hash(String::from(
                "16298153976521878355251099828571002132091939055949793118029983320576459321465",
            ))
            .tx_hash(String::from(
                "0x2485ab03fbc089453c71d093aa7bc5db194f78d465e2288478b16864ef07b7cd",
            ))
            .serial_num(String::from(
                "13957096243013284125113138367134647676297156555685243681153248870297906895235",
            ))
            .block_num(7590302)
            .create_at(1663058707)
            .update_at(1663058707)
            .status(1)
            .contract_id(2)
            .build(),
        CommitmentSpentResponse::builder()
            .id(3)
            .chain_id(5)
            .contract_address(String::from("0xDDcF4fB58633a903b890f2C5ec11605063D8bFE1"))
            .root_hash(String::from(
                "11182207403285972011418180528458663983668852943544431829230655838393827577659",
            ))
            .tx_hash(String::from(
                "0xde4de84f34929d496df7b9d36c012ce18517767e663c29fc0647b07c0c656d6e",
            ))
            .serial_num(String::from(
                "9513111546062163229603770214785689228965480378383041820757927697515308126521",
            ))
            .block_num(7590302)
            .create_at(1663058707)
            .update_at(1663058707)
            .status(1)
            .contract_id(2)
            .build(),
    ];

    let chain_id = 97;
    let start_block = 0;
    let end_block = 21716247;
    // test find without filter
    let mocked_api_resp = ApiResponse {
        code: 0,
        result: &resp_list,
    };
    let m = mocked_server
        .mock("post", "/chains/97/events/commitment-spent")
        .with_status(200)
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("startBlock".into(), "0".into()),
            Matcher::UrlEncoded("endBlock".into(), "21716247".into()),
        ]))
        .with_body(serde_json::to_string(&mocked_api_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let where_filter = CommitmentSpentFilter::builder().build();
    let resp = indexer_client
        .find_commitment_spent_for_chain(
            &CommitmentSpentForChainRequest::builder()
                .chain_id(chain_id)
                .start_block(start_block)
                .end_block(end_block)
                .where_filter(where_filter)
                .build(),
        )
        .await;
    assert!(resp.is_ok());
    assert_eq!(resp.unwrap(), resp_list);
    m.assert_async().await;

    //test find with filter
    let where_filter = CommitmentSpentFilter::builder()
        .contract_address(String::from("0xDDcF4fB58633a903b890f2C5ec11605063D8bFE1"))
        .build();
    let mocked_api_resp = ApiResponse {
        code: 0,
        result: vec![&resp_list[2]],
    };
    let m = mocked_server
        .mock("post", "/chains/5/events/commitment-spent")
        .with_status(200)
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("startBlock".into(), "0".into()),
            Matcher::UrlEncoded("endBlock".into(), "21716247".into()),
        ]))
        .match_body(Matcher::JsonString(
            serde_json::to_string(&where_filter).unwrap(),
        ))
        .with_body(serde_json::to_string(&mocked_api_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let resp = indexer_client
        .find_commitment_spent_for_chain(
            &CommitmentSpentForChainRequest::builder()
                .chain_id(5)
                .start_block(start_block)
                .end_block(end_block)
                .where_filter(where_filter)
                .build(),
        )
        .await;
    assert!(resp.is_ok());
    assert_eq!(resp.as_ref().unwrap().len(), 1);
    assert_eq!(resp.unwrap()[0], resp_list[2]);
    m.assert_async().await;

    //test response error
    let mocked_api_resp = ApiResponse {
        code: -1,
        result: "unknow error",
    };
    let m = mocked_server
        .mock("post", "/chains/97/events/commitment-spent")
        .with_status(200)
        .with_body(serde_json::to_string(&mocked_api_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let resp = indexer_client
        .find_commitment_spent_for_chain(
            &CommitmentSpentForChainRequest::builder()
                .chain_id(chain_id)
                .build(),
        )
        .await;
    assert!(resp.is_err());
    m.assert_async().await;
}

#[tokio::test]
async fn test_query_chain_sync_response_by_id() {
    let TestClientSetupData {
        mut mocked_server,
        indexer_client,
    } = setup().await.unwrap();
    let test_chain_id = 5;
    let contracts = vec![
        ContractSyncResponse::builder()
            .chain_id(test_chain_id)
            .contract_address(String::from("address1"))
            .current_sync_block_num(100)
            .current_sync_time(10000000)
            .build(),
        ContractSyncResponse::builder()
            .chain_id(test_chain_id)
            .contract_address(String::from("address1"))
            .current_sync_block_num(200)
            .current_sync_time(200000000000)
            .build(),
    ];
    let chain_sync_resp = ChainSyncRepsonse::builder()
        .chain_id(test_chain_id)
        .current_sync_block_num(10000000)
        .contracts(contracts)
        .build();
    let mocked_api_resp = ApiResponse {
        code: 0,
        result: &chain_sync_resp,
    };
    let m = mocked_server
        .mock("get", "/chains/5/block-number")
        .with_status(200)
        .with_body(serde_json::to_string(&mocked_api_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let resp = indexer_client.query_chain_sync_repsonse_by_id(5).await;
    assert!(resp.is_ok());
    assert_eq!(resp.unwrap(), chain_sync_resp);
    m.assert_async().await;
}
