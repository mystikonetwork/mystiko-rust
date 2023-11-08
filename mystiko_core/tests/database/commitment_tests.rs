use mystiko_config::MystikoConfig;
use mystiko_core::{Commitment, CommitmentCollection, CommitmentColumn};
use mystiko_dataloader::handler::document::DatabaseCommitment;
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_protos::storage::v1::{ConditionOperator, QueryFilter, SubFilter};
use mystiko_storage::{Collection, Document, SqlStatementFormatter};
use mystiko_storage_sqlite::SqliteStorage;
use mystiko_utils::convert::biguint_to_bytes;
use mystiko_utils::hex::{decode_hex, encode_hex};
use num_bigint::BigUint;
use std::str::FromStr;
use std::sync::Arc;

async fn create_commitments() -> CommitmentCollection<SqlStatementFormatter, SqliteStorage> {
    let storage = SqliteStorage::from_memory().await.unwrap();
    let commitments = CommitmentCollection::new(Arc::new(Collection::new(SqlStatementFormatter::sqlite(), storage)));
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
        commitments
            .insert(&Commitment {
                chain_id: 5,
                contract_address: String::from("0x4fd0ade06b9654437f46EA59e6edEe056F9d5EF7"),
                bridge_type: 1,
                commitment_hash: BigUint::from_str(
                    "9709495941671889428395361755215352896616366060066411186055604144562505250548",
                )
                .unwrap(),
                asset_symbol: String::from("MTT"),
                asset_decimals: 18,
                asset_address: Some(String::from("0x80C46C896E26C1cB7DCdD23019d9e7cca6854864")),
                status: CommitmentStatus::SrcSucceeded as i32,
                spent: true,
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
                queued_transaction_hash: Some(String::from(
                    "0x81d3510c46dfe7a1fc282eb54034b848a3d83f440c551c19e4d513801be00130",
                )),
                included_transaction_hash: Some(String::from("")),
                src_chain_transaction_hash: Some(String::from("")),
            })
            .await
            .unwrap(),
    );
    assert_eq!(commitments.count_all().await.unwrap(), 1);
    // testing insert_batch
    inserted_commitments.extend(
        commitments
            .insert_batch(&vec![
                Commitment {
                    chain_id: 5,
                    contract_address: String::from("0x4fd0ade06b9654437f46EA59e6edEe056F9d5EF7"),
                    bridge_type: 1,
                    commitment_hash: BigUint::from_str(
                        "9709505941671889428395361755215352896616366060066411186\
                        055604144562505250548",
                    )
                    .unwrap(),
                    asset_symbol: String::from("FTM"),
                    asset_decimals: 18,
                    asset_address: Some(String::from("0x81C6C896E26C1cB7DCdD23019d9e7cca6854864")),
                    status: CommitmentStatus::Queued as i32,
                    spent: false,
                    block_number: 10000000u64,
                    src_chain_block_number: Some(10000000u64),
                    included_block_number: Some(10000100u64),
                    rollup_fee_amount: Some(BigUint::from(30000000000000000u64)),
                    encrypted_note: Some(String::from(
                        "0x3g001f7ca97994c67443db00a45994940489df55198e980b58266b868191\
                        c5d472115e8b8c2fd57434d43d020de2f45d250fd0fa317ca73b5e7e341adac479c99449a708cd46a0c058b49e84d\
                        568371427ff57c3fe2c7c9371d9ae4e94461947b29ee94029df862a0a4c7e6f971b0d9569988dce59e42109e99c1d\
                        50f041e5fea6ca49427f49b12a84407ac4b4e1c050299d4fc3ef3e60cf5c68d91d8bd722e2abd92c98b5a1b225011\
                        7951d20b355f66b25b11f4991813604b348f024dc9813540d1ee3882085e56e7b60b205fa2dcab4",
                    )),
                    leaf_index: Some(1u64),
                    amount: Some(BigUint::from(2000000000000000000u64)),
                    nullifier: None,
                    shielded_address: Some(String::from("0x9234320Db7C1074D07898D655D2Bc7308395B041b")),
                    queued_transaction_hash: Some(String::from(
                        "0xc2e0fac7be52ad359a7cab1552d33fc190885dcaf483b555135e9efc0afc0873",
                    )),
                    included_transaction_hash: Some(String::from("")),
                    src_chain_transaction_hash: Some(String::from("")),
                },
                Commitment {
                    chain_id: 5,
                    contract_address: String::from("0x4fd0ade06b9654437f46EA59e6edEe056F9d5EF7"),
                    bridge_type: 1,
                    commitment_hash: BigUint::from_str(
                        "9709515941671889428395361755215352896616366060066411186055604144562505250548",
                    )
                    .unwrap(),
                    asset_symbol: String::from("MTT"),
                    asset_decimals: 18,
                    asset_address: Some(String::from("0x80C46C896E26C1cB7DCdD23019d9e7cca6854864")),
                    status: CommitmentStatus::Included as i32,
                    spent: true,
                    block_number: 10000000u64,
                    src_chain_block_number: Some(10000000u64),
                    included_block_number: Some(10000100u64),
                    rollup_fee_amount: Some(BigUint::from(20000000000000000u64)),
                    encrypted_note: Some(String::from(
                        "9f86d4d7e35fb1f2a24f9784d4fced7f045cdf768b82d33e17ed1b62cb0a97\
                        06d13ff263c74d746df89a09cfa57405547db8cf3c4300e693d3cbd117f5f0c6e7af10c022c2a0110fa91afbd8ac01\
                        a55ad28e7b2ec01a3268a980e2a8c3f349f19e8d26cc8131bbe3f68c418f7cb6ba2bcdcdd86a5b2370792b2a863300\
                        96104637f0b7b992436f1a83000a727476b006b05da69f5eb0812f57ad6d871e53dd2f73c6b8ede35effdb39c1f2e7\
                        8357a96da6f7af8d57d9ae524df2cd001ec1e6cbaa2cf5cb90ded104783af9b5c144c9513e",
                    )),
                    leaf_index: Some(2u64),
                    amount: Some(BigUint::from(1000000000000000000u64)),
                    nullifier: Some(
                        BigUint::from_str(
                            "5459390987378850672482241551738869467430827449712871069062180681644093249312",
                        )
                        .unwrap(),
                    ),
                    shielded_address: Some(String::from("0x89c828e580d0A64d66a95F8d7655F509959915BC")),
                    queued_transaction_hash: Some(String::from("")),
                    included_transaction_hash: Some(String::from(
                        "0x330687d04916477dc947196237316f1a747dde19eeaf95be65a57ce050c936b7",
                    )),
                    src_chain_transaction_hash: Some(String::from("")),
                },
            ])
            .await
            .unwrap(),
    );
    assert_eq!(commitments.count_all().await.unwrap(), 3);

    // testing count
    assert_eq!(
        commitments
            .count(SubFilter::equal(CommitmentColumn::LeafIndex, 1u64))
            .await
            .unwrap(),
        1
    );

    // // testing find_all
    let mut found_commitments = commitments.find_all().await.unwrap();
    assert_eq!(found_commitments, inserted_commitments);
    //testing find
    found_commitments = commitments
        .find(
            QueryFilter::builder()
                .conditions_operator(ConditionOperator::And)
                .limit(2)
                .offset(1)
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(found_commitments, inserted_commitments[1..]);
    // testing find_one
    let mut found_commitment = commitments
        .find_one(SubFilter::equal(
            CommitmentColumn::CommitmentHash,
            num_bigint::BigUint::from_str(
                "9709495941671889428395361755215352896616366060066411186055604144562505250548",
            )
            .unwrap(),
        ))
        .await
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
    found_commitment.data.status = CommitmentStatus::Included as i32;
    let updated_commitment = commitments.update(&found_commitment).await.unwrap();
    assert_eq!(updated_commitment.data, found_commitment.data);
    // testing update_batch
    inserted_commitments[0].data.status = CommitmentStatus::Queued as i32;
    inserted_commitments[2].data.queued_transaction_hash = Some(String::from(
        "0xdc8208b5670c42266587330a4cfc796fa795830e73e9732da4faa884d77caeec",
    ));
    found_commitments = commitments.update_batch(&inserted_commitments).await.unwrap();
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
    commitments.insert(&inserted_commitments[1].data).await.unwrap();
    assert_eq!(commitments.count_all().await.unwrap(), 2);
    commitments
        .delete_by_filter(SubFilter::equal(
            CommitmentColumn::CommitmentHash,
            num_bigint::BigUint::from_str(
                "9709505941671889428395361755215352896616366060066411186055604144562505250548",
            )
            .unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(commitments.count_all().await.unwrap(), 1);
    // testing delete_all
    commitments.delete_all().await.unwrap();
    assert_eq!(commitments.count_all().await.unwrap(), 0);
}

#[tokio::test]
async fn test_commitment_serde() {
    let commitments = create_commitments().await;
    let commitment = commitments
        .insert(&Commitment {
            chain_id: 5,
            contract_address: String::from("0x4fd0ade06b9654437f46EA59e6edEe056F9d5EF7"),
            bridge_type: 1,
            commitment_hash: BigUint::from_str(
                "9709495941671889428395361755215352896616366060066411186055604144562505250548",
            )
            .unwrap(),
            asset_symbol: String::from("MTT"),
            asset_decimals: 18,
            asset_address: Some(String::from("0x80C46C896E26C1cB7DCdD23019d9e7cca6854864")),
            status: CommitmentStatus::SrcSucceeded as i32,
            spent: true,
            block_number: 10000000u64,
            src_chain_block_number: Some(10000000u64),
            included_block_number: Some(10000100u64),
            rollup_fee_amount: Some(BigUint::from(20000000000000000u64)),
            encrypted_note: Some(String::from(
                "9f86d4d7e35fb1f2a24f9784d4fced7f045cdf768b82d33e17ed1b62cb0a97\
                06d13ff263c74d746df89a09cfa57405547db8cf3c4300e693d3cbd117f5f0c6e7af10c022c2a0110fa91afbd8ac01a55\
                ad28e7b2ec01a3268a980e2a8c3f349f19e8d26cc8131bbe3f68c418f7cb6ba2bcdcdd86a5b2370792b2a863300961046\
                37f0b7b992436f1a83000a727476b006b05da69f5eb0812f57ad6d871e53dd2f73c6b8ede35effdb39c1f2e78357a96da\
                6f7af8d57d9ae524df2cd001ec1e6cbaa2cf5cb90ded104783af9b5c144c9513e",
            )),
            leaf_index: Some(0u64),
            amount: Some(BigUint::from(1000000000000000000u64)),
            nullifier: None,
            shielded_address: Some(String::from("0x8695520Db7C1074D07898D655D2Bc7308395B041b")),
            queued_transaction_hash: Some(String::from(
                "0x81d3510c46dfe7a1fc282eb54034b848a3d83f440c551c19e4d513801be00130",
            )),
            included_transaction_hash: Some(String::from("")),
            src_chain_transaction_hash: Some(String::from("")),
        })
        .await
        .unwrap();
    assert_eq!(
        commitment,
        serde_json::from_str(&serde_json::to_string(&commitment).unwrap()).unwrap()
    );
}

#[tokio::test]
async fn test_loader_database_commitment() {
    assert_eq!(Commitment::column_chain_id(), "chain_id");
    assert_eq!(Commitment::column_contract_address(), "contract_address");
    assert_eq!(Commitment::column_bridge_type(), "bridge_type");
    assert_eq!(Commitment::column_commitment_hash(), "commitment_hash");
    assert_eq!(Commitment::column_block_number(), "block_number");
    assert_eq!(Commitment::column_status(), "status");
    assert_eq!(Commitment::column_src_block_number(), "src_chain_block_number");
    assert_eq!(Commitment::column_included_block_number(), "included_block_number");
    assert_eq!(Commitment::column_rollup_fee(), "rollup_fee_amount");
    assert_eq!(Commitment::column_encrypted_note(), "encrypted_note");
    assert_eq!(Commitment::column_leaf_index(), "leaf_index");
    assert_eq!(Commitment::column_queued_transaction_hash(), "queued_transaction_hash");
    assert_eq!(
        Commitment::column_included_transaction_hash(),
        "included_transaction_hash"
    );
    assert_eq!(Commitment::column_src_transaction_hash(), "src_chain_transaction_hash");
    let mut commitment = Commitment {
        chain_id: 5,
        contract_address: String::from("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"),
        bridge_type: 1,
        commitment_hash: BigUint::from_str(
            "9709495941671889428395361755215352896616366060066411186055604144562505250548",
        )
        .unwrap(),
        asset_symbol: String::from("MTT"),
        asset_decimals: 16,
        asset_address: Some(String::from("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")),
        status: CommitmentStatus::SrcSucceeded as i32,
        spent: true,
        block_number: 10000000u64,
        src_chain_block_number: Some(10000000u64),
        included_block_number: Some(10000100u64),
        rollup_fee_amount: Some(BigUint::from(20000000000000000u64)),
        encrypted_note: Some(String::from(
            "9f86d4d7e35fb1f2a24f9784d4fced7f045cdf768b82d33e17ed1b62cb\
            0a9706d13ff263c74d746df89a09cfa57405547db8cf3c4300e693d3cbd117f5f0c6e7af10c022c2a0110\
            fa91afbd8ac01a55ad28e7b2ec01a3268a980e2a8c3f349f19e8d26cc8131bbe3f68c418f7cb6ba2bcdcd\
            d86a5b2370792b2a86330096104637f0b7b992436f1a83000a727476b006b05da69f5eb0812f57ad6d871\
            e53dd2f73c6b8ede35effdb39c1f2e78357a96da6f7af8d57d9ae524df2cd001ec1e6cbaa2cf5cb90ded1\
            04783af9b5c144c9513e",
        )),
        leaf_index: Some(0u64),
        amount: Some(BigUint::from(1000000000000000000u64)),
        nullifier: None,
        shielded_address: Some(String::from("0x8695520Db7C1074D07898D655D2Bc7308395B041b")),
        queued_transaction_hash: Some(String::from(
            "0x81d3510c46dfe7a1fc282eb54034b848a3d83f440c551c19e4d513801be00130",
        )),
        included_transaction_hash: Some(String::from(
            "0x81d3510c46dfe7a1fc282eb54034b848a3d83f440c551c19e4d513801be00130",
        )),
        src_chain_transaction_hash: Some(String::from(
            "0x81d3510c46dfe7a1fc282eb54034b848a3d83f440c551c19e4d513801be00130",
        )),
    };
    assert_eq!(commitment.get_chain_id(), 5);
    assert_eq!(
        commitment.get_contract_address(),
        "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
    );
    assert_eq!(commitment.get_bridge_type(), 1);
    assert_eq!(
        commitment.get_commitment_hash(),
        &BigUint::from_str("9709495941671889428395361755215352896616366060066411186055604144562505250548",).unwrap()
    );
    assert_eq!(commitment.get_block_number(), 10000000u64);
    assert_eq!(commitment.get_status(), CommitmentStatus::SrcSucceeded as i32);
    assert_eq!(commitment.get_src_block_number(), Some(10000000u64));
    assert_eq!(commitment.get_included_block_number(), Some(10000100u64));
    assert_eq!(commitment.get_rollup_fee(), Some(&BigUint::from(20000000000000000u64)));
    assert_eq!(
        commitment.get_encrypted_note(),
        Some(&String::from(
            "9f86d4d7e35fb1f2a24f9784d4fced7f045cdf768b82d33e17ed1b62cb\
            0a9706d13ff263c74d746df89a09cfa57405547db8cf3c4300e693d3cbd117f5f0c6e7af10c022c2a0110\
            fa91afbd8ac01a55ad28e7b2ec01a3268a980e2a8c3f349f19e8d26cc8131bbe3f68c418f7cb6ba2bcdcd\
            d86a5b2370792b2a86330096104637f0b7b992436f1a83000a727476b006b05da69f5eb0812f57ad6d871\
            e53dd2f73c6b8ede35effdb39c1f2e78357a96da6f7af8d57d9ae524df2cd001ec1e6cbaa2cf5cb90ded1\
            04783af9b5c144c9513e"
        ))
    );
    assert_eq!(commitment.get_leaf_index(), Some(0u64));
    assert_eq!(
        commitment.get_queued_transaction_hash(),
        Some(&String::from(
            "0x81d3510c46dfe7a1fc282eb54034b848a3d83f440c551c19e4d513801be00130",
        ))
    );
    assert_eq!(
        commitment.get_included_transaction_hash(),
        Some(&String::from(
            "0x81d3510c46dfe7a1fc282eb54034b848a3d83f440c551c19e4d513801be00130"
        ))
    );
    assert_eq!(
        commitment.get_src_transaction_hash(),
        Some(&String::from(
            "0x81d3510c46dfe7a1fc282eb54034b848a3d83f440c551c19e4d513801be00130"
        ))
    );

    let mut proto_commitment = commitment.clone().into_proto().unwrap();
    assert_eq!(
        proto_commitment.commitment_hash,
        biguint_to_bytes(
            &BigUint::from_str("9709495941671889428395361755215352896616366060066411186055604144562505250548").unwrap()
        )
    );
    assert_eq!(proto_commitment.block_number, 10000000u64);
    assert_eq!(proto_commitment.status, CommitmentStatus::SrcSucceeded as i32);
    assert_eq!(proto_commitment.src_chain_block_number, Some(10000000u64));
    assert_eq!(proto_commitment.included_block_number, Some(10000100u64));
    assert_eq!(
        proto_commitment.rollup_fee,
        Some(biguint_to_bytes(&BigUint::from(20000000000000000u64)))
    );
    assert_eq!(
        proto_commitment.encrypted_note,
        Some(
            decode_hex(
                "9f86d4d7e35fb1f2a24f9784d4fced7f045cdf768b82d33e17ed1b62cb\
                0a9706d13ff263c74d746df89a09cfa57405547db8cf3c4300e693d3cbd117f5f0c6e7af10c022c2a0110\
                fa91afbd8ac01a55ad28e7b2ec01a3268a980e2a8c3f349f19e8d26cc8131bbe3f68c418f7cb6ba2bcdcd\
                d86a5b2370792b2a86330096104637f0b7b992436f1a83000a727476b006b05da69f5eb0812f57ad6d871\
                e53dd2f73c6b8ede35effdb39c1f2e78357a96da6f7af8d57d9ae524df2cd001ec1e6cbaa2cf5cb90ded1\
                04783af9b5c144c9513e"
            )
            .unwrap()
        )
    );
    assert_eq!(proto_commitment.leaf_index, Some(0u64));
    assert_eq!(
        proto_commitment.queued_transaction_hash,
        Some(decode_hex("0x81d3510c46dfe7a1fc282eb54034b848a3d83f440c551c19e4d513801be00130",).unwrap())
    );
    assert_eq!(
        proto_commitment.included_transaction_hash,
        Some(decode_hex("0x81d3510c46dfe7a1fc282eb54034b848a3d83f440c551c19e4d513801be00130").unwrap())
    );
    assert_eq!(
        proto_commitment.src_chain_transaction_hash,
        Some(decode_hex("0x81d3510c46dfe7a1fc282eb54034b848a3d83f440c551c19e4d513801be00130").unwrap())
    );

    let config = Arc::new(
        MystikoConfig::from_json_file("tests/files/mystiko/config.json")
            .await
            .unwrap(),
    );
    let converted_commitment = Commitment::from_proto(
        config.clone(),
        commitment.chain_id,
        &commitment.contract_address,
        commitment.bridge_type,
        proto_commitment.clone(),
    )
    .unwrap();
    compare_commitment(&converted_commitment, &commitment);
    assert!(Commitment::from_proto(
        config.clone(),
        123434u64,
        &commitment.contract_address,
        commitment.bridge_type,
        proto_commitment.clone(),
    )
    .is_err());

    commitment.chain_id = 97;
    commitment.contract_address = String::from("0xd791049D0a154bC7860804e1A18ACD148Eb0afD9");
    commitment.asset_address = None;
    commitment.asset_symbol = String::from("BNB");
    commitment.asset_decimals = 18;
    let converted_commitment = Commitment::from_proto(
        config.clone(),
        commitment.chain_id,
        &commitment.contract_address,
        commitment.bridge_type,
        proto_commitment.clone(),
    )
    .unwrap();
    compare_commitment(&converted_commitment, &commitment);

    proto_commitment.block_number -= 1;
    commitment
        .update_by_proto(config.clone(), &proto_commitment.clone())
        .unwrap();
    assert_eq!(commitment.block_number, proto_commitment.block_number);

    proto_commitment.status = CommitmentStatus::Included as i32;
    commitment
        .update_by_proto(config.clone(), &proto_commitment.clone())
        .unwrap();
    assert_eq!(commitment.status, proto_commitment.status);

    commitment.src_chain_block_number = None;
    commitment
        .update_by_proto(config.clone(), &proto_commitment.clone())
        .unwrap();
    assert_eq!(
        commitment.src_chain_block_number,
        proto_commitment.src_chain_block_number
    );

    commitment.included_block_number = None;
    commitment
        .update_by_proto(config.clone(), &proto_commitment.clone())
        .unwrap();
    assert_eq!(commitment.included_block_number, proto_commitment.included_block_number);

    commitment.rollup_fee_amount = None;
    commitment
        .update_by_proto(config.clone(), &proto_commitment.clone())
        .unwrap();
    assert_eq!(commitment.rollup_fee_amount, proto_commitment.rollup_fee_as_biguint());

    commitment.encrypted_note = None;
    commitment
        .update_by_proto(config.clone(), &proto_commitment.clone())
        .unwrap();
    assert_eq!(
        commitment.encrypted_note,
        proto_commitment.encrypted_note.as_ref().map(encode_hex)
    );

    commitment.leaf_index = None;
    commitment
        .update_by_proto(config.clone(), &proto_commitment.clone())
        .unwrap();
    assert_eq!(commitment.leaf_index, proto_commitment.leaf_index);

    commitment.queued_transaction_hash = None;
    commitment
        .update_by_proto(config.clone(), &proto_commitment.clone())
        .unwrap();
    assert_eq!(
        commitment.queued_transaction_hash,
        proto_commitment.queued_transaction_hash_as_hex()
    );

    commitment.included_transaction_hash = None;
    commitment
        .update_by_proto(config.clone(), &proto_commitment.clone())
        .unwrap();
    assert_eq!(
        commitment.included_transaction_hash,
        proto_commitment.included_transaction_hash_as_hex()
    );

    commitment.src_chain_transaction_hash = None;
    commitment
        .update_by_proto(config.clone(), &proto_commitment.clone())
        .unwrap();
    assert_eq!(
        commitment.src_chain_transaction_hash,
        proto_commitment.src_chain_transaction_hash_as_hex()
    );
}

fn compare_commitment(commitment1: &Commitment, commitment2: &Commitment) {
    assert_eq!(commitment1.chain_id, commitment2.chain_id);
    assert_eq!(commitment1.contract_address, commitment2.contract_address);
    assert_eq!(commitment1.commitment_hash, commitment2.commitment_hash);
    assert_eq!(commitment1.block_number, commitment2.block_number);
    assert_eq!(commitment1.asset_symbol, commitment2.asset_symbol);
    assert_eq!(commitment1.asset_decimals, commitment2.asset_decimals);
    assert_eq!(commitment1.asset_address, commitment2.asset_address);
    assert_eq!(commitment1.status, commitment2.status);
    assert_eq!(commitment1.src_chain_block_number, commitment2.src_chain_block_number);
    assert_eq!(commitment1.included_block_number, commitment2.included_block_number);
    assert_eq!(commitment1.rollup_fee_amount, commitment2.rollup_fee_amount);
    assert_eq!(commitment1.encrypted_note, commitment2.encrypted_note);
    assert_eq!(commitment1.leaf_index, commitment2.leaf_index);
    assert_eq!(commitment1.queued_transaction_hash, commitment2.queued_transaction_hash);
    assert_eq!(
        commitment1.included_transaction_hash,
        commitment2.included_transaction_hash
    );
    assert_eq!(
        commitment1.src_chain_transaction_hash,
        commitment2.src_chain_transaction_hash
    );
}
