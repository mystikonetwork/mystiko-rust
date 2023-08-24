use mystiko_datapacker_common::{CheckSum, Sha512CheckSum};

#[test]
fn test_sha512_checksum() {
    let bytes = "Hello Mystiko!\n".as_bytes();
    let checksum = Sha512CheckSum;
    assert_eq!(
        checksum.checksum(bytes),
        "397f864a875d3f2c051a75390d89385f846bfc1dacc57bbff875dd6118e9e0c6\
    bd1dc581af5ba04f123ffdcac56f18b37e5916dd6d0f0ec4e8ad3532fbabef34"
    );
}
