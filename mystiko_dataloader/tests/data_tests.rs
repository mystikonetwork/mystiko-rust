use mystiko_dataloader::data::{ChainData, ContractData, FullData};
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

#[test]
fn test_chain_data_into_proto() {
    let chain_data = ChainData::<FullData>::builder().chain_id(1u64).build();
    assert!(chain_data.into_proto().unwrap().is_none());
    let commitments = vec![Commitment::builder()
        .block_number(1000000u64)
        .status(CommitmentStatus::Included as i32)
        .commitment_hash(vec![1, 2, 3])
        .build()];
    let nullifiers = vec![Nullifier::builder()
        .block_number(1000000u64)
        .nullifier(vec![1, 2, 3])
        .transaction_hash(vec![1, 2, 3])
        .build()];
    let full_data = FullData::builder()
        .commitments(commitments.clone())
        .nullifiers(nullifiers.clone())
        .build();
    let contract_data1 = ContractData::<FullData>::builder()
        .start_block(1000000u64)
        .end_block(1000001u64)
        .address("0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326")
        .data(full_data.clone())
        .build();
    let contract_data2 = ContractData::<FullData>::builder()
        .start_block(1000000u64)
        .end_block(1000001u64)
        .address("0x795E1dFCE63F2500Ce3C68834324E6f62c5d5015")
        .build();
    let contract_data3 = ContractData::<FullData>::builder()
        .start_block(1000000u64)
        .end_block(1000002u64)
        .address("0x3c11F6265Ddec22f4d049Dde480615735f451646")
        .data(full_data.clone())
        .build();
    let contract_data4 = ContractData::<FullData>::builder()
        .start_block(1000001u64)
        .end_block(1000001u64)
        .address("0x439190E826DDAd6ECBC12E26C69666E5A034800b")
        .data(full_data.clone())
        .build();

    let chain_data = ChainData::<FullData>::builder()
        .chain_id(1u64)
        .contracts_data(vec![contract_data1.clone(), contract_data2])
        .build();
    let proto_chain_data = chain_data.into_proto().unwrap().unwrap();
    assert_eq!(proto_chain_data.start_block, 1000000);
    assert_eq!(proto_chain_data.end_block, 1000001);
    assert_eq!(proto_chain_data.contract_data.len(), 2);
    let proto_contract_data1 = proto_chain_data.contract_data.get(0).unwrap();
    assert_eq!(
        proto_contract_data1.contract_address,
        decode_hex("0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326").unwrap()
    );
    assert_eq!(proto_contract_data1.commitments, commitments);
    assert_eq!(proto_contract_data1.nullifiers, nullifiers);
    let proto_contract_data2 = proto_chain_data.contract_data.get(1).unwrap();
    assert_eq!(
        proto_contract_data2.contract_address,
        decode_hex("0x795E1dFCE63F2500Ce3C68834324E6f62c5d5015").unwrap()
    );
    assert!(proto_contract_data2.commitments.is_empty());
    assert!(proto_contract_data2.nullifiers.is_empty());

    let chain_data = ChainData::<FullData>::builder()
        .chain_id(1u64)
        .contracts_data(vec![contract_data1.clone(), contract_data3])
        .build();
    assert_eq!(
        chain_data.into_proto().unwrap_err().to_string(),
        "all contracts data must have the same start and end blocks"
    );

    let chain_data = ChainData::<FullData>::builder()
        .chain_id(1u64)
        .contracts_data(vec![contract_data1, contract_data4])
        .build();
    assert_eq!(
        chain_data.into_proto().unwrap_err().to_string(),
        "all contracts data must have the same start and end blocks"
    );
}
