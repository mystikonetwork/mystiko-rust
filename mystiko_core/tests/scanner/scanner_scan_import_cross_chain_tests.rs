use crate::scanner::{
    build_mock_provider_with_cross_chain_event, create_scanner, MockCommitmentPoolContracts, MockSequencerClient,
    DEFAULT_WALLET_PASSWORD,
};
use mystiko_core::ScannerHandler;
use mystiko_protos::core::scanner::v1::{
    AssetChainImportOptions, AssetChainImportResult, AssetImportOptions, AssetImportResult,
};
use mystiko_protos::data::v1::Commitment as ProtosCommitment;
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_utils::hex::decode_hex;
use num_bigint::BigUint;
use std::collections::HashMap;

#[tokio::test]
async fn test_import_one_cross_chain_commitment() {
    let chain_id = 5_u64;
    let account_count = 1_usize;
    let mut mock_commitment_pool = MockCommitmentPoolContracts::default();
    mock_commitment_pool
        .expect_is_spent_nullifier()
        .returning(|_| Ok(false));
    let provider = build_mock_provider_with_cross_chain_event(1);
    let mut sequencer = MockSequencerClient::default();
    sequencer.expect_get_commitments().returning(|_, _, _| {
        let cm = ProtosCommitment::builder().status(CommitmentStatus::Included).build();
        Ok(vec![cm])
    });
    let (scanner, db, _) = create_scanner(
        account_count,
        Some("fragile hat december author fancy include nominee spot produce priority income inmate catch aware level poet group pretty rude exit route pizza perfect anger".to_string()),
        HashMap::from([(chain_id, provider)]),
        Some(mock_commitment_pool),Some(sequencer),
    )
        .await;

    let options = AssetImportOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .chains([AssetChainImportOptions::builder()
            .chain_id(chain_id)
            .tx_hashes(vec![
                "0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2".to_string(),
            ])
            .build()])
        .build();
    let r = scanner.import(options).await.unwrap();
    assert_eq!(
        r,
        AssetImportResult::builder()
            .chains([AssetChainImportResult::builder()
                .chain_id(97_u32)
                .found_count(1_u32)
                .imported_count(0_u32)
                .build()])
            .build()
    );
    let cm = db.commitments.find_all().await.unwrap();
    assert_eq!(cm.len(), 0);
}

#[tokio::test]
async fn test_import_two_cross_chain_commitment() {
    let chain_id = 5_u64;
    let account_count = 1_usize;
    let mut mock_commitment_pool = MockCommitmentPoolContracts::default();
    mock_commitment_pool
        .expect_is_spent_nullifier()
        .returning(|_| Ok(false));
    let provider = build_mock_provider_with_cross_chain_event(2);
    let mut sequencer = MockSequencerClient::default();
    sequencer.expect_get_commitments().returning(|_, _, cm| {
        let mut cm_data = ProtosCommitment::builder().build();
        if cm[0] == BigUint::parse_bytes("1db84c1b0bd7877f4cddd3f5b0a8ae202b017234f84dc75face85b7556951fc4".as_bytes(),16).unwrap() {
            cm_data.encrypted_note =  Some(decode_hex("0xd254df768aad077345bcca57768ba1df0424264ca8f76a2fab3549b051e5d0339f73f26c5d4f2e0e1e2575829a0a7afeb0a380f67b100b866b0bfbc84713de7b92d622a98cdbec15690d203cc33b428591d8f549ff7e48e44475f14c3d04b4c9c1c16c3c8beac96ae05db6762e01466d49e04422b9d84a7335b798bb7fbccb2b2fe7732bd991c9a29c861b6c48a692da6e02af21b6a0410aedbdf5f4a3094729873d58f42cc59ac41c54a2b7b2b392277ed72a1b7121efc24d039d7830e20e1f3cb60de6e96e33ed7f344962635832c173").unwrap());
            cm_data.status = CommitmentStatus::Included.into();
            cm_data.leaf_index = Some(1);
            cm_data.block_number = 100;
        } else if cm[0]
            == BigUint::parse_bytes("18812c5d6d451a1c7396e04aaaed04ddbbe8a3908d3db55a890a9527ba4ea8c3".as_bytes(),16).unwrap()
        {
            cm_data.encrypted_note = Some(decode_hex("0x61e3ffa2a53509de01250f2b8af2e0d50433e50f647361b877918a55745d075c9bbf7aa8a1f41bd3641a606a0cbcd00d459fcf231acafdec6eca9b7aacfd5f67e1b63529a0564e82cdde6754d6b946f5bae36f748e3295276df68b8b19d1d8bccd8105d7feca23c5f6288a78f9f038c5d3594d2ef903fc145b07b203f98796c58a9210a5f7c37e07215f9b1ac273c51688c6b72dad855e7008d81f69c28762b289caf7d93d44b6416f7fce60b53669624a9ec0d3289f2a8ee8b8811661fc73d9ece7376574c15baa8a8b21083aa49f8d54").unwrap());
            cm_data.status = CommitmentStatus::Queued.into();
            cm_data.leaf_index = Some(2);
            cm_data.block_number = 200;
        }
        Ok(vec![cm_data])
    });
    let (scanner, db, _) = create_scanner(
        account_count,
        Some("fragile hat december author fancy include nominee spot produce priority income inmate catch aware level poet group pretty rude exit route pizza perfect anger".to_string()),
        HashMap::from([(chain_id, provider)]),
        Some(mock_commitment_pool),Some(sequencer),
    )
        .await;

    let options = AssetImportOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .chains([AssetChainImportOptions::builder()
            .chain_id(chain_id)
            .tx_hashes(vec![
                "0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2".to_string(),
            ])
            .build()])
        .build();
    let r = scanner.import(options).await.unwrap();
    assert_eq!(
        r,
        AssetImportResult::builder()
            .chains([AssetChainImportResult::builder()
                .chain_id(97_u32)
                .found_count(2_u32)
                .imported_count(2_u32)
                .build()])
            .build()
    );
    let cm = db.commitments.find_all().await.unwrap();
    assert_eq!(cm.len(), 2);
    assert_eq!(cm[0].data.status, CommitmentStatus::Included as i32);
    assert_eq!(cm[0].data.leaf_index, Some(1));
    assert!(!cm[0].data.spent);
    assert_eq!(cm[1].data.status, CommitmentStatus::Queued as i32);
    assert_eq!(cm[1].data.leaf_index, Some(2));
    assert!(!cm[1].data.spent);
}
