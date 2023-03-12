extern crate mystiko_protocol;
extern crate num_bigint;

use num_bigint::BigInt;

use mystiko_protocol::types::{DecryptedNote, RandomSecrets};

#[tokio::test]
async fn test_decrypted_note() {
    let r = RandomSecrets::generate();
    let amount = BigInt::from(10u32);

    let note = DecryptedNote {
        random_p: r.random_p,
        random_r: r.random_r,
        random_s: r.random_s,
        amount,
    };

    let enc_v = note.to_vec();
    let note_s = DecryptedNote::from_vec(enc_v);
    assert_eq!(note, note_s);
}