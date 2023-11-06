use mystiko_protos::core::v1::{PackerChecksum, PackerCompression};

#[test]
fn test_packer_checksum_to_proto() {
    let type1: PackerChecksum = Into::into(&mystiko_types::PackerChecksum::Sha512);
    assert_eq!(type1, PackerChecksum::Sha512);
}

#[test]
fn test_packer_compression_to_proto() {
    let type1: PackerCompression = Into::into(&mystiko_types::PackerCompression::Zstd);
    assert_eq!(type1, PackerCompression::Zstd);
}
