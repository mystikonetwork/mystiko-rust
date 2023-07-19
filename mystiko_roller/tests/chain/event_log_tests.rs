use ethers_core::types::{Log, H256};
use mystiko_roller::chain::event_log::parse_event_logs;
use mystiko_roller::common::error::RollerError;

#[tokio::test]
pub async fn test_parse_event_logs() {
    let result = parse_event_logs::<Log>(1, "0x0", vec![]);
    assert!(result.is_ok());

    let mut log = Log::default();
    let result = parse_event_logs::<Log>(1, "0x0", vec![log.clone()]);
    assert!(matches!(result.err().unwrap(), RollerError::InvalidEventLogs(_)));

    log.block_number = Some(1.into());
    let result = parse_event_logs::<Log>(1, "0x0", vec![log.clone()]);
    assert!(matches!(result.err().unwrap(), RollerError::InvalidEventLogs(_)));

    log.transaction_hash = Some(
        "0x7be34aae6cbb98ef62eea2caaccc75e35efe0b3ebc58269d2dfd1f42cfe363b9"
            .parse()
            .unwrap(),
    );
    let result = parse_event_logs::<Log>(1, "0x0", vec![log.clone()]);
    assert!(matches!(result.err().unwrap(), RollerError::AbiError(_)));

    log.topics = vec![
        "0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9"
            .parse::<H256>()
            .unwrap(),
        "0x1910433764d42a31380011ba6b129d61db48c3f48a281ae277f8d5e6e8e92210"
            .parse::<H256>()
            .unwrap(),
    ];
    log.data="0x000000000000000000000000000000000000000000000000002386f26fc100000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000d15c6a50d7f116ae1dc4b1f865e6e7b194048c7cf2151da8a2a910e7ad6d9ba8c31b9e153d49aead0343c10e9ec025af17b6470e648704e54227e88593fe08a9dbd6f5b2c3d655114c44b53bf4c404bbae532456dce2de0b9574122f9984e2b36fe9f3de867aa2d2ecbe11d95338eeb8d674654d09df41be2f5b06b2fd462352542e9c8ac56e5907cc325cb75a24164523899629c2c1a45a655152bd73f91b8b20a91a4b484283efc3cf02a780f1d4e48d125e6b6303e7bbdb253eaf59a8ec6499a4e3898f525b0f842cffd3dad592bb7d71000000000000000000000000000000"
            .parse()
            .unwrap();
    let result = parse_event_logs::<Log>(1, "0x0", vec![log]);
    assert!(result.is_ok());
}
