use crate::synchronizer::loader::mock::MockSyncDataHandler;
use crate::synchronizer::loader::{build_deposits, create_sync_loader_handler};
use mystiko_dataloader::data::{ChainData, ChainResult, ContractData, FullData};
use mystiko_dataloader::handler::{DataHandler, HandleOption};
use mystiko_protos::core::v1::DepositStatus;
use mystiko_protos::data::v1::{Commitment, CommitmentStatus};
use mystiko_utils::convert::biguint_to_bytes;
use mystiko_utils::hex::decode_hex;
use num_bigint::BigUint;

#[tokio::test]
async fn test_handle_included_commitment_without_update() {
    let mut mock = MockSyncDataHandler::<FullData>::default();
    mock.expect_handle()
        .returning(|_, _| Ok(ChainResult::builder().chain_id(5_u64).build()));
    let (handler, mystiko_db, config) = create_sync_loader_handler(mock).await;
    let option = HandleOption::builder().config(config).build();
    for i in 0..3 {
        let data = build_included_commitment_data(5, "0x4fd0ade06b9654437f46EA59e6edEe056F9d5EF7", i);
        let result = handler.handle(&data, &option).await;
        assert!(result.is_ok());
        let deposits = mystiko_db.deposits.find_all().await.unwrap();
        assert!(deposits.is_empty());
    }
}

#[tokio::test]
async fn test_handle_included_commitment_with_update() {
    let count = 3_usize;
    let mut mock = MockSyncDataHandler::<FullData>::default();
    mock.expect_handle()
        .returning(|_, _| Ok(ChainResult::builder().chain_id(5_u64).build()));
    let (handler, mystiko_db, config) = create_sync_loader_handler(mock).await;
    let all_status: Vec<DepositStatus> = vec![
        DepositStatus::Unspecified,
        DepositStatus::AssetApproving,
        DepositStatus::AssetApproved,
        DepositStatus::SrcPending,
        DepositStatus::SrcSucceeded,
        DepositStatus::Queued,
        DepositStatus::Included,
        DepositStatus::Failed,
    ];

    for status in all_status {
        match status {
            DepositStatus::Unspecified
            | DepositStatus::AssetApproving
            | DepositStatus::AssetApproved
            | DepositStatus::Queued
            | DepositStatus::Failed => {
                mystiko_db.deposits.delete_all().await.unwrap();
                let deposits = build_deposits(status as i32, 5, count);
                let mut db_deposits = mystiko_db.deposits.insert_batch(&deposits).await.unwrap();
                let option = HandleOption::builder().config(config.clone()).build();
                for i in 0..count {
                    let data = build_included_commitment_data(5, "0x4fd0ade06b9654437f46EA59e6edEe056F9d5EF7", i + 1);
                    let result = handler.handle(&data, &option).await;
                    assert!(result.is_ok());
                    let updated_deposits = mystiko_db.deposits.find_all().await.unwrap();
                    db_deposits[i].data.status = DepositStatus::Included as i32;
                    db_deposits[i].data.queued_transaction_hash = Some(String::from("0x3333"));
                    db_deposits[i].data.included_transaction_hash = Some(String::from("0x4444"));
                    for j in 0..count {
                        assert_eq!(updated_deposits[j].data, db_deposits[j].data);
                    }
                }
            }
            DepositStatus::Included => {
                mystiko_db.deposits.delete_all().await.unwrap();
                let deposits = build_deposits(status as i32, 5, count);
                let db_deposits = mystiko_db.deposits.insert_batch(&deposits).await.unwrap();
                let option = HandleOption::builder().config(config.clone()).build();
                for i in 0..count {
                    let data = build_included_commitment_data(5, "0x4fd0ade06b9654437f46EA59e6edEe056F9d5EF7", i + 1);
                    let result = handler.handle(&data, &option).await;
                    assert!(result.is_ok());
                    let updated_deposits = mystiko_db.deposits.find_all().await.unwrap();
                    for j in 0..count {
                        assert_eq!(updated_deposits[j].data, db_deposits[j].data);
                    }
                }
            }
            DepositStatus::SrcPending => {}
            DepositStatus::SrcSucceeded => {}
        }
    }
}

fn build_included_commitment_data(chain_id: u64, address: &str, count: usize) -> ChainData<FullData> {
    let mut commitments = vec![];
    for i in 0..count {
        let commitment = Commitment {
            commitment_hash: biguint_to_bytes(&BigUint::from(1000 + i as u32)),
            status: CommitmentStatus::Included as i32,
            block_number: 2000 + i as u64,
            included_block_number: None,
            src_chain_block_number: None,
            leaf_index: Some(i as u64),
            rollup_fee: None,
            encrypted_note: None,
            queued_transaction_hash: Some(decode_hex("0x3333").unwrap()),
            included_transaction_hash: Some(decode_hex("0x4444").unwrap()),
            src_chain_transaction_hash: None,
        };
        commitments.push(commitment);
    }
    let contract_data = ContractData::builder()
        .address(address.to_string())
        .start_block(1_u64)
        .end_block(100_u64)
        .data(FullData::builder().commitments(commitments).build())
        .build();
    ChainData::<FullData>::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build()
}
