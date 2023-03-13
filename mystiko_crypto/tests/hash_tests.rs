extern crate mystiko_crypto;

use mystiko_crypto::hash::keccak256;

#[tokio::test]
async fn test_keccak256() {
    let msg = b"mystiko is ansome";
    let expect = [
        177, 73, 3, 90, 48, 15, 126, 207, 23, 0, 195, 9, 161, 60, 15, 218, 149, 186, 80, 255, 219,
        169, 248, 92, 210, 82, 54, 254, 36, 133, 94, 49,
    ];
    let k1 = keccak256(msg);
    assert_eq!(k1, expect);
}
