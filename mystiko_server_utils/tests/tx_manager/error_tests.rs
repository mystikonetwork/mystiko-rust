use mystiko_server_utils::tx_manager::error::TxManagerError;

#[tokio::test]
async fn test_error() {
    let err = TxManagerError::GasPriceError("test".to_string());
    let err_str = format!("{:?}", err);
    assert_eq!(err_str, "GasPriceError(\"test\")");
}
