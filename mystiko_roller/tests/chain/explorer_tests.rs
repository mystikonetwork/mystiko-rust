use crate::common::ENV_MUTEX;
use mystiko_roller::chain::explorer::ExplorerStub;
use mystiko_roller::chain::ChainDataGiver;
use mystiko_roller::config::roller::ChainDataSource;
use mystiko_roller::db::document::commitment::CommitmentInfo;
use num_bigint::BigInt;
use std::env;
use std::str::FromStr;

#[tokio::test]
pub async fn test_explorer_stub() {
    let stub = {
        let _guard = ENV_MUTEX.write().await;
        env::set_var("MYSTIKO_ROLLER_CHAIN_EXPLORER_API_KEY", "");
        let stub = ExplorerStub::new("https://api.polygonscan.com", 5);
        env::remove_var("MYSTIKO_ROLLER_CHAIN_EXPLORER_API_KEY");
        stub
    };

    assert!(stub.is_ok());
    let stub = stub.unwrap();
    let data_source = stub.data_source();
    assert_eq!(data_source, ChainDataSource::Explorer);
    let result = stub
        .get_latest_block_number(137, "0x95Dfa68De44eCe33F64C4Ac8e5D569B2C5e90A51")
        .await;
    assert!(result.unwrap() >= 44789729);
    let result = stub
        .get_included_count(137, "0x95Dfa68De44eCe33F64C4Ac8e5D569B2C5e90A51")
        .await;
    assert!(result.unwrap() >= 13223);
    let result = stub
        .get_queued_commitments(137, "0x95Dfa68De44eCe33F64C4Ac8e5D569B2C5e90A51", 44899568, 44899569)
        .await;
    assert_eq!(
        result.unwrap(),
        vec![CommitmentInfo {
            chain_id: 137,
            contract_address: "0x95Dfa68De44eCe33F64C4Ac8e5D569B2C5e90A51".to_string(),
            commitment_hash: BigInt::from_str(
                "8856492658588389416279023856973501927042415036331927671035138352783086871533"
            )
            .unwrap(),
            block_number: 44899568,
            rollup_fee: "100000000000000000".to_string(),
            leaf_index: 13235,
            tx_hash: "0xc0be8719bc039f67e57792e8b0735262eaeb9fd387b7c1683d6f68822bc86087".to_string(),
        }]
    );
}
