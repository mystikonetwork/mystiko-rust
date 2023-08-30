use mystiko_dataloader::data::{ChainData, FullData};
use mystiko_protos::data::v1::{
    ChainData as ProtoChainData, Commitment, CommitmentStatus, ContractData as ProtoContractData, Nullifier,
};
use mystiko_utils::hex::decode_hex;

#[test]
fn test_chain_data_from_proto() {
    let commitment = Commitment::builder()
        .block_number(1000000u64)
        .status(CommitmentStatus::Included as i32)
        .commitment_hash(vec![1, 2, 3])
        .build();
    let nullifier = Nullifier::builder()
        .block_number(1000000u64)
        .nullifier(vec![1, 2, 3])
        .transaction_hash(vec![1, 2, 3])
        .build();
    let proto_contract_data = ProtoContractData::builder()
        .contract_address(decode_hex("0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326").unwrap())
        .commitments(vec![commitment.clone()])
        .nullifiers(vec![nullifier.clone()])
        .build();
    let proto_chain_data = ProtoChainData::builder()
        .start_block(1000000u64)
        .end_block(1000001u64)
        .contract_data(vec![proto_contract_data])
        .build();
    let chain_data = ChainData::<FullData>::from_proto(1, proto_chain_data);
    assert_eq!(chain_data.contracts_data.len(), 1);
    let contract_data = chain_data.contracts_data.into_iter().next().unwrap();
    assert_eq!(contract_data.address, "0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326");
    assert_eq!(contract_data.start_block, 1000000);
    assert_eq!(contract_data.end_block, 1000001);
    let full_data = contract_data.data.unwrap();
    assert_eq!(full_data.commitments, vec![commitment]);
    assert_eq!(full_data.nullifiers, vec![nullifier]);
}
