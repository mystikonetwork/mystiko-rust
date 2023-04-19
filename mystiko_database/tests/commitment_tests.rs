extern crate mystiko_database;
extern crate num_bigint;

use mystiko_database::collection::commitment::CommitmentCollection;
use mystiko_database::document::commitment::Commitment;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::filter::{Condition, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::{SqliteRawData, SqliteStorage, SqliteStorageBuilder};
use mystiko_types::CommitmentStatus;
use num_bigint::BigInt;
use std::sync::Arc;

async fn create_commitments() -> CommitmentCollection<SqlFormatter, SqliteRawData, SqliteStorage> {
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let commitments =
        CommitmentCollection::new(Arc::new(Collection::new(SqlFormatter {}, storage)));
    commitments.migrate().await.unwrap();
    assert!(commitments.collection_exists().await.unwrap());
    commitments
}

#[tokio::test]
async fn test_commitments_crud() {
    let commitments = create_commitments().await;

    // testing insert
    let mut inserted_commitments: Vec<Document<Commitment>> = Vec::new();
    inserted_commitments.push(
        commitments.insert(&Commitment {
            chain_id: 5,
            contract_address: String::from("0x4fd0ade06b9654437f46EA59e6edEe056F9d5EF7"),
            commitment_hash: String::from("9709495941671889428395361755215352896616366060066411186055604144562505250548"),
            asset_symbol: String::from("MTT"),
            asset_decimals: 18,
            asset_address: Some(String::from("0x80C46C896E26C1cB7DCdD23019d9e7cca6854864")),
            status: CommitmentStatus::SrcSucceeded,
            rollup_fee_amount: Some(BigInt::from(20000000000000000u64)),
            encrypted_note: Some(String::from("9f86d4d7e35fb1f2g24f9784d4fced7f045cdf768b82d33e17ed1b62cb0a9706d13ff263c74d746df89a09cfa57405547db8cf3c4300e693d3cbd117f5f0c6e7af10c022c2a0110fa91afbd8ac01a55ad28e7b2ec01a3268a980e2a8c3f349f19e8d26cc8131bbe3f68c418f7cb6ba2bcdcdd86a5b2370792b2a86330096104637f0b7b992436f1a83000a727476b006b05da69f5eb0812f57ad6d871e53dd2f73c6b8ede35effdb39c1f2e78357a96da6f7af8d57d9ae524df2cd001ec1e6cbaa2cf5cb90ded104783af9b5c144c9513e")),
            leaf_index: Some(String::from("0")),
            amount: Some(BigInt::from(1000000000000000000u64)),
            serial_number: Some(String::from("")),
            shielded_address: Some(String::from("0x8695520Db7C1074D07898D655D2Bc7308395B041b")),
            creation_transaction_hash: Some(String::from("0x81d3510c46dfe7a1fc282eb54034b848a3d83f440c551c19e4d513801be00130")),
            spending_transaction_hash: Some(String::from("")),
            rollup_transaction_hash: Some(String::from("")),
        }).await
            .unwrap(),
    );
    assert_eq!(commitments.count_all().await.unwrap(), 1);
    // testing insert_batch
    inserted_commitments.extend(
        commitments.insert_batch(&vec![
            Commitment {
                chain_id: 5,
                contract_address: String::from("0x4fd0ade06b9654437f46EA59e6edEe056F9d5EF7"),
                commitment_hash: String::from("9709505941671889428395361755215352896616366060066411186055604144562505250548"),
                asset_symbol: String::from("FTM"),
                asset_decimals: 18,
                asset_address: Some(String::from("0x81C6C896E26C1cB7DCdD23019d9e7cca6854864")),
                status: CommitmentStatus::Queued,
                rollup_fee_amount: Some(BigInt::from(30000000000000000u64)),
                encrypted_note: Some(String::from("0x3g001f7ca97994c67443db00a45994940489df55198e980b58266b868191c5d472115e8b8c2fd57434d43d020de2f45d250fd0fa317ca73b5e7e341adac479c99449a708cd46a0c058b49e84d568371427ff57c3fe2c7c9371d9ae4e94461947b29ee94029df862a0a4c7e6f971b0d9569988dce59e42109e99c1d50f041e5fea6ca49427f49b12a84407ac4b4e1c050299d4fc3ef3e60cf5c68d91d8bd722e2abd92c98b5a1b2250117951d20b355f66b25b11f4991813604b348f024dc9813540d1ee3882085e56e7b60b205fa2dcab4")),
                leaf_index: Some(String::from("1")),
                amount: Some(BigInt::from(2000000000000000000u64)),
                serial_number: Some(String::from("")),
                shielded_address: Some(String::from("0x9234320Db7C1074D07898D655D2Bc7308395B041b")),
                creation_transaction_hash: Some(String::from("0xc2e0fac7be52ad359a7cab1552d33fc190885dcaf483b555135e9efc0afc0873")),
                spending_transaction_hash: Some(String::from("")),
                rollup_transaction_hash: Some(String::from("")),
            },
            Commitment {
                chain_id: 5,
                contract_address: String::from("0x4fd0ade06b9654437f46EA59e6edEe056F9d5EF7"),
                commitment_hash: String::from("9709515941671889428395361755215352896616366060066411186055604144562505250548"),
                asset_symbol: String::from("MTT"),
                asset_decimals: 18,
                asset_address: Some(String::from("0x80C46C896E26C1cB7DCdD23019d9e7cca6854864")),
                status: CommitmentStatus::Spent,
                rollup_fee_amount: Some(BigInt::from(20000000000000000u64)),
                encrypted_note: Some(String::from("9f86d4d7e35fb1f2g24f9784d4fced7f045cdf768b82d33e17ed1b62cb0a9706d13ff263c74d746df89a09cfa57405547db8cf3c4300e693d3cbd117f5f0c6e7af10c022c2a0110fa91afbd8ac01a55ad28e7b2ec01a3268a980e2a8c3f349f19e8d26cc8131bbe3f68c418f7cb6ba2bcdcdd86a5b2370792b2a86330096104637f0b7b992436f1a83000a727476b006b05da69f5eb0812f57ad6d871e53dd2f73c6b8ede35effdb39c1f2e78357a96da6f7af8d57d9ae524df2cd001ec1e6cbaa2cf5cb90ded104783af9b5c144c9513e")),
                leaf_index: Some(String::from("0")),
                amount: Some(BigInt::from(1000000000000000000u64)),
                serial_number: Some(String::from("[\"5459390987378850672482241551738869467430827449712871069062180681644093249312\"]")),
                shielded_address: Some(String::from("0x89c828e580d0A64d66a95F8d7655F509959915BC")),
                creation_transaction_hash: Some(String::from("")),
                spending_transaction_hash: Some(String::from("0x330687d04916477dc947196237316f1a747dde19eeaf95be65a57ce050c936b7")),
                rollup_transaction_hash: Some(String::from("")),
            },
        ]).await
            .unwrap(),
    );
    assert_eq!(commitments.count_all().await.unwrap(), 3);

    // testing count
    assert_eq!(
        commitments
            .count(
                QueryFilterBuilder::new()
                    .filter(Condition::FILTER(SubFilter::Equal(
                        String::from("leaf_index"),
                        String::from("1")
                    )))
                    .build()
            )
            .await
            .unwrap(),
        1
    );

    // // testing find_all
    let mut found_commitments = commitments.find_all().await.unwrap();
    assert_eq!(found_commitments, inserted_commitments);
    //testing find
    found_commitments = commitments
        .find(QueryFilterBuilder::new().limit(2).offset(1).build())
        .await
        .unwrap();
    assert_eq!(found_commitments, inserted_commitments[1..]);
    // testing find_one
    let mut found_commitment = commitments.find_one(
        QueryFilterBuilder::new()
            .filter(Condition::FILTER(SubFilter::Equal(
                String::from("commitment_hash"),
                String::from("9709495941671889428395361755215352896616366060066411186055604144562505250548")
            )))
            .build(),
    ).await
        .unwrap()
        .unwrap();
    assert_eq!(found_commitment, inserted_commitments[0]);
    // // testing find_by_id
    found_commitment = commitments
        .find_by_id(&inserted_commitments[1].id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_commitment, inserted_commitments[1]);

    // testing update
    found_commitment.data.status = CommitmentStatus::Included;
    let updated_commitment = commitments.update(&found_commitment).await.unwrap();
    assert_eq!(updated_commitment.data, found_commitment.data);
    // testing update_batch
    inserted_commitments[0].data.status = CommitmentStatus::Queued;
    inserted_commitments[2].data.creation_transaction_hash = Some(String::from(
        "0xdc8208b5670c42266587330a4cfc796fa795830e73e9732da4faa884d77caeec",
    ));
    found_commitments = commitments
        .update_batch(&inserted_commitments)
        .await
        .unwrap();
    assert_eq!(found_commitments[0].data, inserted_commitments[0].data);
    assert_eq!(found_commitments[2].data, inserted_commitments[2].data);

    // testing delete
    commitments.delete(&inserted_commitments[0]).await.unwrap();
    assert_eq!(commitments.count_all().await.unwrap(), 2);
    // testing delete_batch
    commitments
        .delete_batch(&vec![inserted_commitments[1].clone()])
        .await
        .unwrap();
    assert_eq!(commitments.count_all().await.unwrap(), 1);
    // testing delete_by_filter
    commitments
        .insert(&inserted_commitments[1].data)
        .await
        .unwrap();
    assert_eq!(commitments.count_all().await.unwrap(), 2);
    commitments.delete_by_filter(
        QueryFilterBuilder::new()
            .filter(Condition::FILTER(SubFilter::Equal(
                String::from("commitment_hash"),
                String::from("9709505941671889428395361755215352896616366060066411186055604144562505250548"),
            )))
            .build(),
    ).await
        .unwrap();
    assert_eq!(commitments.count_all().await.unwrap(), 1);
    // testing delete_all
    commitments.delete_all().await.unwrap();
    assert_eq!(commitments.count_all().await.unwrap(), 0);
}
