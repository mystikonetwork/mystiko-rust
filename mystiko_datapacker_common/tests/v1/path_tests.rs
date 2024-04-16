use mystiko_datapacker_common::v1::PathSchema as PathSchemaV1;
use mystiko_datapacker_common::PathSchema;

#[test]
fn test_default() {
    let path_schema = PathSchemaV1::default();
    assert_eq!(path_schema.chain_path(1).to_str().unwrap(), "/chains/1");
    assert_eq!(path_schema.contracts_path(1).to_str().unwrap(), "/chains/1/contracts");
    assert_eq!(
        path_schema.contract_path(1, "0x1234").to_str().unwrap(),
        "/chains/1/contracts/0x1234"
    );
    assert_eq!(
        path_schema.merkle_tree_path(1, "0x1234").to_str().unwrap(),
        "/chains/1/contracts/0x1234/merkle_tree"
    );
    assert_eq!(
        path_schema.merkle_tree_index_path(1, "0x1234").to_str().unwrap(),
        "/chains/1/contracts/0x1234/merkle_tree/index.json"
    );
    assert_eq!(
        path_schema.merkle_tree_data_path(1, "0x1234", 1000).to_str().unwrap(),
        "/chains/1/contracts/0x1234/merkle_tree/1000.zst"
    );
    assert_eq!(
        path_schema.merkle_tree_latest_data_path(1, "0x1234").to_str().unwrap(),
        "/chains/1/contracts/0x1234/merkle_tree/latest.zst"
    );
    assert_eq!(
        path_schema.granularities_path(1).to_str().unwrap(),
        "/chains/1/granularities"
    );
    assert_eq!(
        path_schema.granularity_path(1, 1000).to_str().unwrap(),
        "/chains/1/granularities/1000"
    );
    assert_eq!(
        path_schema.granularity_index(1, 1000).to_str().unwrap(),
        "/chains/1/granularities/1000/index.json"
    );
    assert_eq!(
        path_schema.blocks_path(1, 1000).to_str().unwrap(),
        "/chains/1/granularities/1000/blocks"
    );
    assert_eq!(
        path_schema.data_path(1, 1000, 100000).to_str().unwrap(),
        "/chains/1/granularities/1000/blocks/100000/data.zst"
    );
    assert_eq!(
        path_schema.checksum_path(1, 1000, 100000).to_str().unwrap(),
        "/chains/1/granularities/1000/blocks/100000/data_checksum.sha512"
    );
}

#[test]
fn test_different_settings() {
    let path_schema = PathSchemaV1::builder()
        .base("packer/v1")
        .data_suffix("zstd")
        .checksum_suffix("checksum")
        .build();
    assert_eq!(path_schema.chain_path(1).to_str().unwrap(), "packer/v1/chains/1");
    assert_eq!(
        path_schema.contracts_path(1).to_str().unwrap(),
        "packer/v1/chains/1/contracts"
    );
    assert_eq!(
        path_schema.contract_path(1, "0x1234").to_str().unwrap(),
        "packer/v1/chains/1/contracts/0x1234"
    );
    assert_eq!(
        path_schema.merkle_tree_path(1, "0x1234").to_str().unwrap(),
        "packer/v1/chains/1/contracts/0x1234/merkle_tree"
    );
    assert_eq!(
        path_schema.merkle_tree_index_path(1, "0x1234").to_str().unwrap(),
        "packer/v1/chains/1/contracts/0x1234/merkle_tree/index.json"
    );
    assert_eq!(
        path_schema.merkle_tree_data_path(1, "0x1234", 1000).to_str().unwrap(),
        "packer/v1/chains/1/contracts/0x1234/merkle_tree/1000.zstd"
    );
    assert_eq!(
        path_schema.merkle_tree_latest_data_path(1, "0x1234").to_str().unwrap(),
        "packer/v1/chains/1/contracts/0x1234/merkle_tree/latest.zstd"
    );
    assert_eq!(
        path_schema.granularities_path(1).to_str().unwrap(),
        "packer/v1/chains/1/granularities"
    );
    assert_eq!(
        path_schema.granularity_path(1, 1000).to_str().unwrap(),
        "packer/v1/chains/1/granularities/1000"
    );
    assert_eq!(
        path_schema.granularity_index(1, 1000).to_str().unwrap(),
        "packer/v1/chains/1/granularities/1000/index.json"
    );
    assert_eq!(
        path_schema.blocks_path(1, 1000).to_str().unwrap(),
        "packer/v1/chains/1/granularities/1000/blocks"
    );
    assert_eq!(
        path_schema.data_path(1, 1000, 100000).to_str().unwrap(),
        "packer/v1/chains/1/granularities/1000/blocks/100000/data.zstd"
    );
    assert_eq!(
        path_schema.checksum_path(1, 1000, 100000).to_str().unwrap(),
        "packer/v1/chains/1/granularities/1000/blocks/100000/data_checksum.checksum"
    );
}

#[test]
fn test_box() {
    let path_schema: Box<dyn PathSchema> = Box::<PathSchemaV1>::default();
    assert_eq!(path_schema.chain_path(1).to_str().unwrap(), "/chains/1");
    assert_eq!(path_schema.contracts_path(1).to_str().unwrap(), "/chains/1/contracts");
    assert_eq!(
        path_schema.contract_path(1, "0x1234").to_str().unwrap(),
        "/chains/1/contracts/0x1234"
    );
    assert_eq!(
        path_schema.merkle_tree_path(1, "0x1234").to_str().unwrap(),
        "/chains/1/contracts/0x1234/merkle_tree"
    );
    assert_eq!(
        path_schema.merkle_tree_index_path(1, "0x1234").to_str().unwrap(),
        "/chains/1/contracts/0x1234/merkle_tree/index.json"
    );
    assert_eq!(
        path_schema.merkle_tree_data_path(1, "0x1234", 1000).to_str().unwrap(),
        "/chains/1/contracts/0x1234/merkle_tree/1000.zst"
    );
    assert_eq!(
        path_schema.merkle_tree_latest_data_path(1, "0x1234").to_str().unwrap(),
        "/chains/1/contracts/0x1234/merkle_tree/latest.zst"
    );
    assert_eq!(
        path_schema.granularities_path(1).to_str().unwrap(),
        "/chains/1/granularities"
    );
    assert_eq!(
        path_schema.granularity_path(1, 1000).to_str().unwrap(),
        "/chains/1/granularities/1000"
    );
    assert_eq!(
        path_schema.granularity_index(1, 1000).to_str().unwrap(),
        "/chains/1/granularities/1000/index.json"
    );
    assert_eq!(
        path_schema.blocks_path(1, 1000).to_str().unwrap(),
        "/chains/1/granularities/1000/blocks"
    );
    assert_eq!(
        path_schema.data_path(1, 1000, 100000).to_str().unwrap(),
        "/chains/1/granularities/1000/blocks/100000/data.zst"
    );
    assert_eq!(
        path_schema.checksum_path(1, 1000, 100000).to_str().unwrap(),
        "/chains/1/granularities/1000/blocks/100000/data_checksum.sha512"
    );
}
