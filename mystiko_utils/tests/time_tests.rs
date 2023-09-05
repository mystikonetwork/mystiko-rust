use mystiko_utils::time::current_timestamp;

#[test]
fn test_current_timestamp() {
    assert!(current_timestamp() > 0);
}
