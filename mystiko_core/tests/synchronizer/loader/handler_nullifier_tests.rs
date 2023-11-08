use crate::synchronizer::loader::create_sync_loader_handler;
use crate::synchronizer::loader::mock::MockSyncDataHandler;
use mystiko_core::Commitment;
use mystiko_dataloader::data::{ChainData, ChainResult, ContractData, FullData};
use mystiko_dataloader::handler::{DataHandler, HandleOption};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::data::v1::{CommitmentStatus, Nullifier};
use mystiko_utils::convert::biguint_to_bytes;
use mystiko_utils::hex::decode_hex;
use num_bigint::BigUint;
use std::ops::Add;
use std::str::FromStr;

#[tokio::test]
async fn test_handle_nullifier_without_update() {
    let mut mock = MockSyncDataHandler::<FullData>::default();
    mock.expect_handle()
        .returning(|_, _| Ok(ChainResult::builder().chain_id(5_u64).build()));
    let (handler, mystiko_db, config) = create_sync_loader_handler(mock).await;
    let option = HandleOption::builder().config(config).build();
    for i in 0..5 {
        let data = build_nullifier_data(5, "0x4fd0ade06b9654437f46EA59e6edEe056F9d5EF7", i);
        let result = handler.handle(&data, &option).await;
        assert!(result.is_ok());
        let commitments = mystiko_db.commitments.find_all().await.unwrap();
        assert!(commitments.is_empty());
    }
}

#[tokio::test]
async fn test_handle_nullifier_with_update() {
    let mut mock = MockSyncDataHandler::<FullData>::default();
    mock.expect_handle()
        .returning(|_, _| Ok(ChainResult::builder().chain_id(5_u64).build()));
    let count = 5;
    let (handler, mystiko_db, config) = create_sync_loader_handler(mock).await;
    let option = HandleOption::builder().config(config).build();
    let data = build_nullifier_data(5, "0x4fd0ade06b9654437f46EA59e6edEe056F9d5EF7", count);

    // no match commitments
    let cms = build_commitments().await;
    let mut db_cms = mystiko_db.commitments.insert_batch(&cms).await.unwrap();
    let result = handler.handle(&data, &option).await;
    assert!(result.is_ok());

    // match one commitment
    for i in 0..count {
        db_cms[i].data.nullifier = Some(BigUint::from((1000 + i) as u32));
        mystiko_db.commitments.update_batch(&db_cms).await.unwrap();
        let result = handler.handle(&data, &option).await;
        assert!(result.is_ok());
        let query_cms = mystiko_db.commitments.find_all().await.unwrap();
        db_cms[i].data.spent = true;
        for j in 0..count {
            assert_eq!(db_cms[j].data, query_cms[j].data);
        }
    }
}

fn build_nullifier_data(chain_id: u64, address: &str, count: usize) -> ChainData<FullData> {
    let mut nullifiers = vec![];
    for i in 0..count {
        let nullifier = Nullifier {
            nullifier: biguint_to_bytes(&BigUint::from((i + 1000) as u32)),
            block_number: 2500_u64,
            transaction_hash: decode_hex((i + 1000).to_string()).unwrap(),
        };
        nullifiers.push(nullifier);
    }
    let contract_data = ContractData::builder()
        .address(address.to_string())
        .start_block(1_u64)
        .end_block(100_u64)
        .data(FullData::builder().nullifiers(nullifiers).build())
        .build();
    ChainData::<FullData>::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build()
}

async fn build_commitments() -> Vec<Commitment> {
    let cm = Commitment {
        chain_id: 5,
        contract_address: String::from("0x4fd0ade06b9654437f46EA59e6edEe056F9d5EF7"),
        bridge_type: BridgeType::Loop as i32,
        commitment_hash: BigUint::from_str(
            "9709495941671889428395361755215352896616366060066411186055604144562505250548",
        )
        .unwrap(),
        asset_symbol: String::from("MTT"),
        asset_decimals: 18,
        asset_address: Some(String::from("0x80C46C896E26C1cB7DCdD23019d9e7cca6854864")),
        status: CommitmentStatus::Unspecified as i32,
        spent: false,
        block_number: 10000000u64,
        src_chain_block_number: Some(10000000u64),
        included_block_number: Some(10000100u64),
        rollup_fee_amount: Some(BigUint::from(20000000000000000u64)),
        encrypted_note: Some(String::from(
            "9f86d4d7e35fb1f2a24f9784d4fced7f045cdf768b82d33e17ed1b62cb0\
                    a9706d13ff263c74d746df89a09cfa57405547db8cf3c4300e693d3cbd117f5f0c6e7af10c022c\
                    2a0110fa91afbd8ac01a55ad28e7b2ec01a3268a980e2a8c3f349f19e8d26cc8131bbe3f68c418\
                    f7cb6ba2bcdcdd86a5b2370792b2a86330096104637f0b7b992436f1a83000a727476b006b05da\
                    69f5eb0812f57ad6d871e53dd2f73c6b8ede35effdb39c1f2e78357a96da6f7af8d57d9ae524df\
                    2cd001ec1e6cbaa2cf5cb90ded104783af9b5c144c9513e",
        )),
        leaf_index: Some(0u64),
        amount: Some(BigUint::from(1000000000000000000u64)),
        nullifier: None,
        shielded_address: Some(String::from("0x8695520Db7C1074D07898D655D2Bc7308395B041b")),
        queued_transaction_hash: Some(String::from("")),
        included_transaction_hash: Some(String::from("")),
        src_chain_transaction_hash: Some(String::from("")),
    };

    let mut cms = vec![];
    for i in 0..5 {
        let mut c = cm.clone();
        c.commitment_hash = c.commitment_hash.add(&BigUint::from(i as u32));
        c.leaf_index = Some(i as u64);
        cms.push(c);
    }
    cms
}
