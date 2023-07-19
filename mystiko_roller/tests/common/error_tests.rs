use mystiko_roller::common::error::RollerError;

#[tokio::test]
async fn test_error() {
    let err = RollerError::NoIndexer;
    let err_str = format!("{:?}", err);
    assert_eq!(err_str, "NoIndexer");
}
