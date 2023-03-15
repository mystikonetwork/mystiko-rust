extern crate mystiko_protocol;
extern crate num_bigint;

use num_bigint::BigInt;

use mystiko_protocol::commitment::Note;

#[tokio::test]
async fn test_decrypted_note() {
    let note = Note::new(Some(BigInt::from(10u32)),None) ;
    let enc_vec = note.to_vec();
    let note_dec = Note::from_vec(enc_vec);
    assert_eq!(note, note_dec);
}
